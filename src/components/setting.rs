use std::borrow::Cow;

use freya::prelude::*;

use crate::components::{Button, Dropdown, Slider};

#[derive(PartialEq)]
pub struct ToggleSettings {
    pub on_change: Option<EventHandler<bool>>,
}

#[derive(PartialEq)]
pub struct SliderSettings {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub on_change: Option<EventHandler<f32>>,
}

#[derive(PartialEq)]
pub struct DropdownSettings {}

#[derive(PartialEq)]
pub enum SettingType {
    Toggle(ToggleSettings),
    Slider(SliderSettings),
    Dropdown(DropdownSettings),
}

#[derive(PartialEq)]
pub struct Setting {
    pub setting_type: SettingType,
    pub text: Cow<'static, str>,
}

impl Setting {
    pub fn new(setting_type: SettingType) -> Self {
        Self {
            setting_type,
            text: Cow::from(""),
        }
    }

    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        let text = text.into();
        self.text = text;
        self
    }
}

impl Render for Setting {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::Fill)
            .height(Size::px(48.0))
            .padding(8.0)
            .direction(Direction::Horizontal)
            .main_align(Alignment::SpaceBetween)
            .cross_align(Alignment::Center)
            .background(Color::from_hex("#0E0E0D80").unwrap())
            .children([
                label()
                    .font_size(20.0)
                    .font_weight(FontWeight::BOLD)
                    .color(Color::from_hex("#E4DAD1").unwrap())
                    .text(self.text.clone())
                    .into(),
                match &self.setting_type {
                    SettingType::Toggle(settings) => {
                        let mut toggle = use_state(|| false);

                        Button::new()
                            .width(Size::px(250.0))
                            .height(Size::Fill)
                            .align(Alignment::Center)
                            .background(Color::from_hex("#0E0E0DBF").unwrap())
                            .background_hover(Color::from_hex("#0E0E0DBF").unwrap())
                            .background_active(Color::from_hex("#434140").unwrap())
                            .text(if toggle() { "ON" } else { "OFF" })
                            .on_press({
                                let on_change = settings.on_change.clone();
                                move |_| {
                                    if let Some(on_change) = &on_change {
                                        let new_state = !toggle();
                                        toggle.set(new_state);
                                        on_change.call(new_state);
                                    }
                                }
                            })
                            .into()
                    }
                    SettingType::Slider(settings) => Slider::new()
                        .value(settings.value)
                        .min(settings.min)
                        .max(settings.max)
                        .step(settings.step)
                        .on_change({
                            let on_change = settings.on_change.clone();
                            move |value| {
                                if let Some(on_change) = &on_change {
                                    on_change.call(value);
                                }
                            }
                        })
                        .into(),
                    SettingType::Dropdown(_) => Dropdown::new(vec![
                        crate::components::DropdownOption {
                            icon: None,
                            text: "TOP LEFT".to_string(),
                            on_press: None,
                        },
                        crate::components::DropdownOption {
                            icon: None,
                            text: "TOP RIGHT".to_string(),
                            on_press: None,
                        },
                        crate::components::DropdownOption {
                            icon: None,
                            text: "BOTTOM LEFT".to_string(),
                            on_press: None,
                        },
                        crate::components::DropdownOption {
                            icon: None,
                            text: "BOTTOM RIGHT".to_string(),
                            on_press: None,
                        },
                    ])
                    .width(Size::px(250.0))
                    .height(Size::Fill)
                    .padding((0.0, 16.0))
                    .background(Color::from_hex("#0E0E0DBF").unwrap())
                    .background_chevron(Color::from_hex("#333333").unwrap())
                    .child_height(Size::px(32.0))
                    .into(),
                },
            ])
    }
}
