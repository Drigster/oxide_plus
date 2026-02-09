use freya::{
    animation::{AnimNum, Ease, use_animation},
    prelude::*,
    radio::*,
};

use crate::{Data, DataChannel, TEXT_COLOR, components::CachedImage};

#[derive(Clone, PartialEq)]
pub struct Dropdown {
    elements: Vec<Element>,

    title: String,
    icon: Option<String>,
}

impl Dropdown {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),

            title: "".to_string(),
            icon: None,
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title.into();
        self
    }

    pub fn icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }
}

impl ChildrenExt for Dropdown {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl Component for Dropdown {
    fn render(&self) -> impl IntoElement {
        let mut hovering = use_state(|| false);
        let mut hovering2 = use_state(|| false);

        use_drop(move || {
            if hovering() || hovering2() {
                Cursor::set(CursorIcon::default());
            }
        });

        let mut animation = use_animation(|_| {
            (
                AnimNum::new(0., 100.).ease(Ease::InOut).time(200),
                AnimNum::new(0., -180.).ease(Ease::InOut).time(200),
            )
        });

        use_side_effect(move || {
            if hovering() || hovering2() {
                if animation.peek().0.value() != 100.0 {
                    animation.start();
                }
            } else if animation.peek().0.value() != 0.0 {
                animation.reverse();
            }
        });

        let height = animation.read().0.value();
        let rotation = animation.read().1.value();

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
                                if let Some(icon) = &self.icon {
                                    CachedImage::new(icon.clone()).into()
                                } else {
                                    rect().into()
                                },
                                label()
                                    .font_size(12.0)
                                    .font_weight(FontWeight::BOLD)
                                    .color(Color::from_hex(TEXT_COLOR).unwrap())
                                    .font_size(12.0)
                                    .text(self.title.clone())
                                    .into(),
                            ])
                            .into(),
                        svg(freya_icons::lucide::chevron_up())
                            .rotate(rotation)
                            .height(Size::Fill)
                            .color(Color::from_hex(TEXT_COLOR).unwrap())
                            .into(),
                    ])
                    .into(),
                rect()
                    .width(Size::px(250.0))
                    .layer(100)
                    .position(Position::new_absolute().top(47.0))
                    .background(Color::from_hex("#222222").unwrap())
                    .overflow(Overflow::Clip)
                    .visible_height(VisibleSize::inner_percent(height))
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
                    .children(self.elements.clone())
                    .into(),
            ])
    }
}
