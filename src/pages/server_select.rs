use std::vec;

use freya::prelude::*;
use freya_radio::hooks::use_radio;
use freya_router::prelude::RouterContext;

use crate::{
    app::{Data, DataChannel, Route},
    components::{ServerCard, UserCard},
    utils::settings::{ServerData, save_servers},
};

#[derive(PartialEq)]
pub struct ServerSelect {}
impl Render for ServerSelect {
    fn render(&self) -> Element {
        let radio = use_radio::<Data, DataChannel>(DataChannel::ServersUpdate);
        let servers = radio.read().servers.clone();

        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .cross_align(Alignment::Center)
            .background_linear_gradient(
                LinearGradient::new()
                    .angle(0.0)
                    .stop((Color::from_hex("#1D1D1B").unwrap(), 0.0))
                    .stop((Color::from_hex("#0E0E0D").unwrap(), 100.0)),
            )
            .content(Content::Flex)
            .children([
                rect()
                    .height(Size::flex(1.0))
                    .padding(16.0)
                    .child(
                        label()
                            .color(Color::from_hex("#E4DAD1").unwrap())
                            .font_size(80.0)
                            .font_weight(FontWeight::BOLD)
                            .text("Oxide+"),
                    )
                    .into(),
                rect()
                    .height(Size::flex(1.0))
                    .padding(16.0)
                    .main_align(Alignment::Center)
                    .spacing(16.0)
                    .children([
                        label()
                            .color(Color::from_hex("#E4DAD1").unwrap())
                            .font_size(20.0)
                            .font_weight(FontWeight::BOLD)
                            .text("Select or pair a server:").into(),
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
                                ServerCard::new(PROFILE_ICON, "Pair new server...".to_string())
                                    .on_press(move |_| {
                                        save_servers(vec![ServerData {
                                            name: format!("{} {}", "New server name", servers.len()) .to_string(),
                                            address: "127.0.0.1:28082".to_string(),
                                        }])
                                        .unwrap();
                                    })
                            )
                            .into()
                    ])
                    .into(),
                rect()
                    .height(Size::flex(1.0))
                    .padding(16.0)
                    .main_align(Alignment::End)
                    .cross_align(Alignment::Center)
                    .children([
                        label()
                            .color(Color::from_hex("#772111").unwrap())
                            .font_size(16.0)
                            .font_weight(FontWeight::BLACK)
                            .text_align(TextAlign::Center)
                            .text("!!! DISCLAIMER !!!")
                            .into(),
                        label()
                            .color(Color::from_hex("#605B55").unwrap())
                            .font_size(14.0)
                            .font_weight(FontWeight::BOLD)
                            .text_align(TextAlign::Center)
                            .text("This is a community app. It is not affiliated with Facepunch Studios or the game Rust. Developer is not responsible for any action on your account resulting from the use of this app.")
                            .into(),
                    ])
                    .into(),
                rect()
                    .width(Size::Fill)
                    .height(Size::Fill)
                    .padding(16.0)
                    .cross_align(Alignment::End)
                    .position(Position::new_absolute())
                    .child(
                        rect().cross_align(Alignment::Center).children([
                            label()
                                .color(Color::from_hex("#E4DAD1").unwrap())
                                .font_size(13.0)
                                .text("Logged in as")
                                .into(),
                            UserCard::new().into(),
                        ]),
                    )
                    .into(),
            ])
            .into()
    }
}

static PROFILE_ICON: (&'static str, &'static [u8]) =
    ("Drigster", include_bytes!("../assets/Drigster.png"));
