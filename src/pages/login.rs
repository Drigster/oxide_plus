use freya::{prelude::*, radio::*, webview::prelude::WebView};

use freya_router::prelude::RouterContext;
use serde::{Deserialize, Serialize};

use crate::{Data, DataChannel, app::Route, utils::save_user_data};
use futures_lite::stream::StreamExt;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserData {
    #[serde(rename = "SteamId")]
    pub steam_id: String,
    #[serde(rename = "Token")]
    pub token: String,
}

#[derive(PartialEq)]
pub struct Login {}
impl Component for Login {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio::<Data, DataChannel>(DataChannel::UserDataUpdate);

        let (data_tx, mut data_rx) = futures_channel::mpsc::unbounded::<UserData>();

        use_hook(|| {
            spawn(async move {
                while let Some(data) = data_rx.next().await {
                    radio.write_channel(DataChannel::UserDataUpdate).user_data = Some(data.clone());
                    match save_user_data(data) {
                        Ok(_) => println!("User data saved successfully."),
                        Err(e) => println!("Failed to save user data: {:?}", e),
                    }
                    RouterContext::get().replace(Route::Loading);
                }
            });
        });

        rect()
            .expanded()
            .height(Size::fill())
            .background((35, 35, 35))
            .child("Login")
            .child(
                Button::new()
                    .child("Test"),
            )
            .child(
                WebView::new("https://companion-rust.facepunch.com/login")
                    .expanded()
                    .close_on_drop(false)
                    .on_created(move |builder| {
                        builder
                        .with_initialization_script(
                            r#"(function() {
                                'use strict';

                                const JSONbig = (function() {
                                    const JSONbigInt = {
                                        parse: function(text) {
                                            return JSON.parse(text, function(key, value) {
                                                if (typeof value === 'string' && /^\d{15,}$/.test(value)) {
                                                    return value;
                                                }
                                                if (typeof value === 'number' && !Number.isSafeInteger(value)) {
                                                    return String(value);
                                                }
                                                return value;
                                            });
                                        }
                                    };
                                    return JSONbigInt;
                                })();

                                window.ReactNativeWebView = {
                                    postMessage: function(message) {
                                        try {
                                            const auth = JSONbig.parse(message);

                                            window.ipc.postMessage(JSON.stringify(auth));
                                        } catch (error) {
                                            console.error('Error:', error);
                                            document.body.innerHTML += `
                                                <div style="background: #cc0000; color: black; padding: 20px; margin: 20px;">
                                                    <h2>✓ Authentication Error!</h2>
                                                    ${error.message}
                                                </div>
                                            `;
                                        }
                                    }
                                };
                            })();"#,
                        )
                        .with_devtools(true)
                        .with_ipc_handler({
                            let data_tx = data_tx.clone();
                            move |request| {
                            match serde_json::from_str::<UserData>(&request.body()) {
                                Ok(value) => {
                                    let _ = data_tx.unbounded_send(value);
                                }
                                Err(error) => {
                                    println!("{:?}", error);
                                }
                            }
                        }})
                    })
                    .into_element()
            )
    }
}
