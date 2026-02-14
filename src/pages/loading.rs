use std::{collections::HashSet, fs::OpenOptions, io::Write, sync::Arc};

use freya::{prelude::*, radio::*};
use freya_router::prelude::RouterContext;
use serde::{Deserialize, Serialize};

use crate::{
    ChannelSend, Data, DataChannel, ToastData,
    app::Route,
    colors,
    components::{Modal, ModalType, Timeout},
    utils::*,
};

#[derive(PartialEq)]
pub struct Loading {}
impl Component for Loading {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio::<Data, DataChannel>(DataChannel::LoadingStateUpdate);
        let toasts = radio.slice_mut(DataChannel::ToastsUpdate, |s| &mut s.toasts);

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
                        create_toast(
                            toasts.into_writable(),
                            "Error loading user data".to_string(),
                            err.to_string(),
                            Timeout::Default,
                            None::<fn(())>,
                        );
                        eprintln!("Error loading user data. {:?}", err);
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
                                Ok(_) => {}
                                Err(e) => eprintln!("Failed to save last persistent ID: {:?}", e),
                            };
                        }

                        if let (Some(channel_id), Some(message)) = (channel_id, message) {
                            match channel_id.as_str() {
                                "pairing" => {
                                    let server_data = serde_json::from_str::<ServerData>(&message);
                                    match server_data {
                                        Ok(mut data) => {
                                            data.desc = data.desc.replace("\\n", "\n");
                                            state_tx
                                                .unbounded_send(ChannelSend::AddToast(ToastData {
                                                    title: "New server paring request".to_string(),
                                                    message: "Click here to pair".to_string(),
                                                    timeout: Timeout::Infinite,
                                                    on_press: Some(Box::new({
                                                        let state_tx = state_tx.clone();
                                                        move |_| {
                                                            state_tx
                                                                .unbounded_send(
                                                                    ChannelSend::ModalUpdate(Some(
                                                                        Modal::new(
                                                                            ModalType::ServerPair(
                                                                                data.clone(),
                                                                            ),
                                                                        ),
                                                                    )),
                                                                )
                                                                .unwrap();
                                                        }
                                                    })),
                                                }))
                                                .unwrap();
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
                                            let mut file = OpenOptions::new()
                                                .write(true)
                                                .append(true)
                                                .create(true) // Creates file if it doesn't exist
                                                .open("team_notif.json")
                                                .unwrap();

                                            file.write(
                                                serde_json::to_string_pretty(&data)
                                                    .unwrap()
                                                    .as_bytes(),
                                            )
                                            .unwrap();
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
                                            let mut file = OpenOptions::new()
                                                .write(true)
                                                .append(true)
                                                .create(true) // Creates file if it doesn't exist
                                                .open("player_notif.json")
                                                .unwrap();
                                            file.write(
                                                serde_json::to_string_pretty(&data)
                                                    .unwrap()
                                                    .as_bytes(),
                                            )
                                            .unwrap();
                                        }
                                        Err(err) => {
                                            println!("Error parsing player data: {}", err);
                                        }
                                    }
                                }
                                data => {
                                    println!("Unknown channel ID: {}", channel_id);
                                    let mut file = OpenOptions::new()
                                        .write(true)
                                        .append(true)
                                        .create(true) // Creates file if it doesn't exist
                                        .open("unknown_notif.json")
                                        .unwrap();
                                    file.write(
                                        serde_json::to_string_pretty(&data).unwrap().as_bytes(),
                                    )
                                    .unwrap();
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
                            match register_with_rust_plus(
                                user_data.token.expect("Token should be set!"),
                                expo_push_token.clone(),
                            )
                            .await
                            {
                                Ok(new_token) => {
                                    user_data.token = Some(new_token);
                                }
                                Err(err) => {
                                    panic!("Error registering with Rust Plus: {}", err);
                                }
                            }
                            println!(
                                "Registered with Rust Plus. New token: {}",
                                user_data.token.expect("Token should be set!")
                            );
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
                        state_tx_clone
                            .unbounded_send(ChannelSend::AddToast(ToastData {
                                title: "FCM listener failed!".to_owned(),
                                message: "Reconnecting!".to_owned(),
                                timeout: Timeout::Infinite,
                                on_press: None,
                            }))
                            .unwrap();
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
                        .into_iter()
                        .filter_map(|e| {
                            if Some(e.player_id.clone()) == user_data.steam_id {
                                Some((e.id.clone(), e.clone()))
                            } else {
                                None
                            }
                        })
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
                .color(Color::from_hex(colors::TEXT).unwrap())
                .text(radio.read().loading_state.clone())
                .into()])
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
struct TeamData {
    r#type: String,
    #[serde(rename = "targetId")]
    target_id: String,
    #[serde(rename = "targetName")]
    target_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
struct PlayerData {
    r#type: String,
    #[serde(rename = "targetId")]
    target_id: String,
    #[serde(rename = "targetName")]
    target_name: String,
}
