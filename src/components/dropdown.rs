use freya::{
    animation::{AnimNum, Ease, use_animation},
    prelude::*,
    radio::*,
};

use crate::{Data, DataChannel, TEXT_COLOR, components::CachedImage};

#[derive(Clone, PartialEq)]
pub struct Dropdown {
    child: Option<Element>,

    title: String,
    icon: Option<String>,

    width: Size,
    height: Size,
    padding: Gaps,
    spacing: f32,
    background: Color,
    background_hover: Color,
    background_chevron: Color,
    border: Border,
    font_size: f32,
}

#[allow(dead_code)]
impl Dropdown {
    pub fn new() -> Self {
        Self {
            child: None,

            title: "".to_string(),
            icon: None,

            width: Size::px(250.0),
            height: Size::Fill,
            padding: Gaps::new_all(8.0),
            spacing: 8.0,
            background: Color::from_hex("#222222").unwrap(),
            background_hover: Color::from_hex("#333333").unwrap(),
            background_chevron: Color::default(),
            border: Border::default(),
            font_size: 12.0,
        }
    }

    pub fn child(mut self, child: Element) -> Self {
        self.child = Some(child);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title.into();
        self
    }

    pub fn icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn width(mut self, width: Size) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Size) -> Self {
        self.height = height;
        self
    }

    pub fn padding(mut self, padding: Gaps) -> Self {
        self.padding = padding;
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    pub fn background_hover(mut self, background_hover: Color) -> Self {
        self.background_hover = background_hover;
        self
    }

    pub fn background_chevron(mut self, background_chevron: Color) -> Self {
        self.background_chevron = background_chevron;
        self
    }

    pub fn border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }
}

impl Component for Dropdown {
    fn render(&self) -> impl IntoElement {
        let mut hovering = use_state(|| false);
        let mut hovering2 = use_state(|| false);
        let mut size = use_state(|| 0.0);

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

        let background = if hovering() {
            self.background_hover
        } else {
            self.background
        };

        rect()
            .on_sized(move |e: Event<SizedEventData>| {
                size.set(e.area.height());
            })
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
                    .width(self.width.clone())
                    .height(self.height.clone())
                    .border(self.border.clone())
                    .background(background)
                    .direction(Direction::Horizontal)
                    .main_align(Alignment::SpaceBetween)
                    .cross_align(Alignment::Center)
                    .children([
                        rect()
                            .direction(Direction::Horizontal)
                            .cross_align(Alignment::Center)
                            .padding(self.padding)
                            .spacing(self.spacing)
                            .children([
                                if let Some(icon) = &self.icon {
                                    CachedImage::new(icon.clone()).into()
                                } else {
                                    rect().into()
                                },
                                label()
                                    .font_weight(FontWeight::BOLD)
                                    .color(Color::from_hex(TEXT_COLOR).unwrap())
                                    .font_size(self.font_size)
                                    .text(self.title.clone())
                                    .into(),
                            ])
                            .into(),
                        rect()
                            .width(Size::px(*size.read()))
                            .height(Size::px(*size.read()))
                            .background(self.background_chevron)
                            .center()
                            .child(
                                svg(freya_icons::lucide::chevron_up())
                                    .rotate(rotation)
                                    .height(Size::Fill)
                                    .color(Color::from_hex(TEXT_COLOR).unwrap()),
                            )
                            .into(),
                    ])
                    .into(),
                rect()
                    .width(self.width.clone())
                    .layer(100)
                    .position(Position::new_absolute().top(*size.read()))
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
                    .maybe_child(self.child.clone())
                    .into(),
            ])
    }
}
