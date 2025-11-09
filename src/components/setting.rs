use std::borrow::Cow;

use freya::prelude::*;

use crate::components::Button;

#[derive(PartialEq)]
pub struct SettingSlider {
    value: f32,
    min: f32,
    max: f32,
    step: f32,
}

#[derive(PartialEq)]
pub enum SettingType {
    Toggle,
    Slider(SettingSlider),
    Dropdown,
}

#[derive(PartialEq)]
pub struct Setting {
    pub setting_type: SettingType,
    pub text: Cow<'static, str>,
    pub on_change: Option<EventHandler<bool>>,
}

impl Setting {
    pub fn new(setting_type: SettingType) -> Self {
        Self {
            setting_type,
            text: Cow::from(""),
            on_change: None,
        }
    }

    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        let text = text.into();
        self.text = text;
        self
    }

    pub fn on_change(mut self, on_change: impl FnMut(bool) + 'static) -> Self {
        self.on_change = Some(EventHandler::new(on_change));
        self
    }
}

impl Render for Setting {
    fn render(&self) -> Element {
        rect()
            .width(Size::Fill)
            .height(Size::px(48.0))
            .padding(8.0)
            .direction(Direction::Horizontal)
            .main_align(Alignment::SpaceBetween)
            .cross_align(Alignment::Center)
            .background(Color::from_hex("#800E0E0D").unwrap())
            .children([
                label()
                    .font_size(20.0)
                    .font_weight(FontWeight::BOLD)
                    .color(Color::from_hex("#E4DAD1").unwrap())
                    .text(self.text.clone())
                    .into(),
                match &self.setting_type {
                    SettingType::Toggle => {
                        let mut toggle = use_state(|| false);

                        Button::new()
                            .width(Size::px(250.0))
                            .height(Size::Fill)
                            .align(Alignment::Center)
                            .background(Color::from_hex("#BF0E0E0D").unwrap())
                            .background_hover(Color::from_hex("#BF0E0E0D").unwrap())
                            .background_active(Color::from_hex("#434140").unwrap())
                            .text(if toggle() { "ON" } else { "OFF" })
                            .on_press({
                                let on_change = self.on_change.clone();
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
                    SettingType::Slider(_) => {
                        rect().width(Size::px(250.0)).height(Size::Fill).into()
                    }
                    SettingType::Dropdown => {
                        rect().width(Size::px(250.0)).height(Size::Fill).into()
                    }
                },
            ])
            .into()
    }
}
