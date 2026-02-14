use freya::{animation::*, prelude::*, radio::use_radio};

use crate::{DataChannel, colors};

#[derive(PartialEq, Clone)]
pub enum Timeout {
    Default,
    Custom(u64),
    Infinite,
}

#[derive(PartialEq, Clone)]
pub struct Toast {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub timeout: Timeout,
    pub on_press: Option<EventHandler<()>>,
}

impl Toast {
    pub fn new(id: u64, title: String, message: String) -> Self {
        Self {
            id,
            title,
            message,
            timeout: Timeout::Default,
            on_press: None,
        }
    }

    pub fn timeout(mut self, timeout: Timeout) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn on_press(mut self, on_press: impl FnMut(()) + 'static) -> Self {
        self.on_press = Some(EventHandler::new(on_press));
        self
    }
}

impl Component for Toast {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio(DataChannel::ToastsUpdate);

        let timeout = match self.timeout {
            Timeout::Default => 3000,
            Timeout::Custom(timeout) => timeout,
            Timeout::Infinite => u64::MAX,
        };

        let mut pos_animation = use_animation(move |conf| {
            conf.on_creation(OnCreation::Run);
            AnimNum::new(300.0, 0.0)
                .function(Function::Linear)
                .ease(Ease::InOut)
                .time(300)
        });

        let mut timeout_animation = use_animation(move |_| {
            AnimNum::new(0.0, 100.0)
                .function(Function::Linear)
                .ease(Ease::InOut)
                .time(timeout)
        });

        let offset_x = pos_animation.read().value();
        let timeout_value = timeout_animation.read().value();

        use_side_effect({
            let id = self.id;
            move || {
                if pos_animation
                    .read()
                    .is_finished(300, AnimDirection::Forward)
                    == true
                    && *pos_animation.direction() == AnimDirection::Forward
                {
                    timeout_animation.start();
                } else if pos_animation
                    .read()
                    .is_finished(300, AnimDirection::Reverse)
                    == true
                    && *pos_animation.direction() == AnimDirection::Reverse
                {
                    radio.write().toasts.remove(&id);
                }
            }
        });

        use_side_effect(move || {
            if timeout_animation
                .read()
                .is_finished(timeout as u128, AnimDirection::Forward)
                == true
            {
                pos_animation.reverse();
            }
        });

        rect().offset_x(offset_x).child(
            rect()
                .background(Color::from_hex(colors::BACKGROUND).unwrap())
                .height(Size::px(48.0))
                .width(Size::px(250.0))
                .padding((0.0, 0.0, 0.0, 6.0))
                .border(
                    Border::new()
                        .fill(Color::from_hex(colors::SELECT).unwrap())
                        .alignment(BorderAlignment::Inner)
                        .width(BorderWidth {
                            left: 6.0,
                            right: 0.0,
                            top: 0.0,
                            bottom: 0.0,
                        }),
                )
                .shadow(
                    Shadow::new()
                        .blur(4.0)
                        .spread(2.0)
                        .color(Color::from_hex("#00000080").unwrap()),
                )
                .on_press({
                    let on_press = self.on_press.clone();
                    move |e: Event<PressEventData>| {
                        if let Some(on_press) = &on_press {
                            on_press.call(());
                        } else {
                            e.stop_propagation();
                        }
                    }
                })
                .children([
                    rect()
                        .width(Size::Fill)
                        .height(Size::Fill)
                        .main_align(Alignment::SpaceBetween)
                        .padding((2.0, 2.0, 4.0, 4.0))
                        .children([
                            label()
                                .width(Size::Fill)
                                .max_lines(1)
                                .color(Color::from_hex(colors::TEXT).unwrap())
                                .font_size(20.0)
                                .text_overflow(TextOverflow::Custom("...".to_owned()))
                                .text(self.title.clone())
                                .into(),
                            label()
                                .width(Size::Fill)
                                .max_lines(1)
                                .color(Color::from_hex(colors::TEXT).unwrap())
                                .font_size(16.0)
                                .text_overflow(TextOverflow::Custom("...".to_owned()))
                                .text(self.message.clone())
                                .into(),
                        ])
                        .into(),
                    rect()
                        .width(Size::Fill)
                        .height(Size::px(2.0))
                        .position(Position::new_absolute().bottom(0.0))
                        .background_linear_gradient(
                            LinearGradient::new()
                                .angle(-90.0)
                                .stop((Color::TRANSPARENT, timeout_value))
                                .stop((Color::from_hex(colors::SELECT).unwrap(), timeout_value))
                                .stop((Color::from_hex(colors::SELECT).unwrap(), 100.0)),
                        )
                        .into(),
                ]),
        )
    }
}
