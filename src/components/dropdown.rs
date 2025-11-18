use freya::prelude::*;

use crate::components::Button;

#[derive(PartialEq, Clone)]
pub struct DropdownOption {
    pub icon: Option<Bytes>,
    pub text: String,
    pub on_press: Option<EventHandler<Event<PressEventData>>>,
}

#[derive(PartialEq, Clone)]
pub struct Dropdown {
    width: Size,
    height: Size,
    child_height: Size,
    padding: Gaps,
    background: Color,
    background_hover: Color,
    background_child: Color,
    background_child_hover: Color,
    background_chevron: Color,

    options: Vec<DropdownOption>,
    selected_index: usize,
}

#[allow(dead_code)]
impl Dropdown {
    pub fn new(options: Vec<DropdownOption>) -> Self {
        if options.is_empty() {
            panic!("Dropdown must have at least one option");
        }

        Self {
            width: Size::default(),
            height: Size::default(),
            child_height: Size::px(48.0),
            padding: Gaps::new_all(4.0),
            background: Color::from_hex("#222222").unwrap(),
            background_hover: Color::from_hex("#333333").unwrap(),
            background_child: Color::from_hex("#222222").unwrap(),
            background_child_hover: Color::from_hex("#333333").unwrap(),
            background_chevron: Color::TRANSPARENT,

            options,
            selected_index: 0,
        }
    }

    pub fn width(mut self, width: Size) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Size) -> Self {
        self.height = height;
        self
    }

    pub fn child_height(mut self, child_height: Size) -> Self {
        self.child_height = child_height;
        self
    }

    pub fn padding(mut self, padding: impl Into<Gaps>) -> Self {
        self.padding = padding.into();
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

    pub fn background_child(mut self, background_child: Color) -> Self {
        self.background_child = background_child;
        self
    }

    pub fn background_child_hover(mut self, background_child_hover: Color) -> Self {
        self.background_child_hover = background_child_hover;
        self
    }

    pub fn background_chevron(mut self, background_chevron: Color) -> Self {
        self.background_chevron = background_chevron;
        self
    }
}

impl Render for Dropdown {
    fn render(&self) -> Element {
        let mut expanded = use_state(|| false);
        let mut size = use_state(Area::default);

        rect()
            .on_sized(move |e: Event<SizedEventData>| {
                size.set(e.area);
            })
            .child(
                Button::new()
                    .width(self.width.clone())
                    .height(self.height.clone())
                    .padding(0.0)
                    .background(self.background.clone())
                    .background_hover(self.background_hover.clone())
                    .align(Alignment::SpaceBetween)
                    .on_press(move |_| {
                        expanded.set_if_modified(!expanded());
                    })
                    .children([
                        rect()
                            .direction(Direction::Horizontal)
                            .cross_align(Alignment::Center)
                            .padding(self.padding.clone())
                            .spacing(8.0)
                            .maybe_child(
                                if let Some(icon) = &self.options[self.selected_index].icon {
                                    // TODO: Generate random name
                                    Some(
                                        ImageViewer::new(("random_name", icon.clone()))
                                            .height(Size::Fill),
                                    )
                                } else {
                                    None
                                },
                            )
                            .child(
                                label()
                                    .font_size(12.0)
                                    .font_weight(FontWeight::BOLD)
                                    .color(Color::from_hex("#E4DAD1").unwrap())
                                    .font_size(12.0)
                                    .text(self.options[self.selected_index].text.clone()),
                            )
                            .into(),
                        rect()
                            .padding(8.0)
                            .height(Size::px(size.read().height()))
                            .width(Size::px(size.read().height()))
                            .background(self.background_chevron.clone())
                            .child(
                                svg(if expanded() {
                                    freya_icons::lucide::chevron_up()
                                } else {
                                    freya_icons::lucide::chevron_down()
                                })
                                .width(Size::Fill)
                                .height(Size::Fill)
                                .color(Color::from_hex("#E4DAD1").unwrap()),
                            )
                            .into(),
                    ])
                    .maybe_child(if expanded() {
                        Some(
                            rect()
                                .width(self.width.clone())
                                .layer(100)
                                .position(
                                    Position::new_global()
                                        .top(size.read().origin.y + size.read().height())
                                        .left(size.read().origin.x),
                                )
                                .on_press(move |_| {
                                    if expanded() {
                                        Cursor::set(CursorIcon::default());
                                        expanded.set(false);
                                    }
                                })
                                .children_iter({
                                    let options = self.options.clone();
                                    options.into_iter().enumerate().filter_map(
                                        move |(i, option)| {
                                            if i == self.selected_index {
                                                None
                                            } else {
                                                Some(
                                                    Button::new()
                                                        .width(Size::Fill)
                                                        .height(self.child_height.clone())
                                                        .padding(self.padding)
                                                        .background(self.background_child.clone())
                                                        .background_hover(
                                                            self.background_child_hover.clone(),
                                                        )
                                                        .border(
                                                            Border::new()
                                                                .width(BorderWidth {
                                                                    top: 1.0,
                                                                    right: 0.0,
                                                                    bottom: 0.0,
                                                                    left: 0.0,
                                                                })
                                                                .alignment(BorderAlignment::Inner)
                                                                .fill(
                                                                    Color::from_hex("#393834")
                                                                        .unwrap(),
                                                                ),
                                                        )
                                                        .on_press(
                                                            move |e: Event<PressEventData>| {
                                                                if let Some(on_press) =
                                                                    &option.on_press
                                                                {
                                                                    on_press.call(e);
                                                                }
                                                            },
                                                        )
                                                        .maybe_child(
                                                            if let Some(image) = &option.icon {
                                                                // TODO: Generate random name
                                                                Some(
                                                                    ImageViewer::new((
                                                                        "random_name",
                                                                        image.clone(),
                                                                    ))
                                                                    .height(Size::Fill),
                                                                )
                                                            } else {
                                                                None
                                                            },
                                                        )
                                                        .child(
                                                            label()
                                                                .font_size(12.0)
                                                                .font_weight(FontWeight::BOLD)
                                                                .color(
                                                                    Color::from_hex("#E4DAD1")
                                                                        .unwrap(),
                                                                )
                                                                .font_size(12.0)
                                                                .text(option.text),
                                                        )
                                                        .into(),
                                                )
                                            }
                                        },
                                    )
                                }),
                        )
                    } else {
                        None
                    }),
            )
            .into()
    }
}
