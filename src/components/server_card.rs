use freya::prelude::*;

use crate::{colors, components::CachedImage};

#[derive(Clone, PartialEq)]
pub struct ServerCard {
    pub icon_url: String,
    pub name: String,
    pub on_press: Option<EventHandler<Event<PressEventData>>>,
}

impl ServerCard {
    pub fn new(icon_url: String, name: String) -> Self {
        Self {
            icon_url,
            name,
            on_press: None,
        }
    }

    pub fn on_press(mut self, on_press: impl FnMut(Event<PressEventData>) + 'static) -> Self {
        self.on_press = Some(EventHandler::new(on_press));
        self
    }
}

impl Component for ServerCard {
    fn render(&self) -> impl IntoElement {
        let mut hovering = use_state(|| false);

        // use_drop(move || {
        //     if hovering() {
        //         Cursor::set(CursorIcon::default());
        //     }
        // });

        let background = if hovering() { "#333333" } else { "#222222" };

        rect()
            .width(Size::px(250.0))
            .height(Size::px(48.0))
            .padding(4.0)
            .background(Color::from_hex(background).unwrap())
            .direction(Direction::Horizontal)
            .cross_align(Alignment::Center)
            // .border(
            //     Border::new()
            //         .width(BorderWidth {
            //             top: 0.0,
            //             right: 0.0,
            //             bottom: 1.0,
            //             left: 0.0,
            //         })
            //         .alignment(BorderAlignment::Outer)
            //         .fill(Color::from_hex(colors::BORDER).unwrap()),
            // )
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
            .children([
                CachedImage::new(self.icon_url.clone()).into_element(),
                rect()
                    .padding(8.0)
                    .child(
                        label()
                            .font_size(12.0)
                            .font_weight(FontWeight::BOLD)
                            .color(Color::from_hex(colors::TEXT).unwrap())
                            .text(self.name.clone()),
                    )
                    .into(),
            ])
    }
}
