use std::str::FromStr;

use freya::prelude::*;
use freya_radio::prelude::*;
use http::Uri;

use crate::app::{Data, DataChannel};

#[derive(Clone, PartialEq)]
pub struct DropdownItem {
    pub text: String,
    pub on_press: Option<EventHandler<Event<PressEventData>>>,
}

impl DropdownItem {
    pub fn new() -> Self {
        Self {
            text: "Item".to_string(),
            on_press: None,
        }
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn _on_press(mut self, on_press: impl FnMut(Event<PressEventData>) + 'static) -> Self {
        self.on_press = Some(EventHandler::new(on_press));
        self
    }
}

impl Render for DropdownItem {
    fn render(&self) -> Element {
        let mut hovering = use_state(|| false);

        use_drop(move || {
            if hovering() {
                Cursor::set(CursorIcon::default());
            }
        });

        let background = if hovering() { "#333333" } else { "#222222" };

        rect()
            .width(Size::px(250.0))
            .height(Size::px(48.0))
            .padding(8.0)
            .background(Color::from_hex(background).unwrap())
            .direction(Direction::Horizontal)
            .main_align(Alignment::SpaceBetween)
            .cross_align(Alignment::Center)
            .on_press({
                let on_press = self.on_press.clone();
                move |e| {
                    if let Some(on_press) = &on_press {
                        on_press.call(e)
                    } else {
                        e.stop_propagation();
                    }
                }
            })
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
            .children([label()
                .font_size(12.0)
                .font_weight(FontWeight::BOLD)
                .color(Color::from_hex("#E4DAD1").unwrap())
                .text(self.text.clone())
                .into()])
            .into()
    }
}

#[derive(Clone, PartialEq)]
pub struct Dropdown {}

impl Dropdown {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for Dropdown {
    fn render(&self) -> Element {
        let connection_state_binding =
            use_radio::<Data, DataChannel>(DataChannel::ConnectionStateUpdate);
        let connection_state = connection_state_binding.read().connection_state.clone();
        let info_state_binding = use_radio::<Data, DataChannel>(DataChannel::InfoStateUpdate);
        let info_state = info_state_binding.read().info_state.clone();

        let mut hovering = use_state(|| false);
        let mut hovering2 = use_state(|| false);

        let image_uri: String = if let Some(info) = info_state.clone() {
            if let Some(logo_image) = info.logo_image.clone() {
                logo_image
            } else {
                String::new()
            }
        } else {
            String::new()
        };

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
                                if !image_uri.is_empty() {
                                    ImageViewer::new(Uri::from_str(image_uri.as_str()).expect(""))
                                        .into()
                                } else {
                                    rect().into()
                                },
                                label()
                                    .font_size(12.0)
                                    .font_weight(FontWeight::BOLD)
                                    .color(Color::from_hex("#E4DAD1").unwrap())
                                    .font_size(12.0)
                                    .text(if let Some(state) = info_state {
                                        format!("{}", state.name)
                                    } else {
                                        "Loading...".to_string()
                                    })
                                    .into(),
                            ])
                            .into(),
                        ImageViewer::new(CHEVRON_DOWN)
                            .width(Size::px(16.0))
                            .height(Size::px(16.0))
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
                            DropdownItem::new().text("Item 1".to_string()).into(),
                            DropdownItem::new().text("Item 2".to_string()).into(),
                            DropdownItem::new().text("Item 3".to_string()).into(),
                        ])
                        .into()
                } else {
                    rect().into()
                },
                label()
                    .font_size(12.0)
                    .font_weight(FontWeight::BOLD)
                    .color(Color::from_hex("#E4DAD1").unwrap())
                    .text(connection_state)
                    .into(),
            ])
            .into()
    }
}

static CHEVRON_DOWN: (&'static str, &'static [u8]) = (
    "chevron-down",
    include_bytes!("./../assets/lucide/chevron-down.png"),
);
