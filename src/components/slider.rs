use freya::prelude::*;

use crate::{SELECT_COLOR, TEXT_COLOR};

#[derive(Clone, PartialEq)]
pub struct Slider {
    value: f32,
    min: f32,
    max: f32,
    step: f32,

    background: Color,
    background_fill: Color,

    on_changed: Option<EventHandler<f32>>,
}

#[allow(dead_code)]
impl Slider {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            background: Color::from_hex(SELECT_COLOR).unwrap(),
            background_fill: Color::from_hex("#434140").unwrap(),
            on_changed: None,
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn background(mut self, background: impl Into<Color>) -> Self {
        self.background = background.into();
        self
    }

    pub fn background_fill(mut self, background_fill: impl Into<Color>) -> Self {
        self.background_fill = background_fill.into();
        self
    }

    pub fn on_change(mut self, on_changed: impl FnMut(f32) + 'static) -> Self {
        self.on_changed = Some(EventHandler::new(on_changed));
        self
    }
}

impl Component for Slider {
    fn render(&self) -> impl IntoElement {
        let mut clicking = use_state(|| false);
        let mut size = use_state(Area::default);

        let steps = (self.max - self.min) / self.step;

        let slider_pos = use_reactive(&((((self.value - self.min) / self.step) / steps) * 100.0));

        rect()
            .width(Size::px(250.0))
            .height(Size::Fill)
            .direction(Direction::Horizontal)
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            .children([
                rect()
                    .width(Size::px(40.0))
                    .height(Size::Fill)
                    .main_align(Alignment::Center)
                    .child(
                        label()
                            .font_size(16.0)
                            .color(Color::from_hex(TEXT_COLOR).unwrap())
                            .text(self.value.to_string()),
                    )
                    .into(),
                rect()
                    .on_sized(move |e: Event<SizedEventData>| size.set(e.area))
                    .width(Size::Fill)
                    .height(Size::px(10.0))
                    .background_linear_gradient(
                        LinearGradient::new()
                            .angle(-90.0)
                            .stop((Color::from_hex(SELECT_COLOR).unwrap(), 0.0))
                            .stop((Color::from_hex(SELECT_COLOR).unwrap(), slider_pos()))
                            .stop((Color::from_hex("#434140").unwrap(), slider_pos())),
                    )
                    .on_mouse_down(move |e: Event<MouseEventData>| {
                        if e.button != Some(MouseButton::Left) {
                            return;
                        }
                        clicking.set(true);
                    })
                    .on_global_mouse_up(move |e: Event<MouseEventData>| {
                        if e.button != Some(MouseButton::Left) {
                            return;
                        }
                        clicking.set(false);
                    })
                    .on_pointer_press({
                        let min = self.min;
                        let max = self.max;
                        let step = self.step;
                        let on_changed = self.on_changed.clone();
                        move |e: Event<PointerEventData>| {
                            e.stop_propagation();

                            let pos = e.element_location().x as f32;

                            let step_px = size().width() / steps;
                            let step_clicked = (pos / step_px).round();

                            if let Some(on_changed) = &on_changed {
                                on_changed.call((min + step_clicked * step).clamp(min, max));
                            }
                        }
                    })
                    .on_global_mouse_move({
                        let min = self.min;
                        let max = self.max;
                        let step = self.step;
                        let on_changed = self.on_changed.clone();
                        move |e: Event<MouseEventData>| {
                            e.stop_propagation();

                            if !clicking() {
                                return;
                            }

                            let pos = e.global_location.x as f32 - size().origin.x;

                            let step_px = size().width() / steps;
                            let step_clicked = (pos / step_px).round();

                            if let Some(on_changed) = &on_changed {
                                on_changed.call((min + step_clicked * step).clamp(min, max));
                            }
                        }
                    })
                    .on_wheel({
                        let min = self.min;
                        let max = self.max;
                        let step = self.step;
                        let value = self.value;
                        let on_changed = self.on_changed.clone();
                        move |e: Event<WheelEventData>| {
                            e.stop_propagation();
                            if e.delta_y > 0.0 {
                                if value >= max {
                                    return;
                                }

                                if let Some(on_changed) = &on_changed {
                                    on_changed.call(value + step);
                                }
                            } else {
                                if value <= min {
                                    return;
                                }

                                if let Some(on_changed) = &on_changed {
                                    on_changed.call(value - step);
                                }
                            }
                        }
                    })
                    .into(),
            ])
    }
}
