use freya::prelude::*;

use crate::components::{Dropdown, DropdownOption, UserCard};

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
                Dropdown::new(vec![
                    DropdownOption {
                        icon: Some(Bytes::from_static(PROFILE_ICON)),
                        text: "Rusty Moose | EU Hapis".to_string(),
                        on_press: None,
                    },
                    DropdownOption {
                        icon: Some(Bytes::from_static(PROFILE_ICON)),
                        text: "Rusty Moose | EU Hapis".to_string(),
                        on_press: None,
                    },
                    DropdownOption {
                        icon: Some(Bytes::from_static(PROFILE_ICON)),
                        text: "Rusty Moose | EU Hapis".to_string(),
                        on_press: None,
                    },
                ])
                .width(Size::px(250.0))
                .height(Size::Fill)
                .child_height(Size::px(48.0))
                .into(),
                UserCard::new().into(),
            ])
            .into()
    }
}

static PROFILE_ICON: &[u8] = include_bytes!("../assets/Drigster.png");
