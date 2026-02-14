use freya::{prelude::*, router::Outlet};

use crate::{app::Route, colors};

#[derive(PartialEq)]
pub struct LoginLayout;
impl Component for LoginLayout {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .cross_align(Alignment::Center)
            .background(Color::from_hex(colors::BACKGROUND_DARK).unwrap())
            .content(Content::Flex)
            .children([
                rect()
                    .height(Size::flex(1.0))
                    .padding(16.0)
                    .child(
                        label()
                            .color(Color::from_hex(colors::TEXT).unwrap())
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
                        Outlet::<Route>::new().into(),
                    ])
                    .into(),
                rect()
                    .height(Size::flex(1.0))
                    .padding(16.0)
                    .main_align(Alignment::End)
                    .cross_align(Alignment::Center)
                    .children([
                        label()
                            .width(Size::Fill)
                            .color(Color::from_hex("#772111").unwrap())
                            .font_size(16.0)
                            .font_weight(FontWeight::BLACK)
                            .text_align(TextAlign::Center)
                            .text("!!! DISCLAIMER !!!")
                            .into(),
                        label()
                            .width(Size::Fill)
                            .color(Color::from_hex("#605B55").unwrap())
                            .font_size(14.0)
                            .font_weight(FontWeight::BOLD)
                            .text_align(TextAlign::Center)
                            .text("This is a community app. It is not affiliated with Facepunch Studios or the game Rust.\nDeveloper is not responsible for any action on your account resulting from the use of this app.")
                            .into(),
                    ])
                    .into(),
            ])
    }
}
