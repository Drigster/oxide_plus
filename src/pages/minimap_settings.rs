use chrono::offset;
use freya::prelude::*;
use freya_radio::hooks::use_radio_station;

use crate::{
    app::{Data, DataChannel},
    components::{Setting, SettingType, SliderSettings, ToggleSettings},
    pages::{Minimap, Shape},
};

#[derive(Clone)]
enum Position {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone)]
pub struct MinimapSettings {
    pub enabled: bool,
    pub position: Position,
    pub shape: Shape,
    pub size: f32,
    pub offset: f32,
    pub opacity: f32,
}

impl Default for MinimapSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            position: Position::TopRight,
            shape: Shape::Circle,
            size: 250.0,
            offset: 0.0,
            opacity: 100.0,
        }
    }
}

#[derive(PartialEq)]
pub struct MinimapSettingsPage {}

impl MinimapSettingsPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for MinimapSettingsPage {
    fn render(&self) -> Element {
        let radio_station = use_radio_station::<Data, DataChannel>();

        let mut enabled = use_state(|| false);
        // let mut position = use_state(|| 250.0f32);
        let mut shape = use_state(|| Shape::Circle);
        let mut size = use_state(|| 250.0f32);
        let mut offset = use_state(|| 0.0f32);
        let mut opacity = use_state(|| 0.0f32);

        use_side_effect({
            move || {
                if enabled() {
                    EventNotifier::get().launch_window(
                        WindowConfig::new(move || {
                            use_provide_context(move || radio_station);

                            Minimap::new().shape(shape.read().clone()).into()
                        })
                        .with_size(200.0, 200.0)
                        .with_background(Color::TRANSPARENT)
                        .with_transparency(true)
                        .with_decorations(false)
                        .with_resizable(false),
                    );
                }
            }
        });

        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .padding(8.0)
            .spacing(4.0)
            .children([
                rect()
                    .width(Size::Fill)
                    .height(Size::px(48.0))
                    .padding(8.0)
                    .spacing(8.0)
                    .direction(Direction::Horizontal)
                    .cross_align(Alignment::Center)
                    .children([
                        svg(freya_icons::lucide::map())
                            .height(Size::Fill)
                            .color(Color::from_hex("#5D7238").unwrap())
                            .into(),
                        label()
                            .font_size(24.0)
                            .font_weight(FontWeight::BOLD)
                            .color(Color::from_hex("#E4DAD1").unwrap())
                            .text("POSITION")
                            .into(),
                    ])
                    .into(),
                Setting::new(SettingType::Toggle(ToggleSettings {
                    on_change: Some(EventHandler::new(move |active: bool| {
                        enabled.set(active);
                    })),
                }))
                .text("ENABLED")
                .into(),
                Setting::new(SettingType::Toggle(ToggleSettings { on_change: None }))
                    .text("POSITION")
                    .into(),
                Setting::new(SettingType::Slider(SliderSettings {
                    value: size(),
                    min: 100.0,
                    max: 500.0,
                    step: 10.0,
                    on_change: Some(EventHandler::new(move |new_size: f32| {
                        size.set(new_size);
                    })),
                }))
                .text("SIZE")
                .into(),
                Setting::new(SettingType::Slider(SliderSettings {
                    value: 0.0,
                    min: 0.0,
                    max: 512.0,
                    step: 1.0,
                    on_change: Some(EventHandler::new(move |new_offset: f32| {
                        offset.set(new_offset);
                    })),
                }))
                .text("OFFSET")
                .into(),
                Setting::new(SettingType::Slider(SliderSettings {
                    value: 100.0,
                    min: 1.0,
                    max: 100.0,
                    step: 1.0,
                    on_change: Some(EventHandler::new(move |new_opacity: f32| {
                        opacity.set(new_opacity);
                    })),
                }))
                .text("OPACITY")
                .into(),
                rect()
                    .width(Size::Fill)
                    .height(Size::px(48.0))
                    .padding(8.0)
                    .spacing(8.0)
                    .direction(Direction::Horizontal)
                    .cross_align(Alignment::Center)
                    .children([
                        svg(freya_icons::lucide::toggle_left())
                            .height(Size::Fill)
                            .color(Color::from_hex("#5D7238").unwrap())
                            .into(),
                        label()
                            .font_size(24.0)
                            .font_weight(FontWeight::BOLD)
                            .color(Color::from_hex("#E4DAD1").unwrap())
                            .text("TOGGLES")
                            .into(),
                    ])
                    .into(),
                // Setting::new(SettingType::Toggle).text("GRID").into(),
                // Setting::new(SettingType::Toggle).text("TEAMMATES").into(),
                // Setting::new(SettingType::Toggle).text("MONUMENTS").into(),
                // Setting::new(SettingType::Toggle).text("MARKERS").into(),
                // Setting::new(SettingType::Toggle).text("DEATHS").into(),
            ])
            .into()
    }
}

fn get_text_size_concise(scale: f32) -> f32 {
    return 8.864 / scale + 2.446;
}
