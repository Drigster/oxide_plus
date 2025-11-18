use std::borrow::Cow;

use freya::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Button {
    width: Size,
    height: Size,
    padding: Gaps,
    spacing: f32,
    icon: Option<Bytes>,
    text: Option<Cow<'static, str>>,
    border: Border,
    background: Color,
    background_hover: Color,
    background_active: Color,
    icon_color: Color,
    icon_color_active: Color,
    color: Color,
    align: Alignment,

    active: bool,
    on_press: Option<EventHandler<Event<PressEventData>>>,

    elements: Vec<Element>,
}

#[allow(dead_code)]
impl Button {
    pub fn new() -> Self {
        Self {
            width: Size::default(),
            height: Size::default(),
            padding: Gaps::new_all(8.0),
            spacing: 8.0,
            icon: None,
            text: None,
            border: Border::new(),
            background: Color::from_hex("#0DFFFFFF").unwrap(),
            background_hover: Color::from_hex("#2DFFFFFF").unwrap(),
            background_active: Color::from_hex("#5D7238").unwrap(),
            icon_color: Color::from_hex("#605B55").unwrap(),
            icon_color_active: Color::from_hex("#E4DAD1").unwrap(),
            color: Color::from_hex("#E4DAD1").unwrap(),
            align: Alignment::Start,

            active: false,
            on_press: None,

            elements: Vec::new(),
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

    pub fn padding(mut self, padding: impl Into<Gaps>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn icon(mut self, icon: Bytes) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        let text = text.into();
        self.text = Some(text);
        self
    }

    pub fn border(mut self, border: impl Into<Border>) -> Self {
        self.border = border.into();
        self
    }

    pub fn background(mut self, background: impl Into<Color>) -> Self {
        self.background = background.into();
        self
    }

    pub fn background_hover(mut self, background_hover: impl Into<Color>) -> Self {
        self.background_hover = background_hover.into();
        self
    }

    pub fn background_active(mut self, background_active: impl Into<Color>) -> Self {
        self.background_active = background_active.into();
        self
    }

    pub fn icon_color(mut self, icon_color: impl Into<Color>) -> Self {
        self.icon_color = icon_color.into();
        self
    }

    pub fn icon_color_active(mut self, icon_color_active: impl Into<Color>) -> Self {
        self.icon_color_active = icon_color_active.into();
        self
    }

    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }

    pub fn align(mut self, main_align: Alignment) -> Self {
        self.align = main_align;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn on_press(mut self, on_press: impl FnMut(Event<PressEventData>) + 'static) -> Self {
        self.on_press = Some(EventHandler::new(on_press));
        self
    }
}

impl ChildrenExt for Button {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl MaybeExt for Button {}

impl Render for Button {
    fn render(&self) -> Element {
        let mut hovered = use_state(|| false);

        let background_color = if self.active {
            self.background_active.clone()
        } else if hovered() {
            self.background_hover.clone()
        } else {
            self.background.clone()
        };

        let icon_color = if self.active {
            self.icon_color_active.clone()
        } else {
            self.icon_color.clone()
        };

        use_drop(move || {
            if hovered() {
                Cursor::set(CursorIcon::default());
            }
        });

        rect()
            .width(self.width.clone())
            .height(self.height.clone())
            .background(background_color)
            .direction(Direction::Horizontal)
            .padding(self.padding.clone())
            .spacing(self.spacing)
            .cross_align(Alignment::Center)
            .main_align(self.align.clone())
            .border(self.border.clone())
            .on_pointer_enter(move |_| {
                hovered.set_if_modified(true);
                Cursor::set(CursorIcon::Pointer);
            })
            .on_pointer_leave(move |_| {
                hovered.set_if_modified(false);
                Cursor::set(CursorIcon::default());
            })
            .on_press({
                let on_press = self.on_press.clone();
                move |e| {
                    if let Some(on_press) = &on_press {
                        on_press.call(e);
                    } else {
                        e.stop_propagation();
                    }
                }
            })
            .maybe_child(if let Some(icon) = &self.icon {
                Some(svg(icon.clone()).height(Size::Fill).color(icon_color))
            } else {
                None
            })
            .maybe_child(if let Some(text) = &self.text {
                Some(
                    label()
                        .font_size(16.0)
                        .font_weight(FontWeight::BOLD)
                        .color(Color::from_hex("#E4DAD1").unwrap())
                        .text(text.clone()),
                )
            } else {
                None
            })
            .maybe(!self.elements.is_empty(), |rect| {
                rect.children(self.elements.clone())
            })
            .into()
    }
}
