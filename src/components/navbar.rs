use freya::{prelude::*, radio::use_radio, router::RouterContext};

use crate::{
    Data, DataChannel,
    app::Route,
    colors,
    components::{Dropdown, ServerCard, UserCard},
};

#[derive(Clone, PartialEq)]
pub struct Navbar {}

impl Navbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Navbar {
    fn render(&self) -> impl IntoElement {
        let info_state_binding = use_radio::<Data, DataChannel>(DataChannel::InfoStateUpdate);
        let info_state = &info_state_binding.read().info_state;
        let servers_binding = use_radio::<Data, DataChannel>(DataChannel::ServersUpdate);
        let servers = servers_binding.read().servers.clone();
        let state_tx_binding = use_radio::<Data, DataChannel>(DataChannel::StateTxUpdate);
        let state_tx = state_tx_binding.read().state_tx.clone().unwrap();

        rect()
            .width(Size::percent(100.0))
            .height(Size::px(48.0))
            .background(Color::from_hex(colors::BACKGROUND).unwrap())
            .direction(Direction::Horizontal)
            .main_align(Alignment::SpaceBetween)
            .cross_align(Alignment::Center)
            .border(
                Border::new()
                    .width(BorderWidth {
                        top: 0.0,
                        right: 0.0,
                        bottom: 1.0,
                        left: 0.0,
                    })
                    .alignment(BorderAlignment::Outer)
                    .fill(Color::from_hex(colors::BORDER).unwrap()),
            )
            .children([
                Dropdown::new()
                    .title(if let Some(name) = &info_state.name {
                        name.clone()
                    } else {
                        "Retrieving...".to_string()
                    })
                    .child(
                        rect()
                            .children(
                                servers
                                    .into_iter()
                                    .map({
                                        let state_tx = state_tx.clone();
                                        move |server| {
                                            ServerCard::new(
                                                server.logo.clone(),
                                                server.name.clone(),
                                            )
                                            .on_press({
                                                let state_tx = state_tx.clone();
                                                move |_| {
                                                    state_tx
                                                    .unbounded_send(
                                                        crate::ChannelSend::SelectedServerUpdate(
                                                            Some(server.clone()),
                                                        ),
                                                    )
                                                    .unwrap();
                                                    RouterContext::get().replace(Route::Info);
                                                }
                                            })
                                            .into()
                                        }
                                    })
                                    .collect::<Vec<Element>>(),
                            )
                            .into(),
                    )
                    .into(),
                UserCard::new().into(),
            ])
    }
}
