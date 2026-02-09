use std::borrow::Cow;

use freya::prelude::*;

use crate::{
    BACKGROUND_COLOR, INPUT_BACKGROUND, SELECT_COLOR, SIDEBAR_BUTTON_BACKGROUND, TEXT_COLOR,
    components::{Button, Dropdown, Slider},
};

#[derive(PartialEq)]
pub struct ToggleSettings {
    pub value: bool,
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
pub struct DropdownSettings {
    pub selected: String,
    pub options: Vec<DropdownOption>,
}

#[derive(PartialEq)]
pub struct DropdownOption {
    pub name: String,
    pub on_select: Option<EventHandler<()>>,
    pub selected: Readable<bool>,
}

#[derive(PartialEq)]
#[allow(dead_code)]
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

impl Component for Setting {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::Fill)
            .height(Size::px(56.0))
            .padding(8.0)
            .direction(Direction::Horizontal)
            .main_align(Alignment::SpaceBetween)
            .cross_align(Alignment::Center)
            .background(Color::from_hex("#0E0E0D80").unwrap())
            .children([
                label()
                    .margin(8.0)
                    .font_size(20.0)
                    .font_weight(FontWeight::BOLD)
                    .color(Color::from_hex(TEXT_COLOR).unwrap())
                    .text(self.text.clone())
                    .into(),
                match &self.setting_type {
                    SettingType::Toggle(settings) => Button::new()
                        .width(Size::px(250.0))
                        .height(Size::Fill)
                        .align(Alignment::Center)
                        .background(Color::from_hex(SIDEBAR_BUTTON_BACKGROUND).unwrap())
                        .background_hover(Color::from_hex("#0E0E0DBF").unwrap())
                        .background_active(Color::from_hex(INPUT_BACKGROUND).unwrap())
                        .text(if settings.value { "ON" } else { "OFF" })
                        .active(settings.value)
                        .on_press({
                            let on_change = settings.on_change.clone();
                            let value = settings.value;
                            move |_| {
                                if let Some(on_change) = &on_change {
                                    let new_state = !value;
                                    on_change.call(new_state);
                                }
                            }
                        })
                        .into(),
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
                    SettingType::Dropdown(dropdown_settings) => Dropdown::new()
                        .width(Size::px(250.0))
                        .height(Size::Fill)
                        .font_size(16.0)
                        .title(dropdown_settings.selected.clone())
                        .background(Color::from_hex(SIDEBAR_BUTTON_BACKGROUND).unwrap())
                        .background_chevron(Color::from_hex(INPUT_BACKGROUND).unwrap())
                        .child(
                            rect()
                                .background(Color::from_hex("#0D0D0C").unwrap())
                                .spacing(2.0)
                                .padding(8.0)
                                .children(dropdown_settings.options.iter().map(|option| {
                                    Button::new()
                                        .width(Size::Fill)
                                        .height(Size::px(36.0))
                                        .align(Alignment::Center)
                                        .background(if *option.selected.read() {
                                            Color::from_hex(SELECT_COLOR).unwrap()
                                        } else {
                                            Color::from_hex(BACKGROUND_COLOR).unwrap()
                                        })
                                        .text(option.name.clone())
                                        .on_press({
                                            let on_select = option.on_select.clone();
                                            move |_| {
                                                if let Some(on_select) = &on_select {
                                                    on_select.call(());
                                                }
                                            }
                                        })
                                        .into()
                                }))
                                .into(),
                        )
                        .into(),
                },
            ])
    }
}
