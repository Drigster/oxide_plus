use freya::prelude::*;

use crate::components::Dropdown;

#[derive(Clone, PartialEq)]
pub struct Navbar {}

impl Navbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for Navbar {
    fn render(&self) -> Element {
        rect()
            .width(Size::percent(100.0))
            .height(Size::px(48.0))
            .background_linear_gradient(
                LinearGradient::new()
                    .angle(0.0)
                    .stop((Color::from_hex("#171715").unwrap(), 0.0))
                    .stop((Color::from_hex("#11110F").unwrap(), 100.0)),
            )
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
                    .fill(Color::from_hex("#393834").unwrap()),
            )
            .children([
                Dropdown::new().into(),
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
                                    .color(Color::from_hex("#E4DAD1").unwrap())
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
                            .overflow_mode(OverflowMode::Clip)
                            .children([ImageViewer::new(PROFILE_ICON).into()])
                            .into(),
                    ])
                    .into(),
            ])
            .into()
    }
}

static PROFILE_ICON: (&'static str, &'static [u8]) =
    ("Drigster", include_bytes!("../assets/Drigster.png"));
