use freya::{prelude::*, radio::*};
use freya_router::prelude::RouterContext;

use crate::{Data, DataChannel, TEXT_COLOR, app::Route, components::ServerCard};

#[derive(PartialEq)]
pub struct ServerSelect {}
impl Component for ServerSelect {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio::<Data, DataChannel>(DataChannel::ServersUpdate);
        let state_tx = radio.read().state_tx.clone().unwrap();

        rect()
            .children(
                radio
                    .read()
                    .servers
                    .clone()
                    .into_iter()
                    .filter_map(|server| {
                        Some(
                            ServerCard::new(server.logo.clone(), server.name.clone())
                                .on_press({
                                    let state_tx = state_tx.clone();
                                    move |_| {
                                        state_tx
                                            .unbounded_send(
                                                crate::ChannelSend::SelectedServerUpdate(Some(
                                                    server.clone(),
                                                )),
                                            )
                                            .unwrap();
                                        RouterContext::get().replace(Route::Info);
                                    }
                                })
                                .into(),
                        )
                    })
                    .collect::<Vec<Element>>(),
            )
            .maybe_child({
                if radio.read().servers.is_empty() {
                    Some(
                        label()
                            .font_size(20.0)
                            .font_weight(FontWeight::BOLD)
                            .color(Color::from_hex(TEXT_COLOR).unwrap())
                            .text("No servers found"),
                    )
                } else {
                    None
                }
            })
    }
}
