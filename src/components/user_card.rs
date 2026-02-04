use freya::prelude::*;

use crate::TEXT_COLOR;

#[derive(PartialEq)]
pub struct UserCard {}

impl UserCard {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for UserCard {
    fn render(&self) -> impl IntoElement {
        rect()
            .direction(Direction::Horizontal)
            .padding(8.0)
            .spacing(8.0)
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            .children([
                rect()
                    .main_align(Alignment::Center)
                    .children([
                        label()
                            .font_size(12.0)
                            .color(Color::from_hex(TEXT_COLOR).unwrap())
                            .text("Drigster")
                            .into(),
                        label()
                            .font_size(10.0)
                            .color(Color::from_hex("#8D8D8D").unwrap())
                            .text("76561198157374883")
                            .into(),
                    ])
                    .into(),
                rect()
                    .corner_radius(CornerRadius::new_all(1000.0))
                    .height(Size::px(32.0))
                    .width(Size::px(32.0))
                    .overflow(Overflow::Clip)
                    .children([ImageViewer::new(PROFILE_ICON).into()])
                    .into(),
            ])
    }
}

static PROFILE_ICON: (&'static str, &'static [u8]) =
    ("Drigster", include_bytes!("../assets/Drigster.png"));
