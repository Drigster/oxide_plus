use std::{collections::HashSet, sync::Arc};

use freya::{prelude::*, radio::*};
use freya_router::prelude::RouterContext;
use serde::Deserialize;

use crate::{
    ChannelSend, Data, DataChannel,
    app::Route,
    utils::{
        FcmData, ServerData, create_fcm_client, get_expo_push_token, load_expo_push_token,
        load_fcm_data, load_last_persistent_id, load_servers, load_user_data,
        register_with_rust_plus, save_expo_push_token, save_fcm_data, save_last_persistent_id,
        save_server,
    },
};

#[derive(PartialEq)]
pub struct Loading {}
impl Component for Loading {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio::<Data, DataChannel>(DataChannel::LoadingStateUpdate);

        use_future(move || {
            let state_tx = radio.read().state_tx.clone().unwrap();
            async move {
                state_tx
                    .unbounded_send(ChannelSend::LoadingStateUpdate(
                        "Loading user data...".to_string(),
                    ))
                    .unwrap();

                let user_data_result = load_user_data();

                let mut user_data = match user_data_result {
                    Ok(user_data) => match user_data {
                        Some(user_data) => {
                            state_tx
                                .unbounded_send(ChannelSend::UserDataUpdate(user_data.clone()))
                                .unwrap();

                            user_data
                        }
                        None => {
                            RouterContext::get().replace(Route::Login);
                            return;
                        }
                    },
                    Err(err) => {
                        println!("Error loading user data. {:?}", err);
                        RouterContext::get().replace(Route::Login);
                        return;
                    }
                };

                state_tx
                    .unbounded_send(ChannelSend::LoadingStateUpdate(
                        "Creating FCM client...".to_string(),
                    ))
                    .unwrap();
                let mut fcm_client = match create_fcm_client() {
                    Ok(client) => client,
                    Err(err) => {
                        panic!("Error creating FCM client: {}", err);
                    }
                };

                state_tx
                    .unbounded_send(ChannelSend::LoadingStateUpdate(
                        "Loading last persistent ID...".to_string(),
                    ))
                    .unwrap();
                let last_persistent_id = match load_last_persistent_id() {
                    Ok(id) => id,
                    Err(err) => {
                        panic!("Error loading last persistent ID: {}", err);
                    }
                };
                if let Some(id) = last_persistent_id {
                    fcm_client.set_persistent_ids(HashSet::from([id]));
                }

                state_tx
                    .unbounded_send(ChannelSend::LoadingStateUpdate(
                        "Creating FCM message handler...".to_string(),
                    ))
                    .unwrap();
                fcm_client.on_raw_message = Some(Arc::new({
                    let state_tx = state_tx.clone();
                    move |payload| {
                        let mut channel_id: Option<String> = None;
                        let mut message: Option<String> = None;
                        for data in dbg!(&payload).app_data.iter() {
                            match data.key.as_str() {
                                "channelId" => channel_id = Some(data.value.clone()),
                                "body" => message = Some(data.value.clone()),
                                _ => {}
                            }
                        }

                        if let Some(id) = payload.persistent_id {
                            match save_last_persistent_id(&id) {
                                Ok(_) => println!("Last persistent ID saved successfully."),
                                Err(e) => println!("Failed to save last persistent ID: {:?}", e),
                            };
                        }

                        if let (Some(channel_id), Some(message)) = (channel_id, message) {
                            match channel_id.as_str() {
                                "pairing" => {
                                    let server_data = serde_json::from_str::<ServerData>(&message);
                                    match server_data {
                                        Ok(data) => {
                                            state_tx
                                                .unbounded_send(ChannelSend::AddServer(
                                                    data.clone(),
                                                ))
                                                .unwrap();
                                            match save_server(data) {
                                                Ok(_) => {
                                                    println!("Server data saved successfully.");
                                                }
                                                Err(err) => {
                                                    println!("Error saving server data: {}", err);
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("Error parsing server data: {}", err);
                                        }
                                    }
                                }
                                "team" => {
                                    let team_data = serde_json::from_str::<TeamData>(&message);
                                    match team_data {
                                        Ok(data) => {
                                            println!("Received team data: {:?}", data);
                                        }
                                        Err(err) => {
                                            println!("Error parsing team data: {}", err);
                                        }
                                    }
                                }
                                "player" => {
                                    let player_data = serde_json::from_str::<PlayerData>(&message);
                                    match player_data {
                                        Ok(data) => {
                                            println!("Received player data: {:?}", data);
                                        }
                                        Err(err) => {
                                            println!("Error parsing player data: {}", err);
                                        }
                                    }
                                }
                                _ => {
                                    println!("Unknown channel ID: {}", channel_id);
                                }
                            }
                        } else {
                            println!("Received message with incomplete data");
                        }
                    }
                }));

                let mut fcm_data = match load_fcm_data() {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Error loading FCM data: {}", err);
                        None
                    }
                };

                match &fcm_data {
                    Some(data) => {
                        fcm_client.android_id = data.android_id;
                        fcm_client.security_token = data.security_token;
                        match fcm_client.load_keys(&data.private_key, &data.auth_secret) {
                            Ok(_) => {}
                            Err(err) => {
                                println!("Error loading FCM keys: {}", err);
                                return;
                            }
                        };
                    }
                    None => {
                        let (private_key_b64, auth_secret_b64) = match fcm_client.create_new_keys()
                        {
                            Ok(keys) => keys,
                            Err(err) => {
                                println!("Error creating FCM keys: {}", err);
                                return;
                            }
                        };

                        match fcm_client.load_keys(&private_key_b64, &auth_secret_b64) {
                            Ok(_) => println!("FCM keys loaded."),
                            Err(err) => {
                                println!("Error loading FCM keys: {}", err);
                                return;
                            }
                        };
                        state_tx
                            .unbounded_send(ChannelSend::LoadingStateUpdate(
                                "Regestering FCM...".to_string(),
                            ))
                            .unwrap();
                        let (fcm_token, _gcm_token, android_id, security_token) =
                            match fcm_client.register() {
                                Ok(tokens) => tokens,
                                Err(err) => {
                                    panic!("Error registering FCM client: {}", err);
                                }
                            };
                        println!("FCM client registered. FCM Token: {}", fcm_token);

                        fcm_data = Some(FcmData {
                            android_id: android_id,
                            security_token: security_token,
                            private_key: private_key_b64,
                            auth_secret: auth_secret_b64,
                            fcm_token: fcm_token,
                            last_persistent_id: None,
                        });

                        match save_fcm_data(fcm_data.clone().unwrap()) {
                            Ok(_) => println!("FCM data saved successfully."),
                            Err(e) => println!("Failed to save FCM data: {:?}", e),
                        };
                    }
                }

                let fcm_data = match fcm_data {
                    Some(data) => data,
                    None => {
                        panic!("FCM data is None");
                    }
                };

                state_tx
                    .unbounded_send(ChannelSend::LoadingStateUpdate(
                        "Getting expo push token...".to_string(),
                    ))
                    .unwrap();

                let _ = match load_expo_push_token() {
                    Ok(token) => match token {
                        Some(token) => token,
                        None => {
                            println!("No expo push token found, registering with FCM...");
                            let expo_push_token =
                                match get_expo_push_token(fcm_data.fcm_token).await {
                                    Ok(token) => token,
                                    Err(err) => {
                                        panic!("Error getting Expo Push Token: {}", err);
                                    }
                                };
                            println!("Expo Push Token: {}", expo_push_token);
                            match save_expo_push_token(&expo_push_token) {
                                Ok(_) => println!("Expo push token saved successfully."),
                                Err(e) => println!("Failed to save expo push token: {:?}", e),
                            };

                            state_tx
                                .unbounded_send(ChannelSend::LoadingStateUpdate(
                                    "Registering with rust plus...".to_string(),
                                ))
                                .unwrap();
                            match register_with_rust_plus(user_data.token, expo_push_token.clone())
                                .await
                            {
                                Ok(new_token) => {
                                    user_data.token = new_token;
                                }
                                Err(err) => {
                                    panic!("Error registering with Rust Plus: {}", err);
                                }
                            }
                            println!("Registered with Rust Plus. New token: {}", user_data.token);
                            expo_push_token
                        }
                    },
                    Err(err) => panic!("Error getting Expo Push Token: {}", err),
                };

                state_tx
                    .unbounded_send(ChannelSend::LoadingStateUpdate(
                        "Starting FCM listener...".to_string(),
                    ))
                    .unwrap();
                let state_tx_clone = state_tx.clone();
                let task = blocking::unblock(move || match fcm_client.start_listening() {
                    Ok(_) => {
                        state_tx_clone
                            .unbounded_send(ChannelSend::LoadingStateUpdate(
                                "FCM listener started. Waiting for pairing...".to_string(),
                            ))
                            .unwrap();
                    }
                    Err(err) => {
                        panic!("Error starting FCM listening: {}", err);
                    }
                });

                state_tx
                    .unbounded_send(ChannelSend::LoadingStateUpdate(
                        "Loading servers...".to_string(),
                    ))
                    .unwrap();
                let servers = match load_servers() {
                    Ok(servers) => servers
                        .iter()
                        .filter(|e| e.player_id == user_data.steam_id)
                        .cloned()
                        .collect(),
                    Err(err) => {
                        panic!("Error loading servers: {}", err);
                    }
                };

                state_tx
                    .unbounded_send(ChannelSend::ServersUpdate(servers))
                    .unwrap();

                RouterContext::get().replace(Route::ServerSelect);

                let _ = task.await;
            }
        });

        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            .children([label()
                .font_size(20.0)
                .font_weight(FontWeight::BOLD)
                .color(Color::from_hex("#E4DAD1").unwrap())
                .text(radio.read().loading_state.clone())
                .into()])
    }
}

#[derive(Deserialize, Debug)]
struct TeamData {
    r#type: String,
    targetId: String,
    targetName: String,
}

#[derive(Deserialize, Debug)]
struct PlayerData {
    r#type: String,
    targetId: String,
    targetName: String,
}
