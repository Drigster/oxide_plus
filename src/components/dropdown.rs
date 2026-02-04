use freya::{prelude::*, radio::*};

use crate::{Data, DataChannel, TEXT_COLOR, components::CachedImage};

#[derive(Clone, PartialEq)]
pub struct Dropdown {}

impl Dropdown {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Dropdown {
    fn render(&self) -> impl IntoElement {
        let info_state_binding = use_radio::<Data, DataChannel>(DataChannel::InfoStateUpdate);
        let info_state = info_state_binding.read().info_state.clone();

        let mut hovering = use_state(|| false);
        let mut hovering2 = use_state(|| false);

        use_drop(move || {
            if hovering() || hovering2() {
                Cursor::set(CursorIcon::default());
            }
        });

        let background = if hovering() { "#333333" } else { "#222222" };

        rect()
            .direction(Direction::Horizontal)
            .on_pointer_enter(move |_| {
                Cursor::set(CursorIcon::Pointer);
                hovering.set(true);
            })
            .on_pointer_leave(move |_| {
                if hovering() {
                    Cursor::set(CursorIcon::default());
                    hovering.set(false);
                }
            })
            .children([
                rect()
                    .width(Size::px(250.0))
                    .height(Size::Fill)
                    .padding(8.0)
                    .background(Color::from_hex(background).unwrap())
                    .direction(Direction::Horizontal)
                    .main_align(Alignment::SpaceBetween)
                    .cross_align(Alignment::Center)
                    .children([
                        rect()
                            .direction(Direction::Horizontal)
                            .cross_align(Alignment::Center)
                            .spacing(8.0)
                            .children([
                                if let Some(info_state) = &info_state {
                                    CachedImage::new(info_state.logo_image.clone()).into()
                                } else {
                                    rect().into()
                                },
                                label()
                                    .font_size(12.0)
                                    .font_weight(FontWeight::BOLD)
                                    .color(Color::from_hex(TEXT_COLOR).unwrap())
                                    .font_size(12.0)
                                    .text(if let Some(info_state) = &info_state {
                                        info_state.name.clone()
                                    } else {
                                        "Retrieving...".to_string()
                                    })
                                    .into(),
                            ])
                            .into(),
                        svg(freya_icons::lucide::chevron_up())
                            .height(Size::Fill)
                            .color(Color::from_hex(TEXT_COLOR).unwrap())
                            .into(),
                    ])
                    .into(),
                if hovering() || hovering2() {
                    rect()
                        .width(Size::px(250.0))
                        .layer(100)
                        .position(Position::new_absolute().top(47.0))
                        .background(Color::from_hex("#222222").unwrap())
                        .on_press(move |_| {
                            if hovering2() {
                                Cursor::set(CursorIcon::default());
                                hovering2.set(false);
                            }
                        })
                        .on_pointer_enter(move |_| {
                            Cursor::set(CursorIcon::Pointer);
                            hovering2.set(true);
                        })
                        .on_pointer_leave(move |_| {
                            if hovering2() {
                                Cursor::set(CursorIcon::default());
                                hovering2.set(false);
                            }
                        })
                        .children([
                            // ServerCard::new(PROFILE_ICON, "Rusty Moose | EU Hapis".to_string())
                            //     .into(),
                            // ServerCard::new(PROFILE_ICON, "Rusty Moose | EU Hapis".to_string())
                            //     .into(),
                            // ServerCard::new(PROFILE_ICON, "Rusty Moose | EU Hapis".to_string())
                            //     .into(),
                        ])
                        .into()
                } else {
                    rect().into()
                },
                // label()
                //     .font_size(12.0)
                //     .font_weight(FontWeight::BOLD)
                //     .color(Color::from_hex(TEXT_COLOR).unwrap())
                //     .text(connection_state)
                //     .into(),
            ])
    }
}
