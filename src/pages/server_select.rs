use std::vec;

use freya::prelude::*;
use freya_radio::hooks::use_radio;
use freya_router::prelude::RouterContext;

use crate::{
    app::{Data, DataChannel, Route},
    components::ServerCard,
    utils::settings::{ServerData, save_servers},
};

#[derive(PartialEq)]
pub struct ServerSelect {}
impl Render for ServerSelect {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio::<Data, DataChannel>(DataChannel::ServersUpdate);
        let servers = radio.read().servers.clone();

        rect()
            .children(
                servers
                    .iter()
                    .map(|server| {
                        ServerCard::new(PROFILE_ICON, server.name.clone())
                            .on_press(move |_| {
                                RouterContext::get().replace(Route::Info);
                            })
                            .into()
                    })
                    .collect::<Vec<Element>>(),
            )
            .child(
                ServerCard::new(PROFILE_ICON, "Pair new server...".to_string()).on_press(
                    move |_| {
                        save_servers(vec![ServerData {
                            name: format!("{} {}", "New server name", servers.len()).to_string(),
                            address: "127.0.0.1:28082".to_string(),
                        }])
                        .unwrap();
                    },
                ),
            )
    }
}

static PROFILE_ICON: (&'static str, &'static [u8]) =
    ("Drigster", include_bytes!("../assets/Drigster.png"));
