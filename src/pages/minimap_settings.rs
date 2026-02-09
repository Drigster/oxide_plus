use freya::{prelude::*, radio::use_radio};
use serde::{Deserialize, Serialize};

use crate::{
    BACKGROUND_COLOR, Data, DataChannel, SELECT_COLOR, TEXT_COLOR,
    components::{
        DropdownOption, DropdownSettings, MAX_ZOOM, MIN_ZOOM, Setting, SettingType, SliderSettings,
        ToggleSettings,
    },
    pages::Shape,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Position {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MinimapSettings {
    pub enabled: bool,
    pub position: Position,
    pub shape: Shape,
    pub size: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub opacity: f32,
    pub zoom: f32,

    pub grid: bool,
    pub markers: bool,
    pub deaths: bool,
    pub monuments: bool,
    pub shops: bool,
}

impl Default for MinimapSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            position: Position::TopRight,
            shape: Shape::Circle,
            size: 250.0,
            offset_x: 0.0,
            offset_y: 0.0,
            opacity: 100.0,
            zoom: 1.0,

            grid: true,
            markers: true,
            deaths: true,
            monuments: true,
            shops: true,
        }
    }
}

#[derive(PartialEq)]
pub struct MinimapSettingsPage {}

impl Component for MinimapSettingsPage {
    fn render(&self) -> impl IntoElement {
        let mut minimap_settings_state =
            use_radio::<Data, DataChannel>(DataChannel::MinimapSettingsUpdate);
        let monitor_size =
            minimap_settings_state.slice(DataChannel::MonitorSizeUpdate, |s| &s.monitor_size);

        let state_tx = minimap_settings_state.read().state_tx.clone().unwrap();

        use_hook(|| {
            Platform::get().with_window(None, move |window| {
                minimap_settings_state
                    .write_channel(DataChannel::MonitorSizeUpdate)
                    .monitor_size = Some(window.current_monitor().unwrap().size());
            });
        });

        let max_offset_x = if let Some(monitor_size) = *monitor_size.read() {
            monitor_size.width as f32
        } else {
            1920.0
        } - minimap_settings_state.read().settings.minimap_settings.size;
        let max_offset_y = if let Some(monitor_size) = *monitor_size.read() {
            monitor_size.height as f32
        } else {
            1080.0
        } - minimap_settings_state.read().settings.minimap_settings.size;

        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .background(Color::from_hex(BACKGROUND_COLOR).unwrap())
            .corner_radius(8.0)
            .child(
                ScrollView::new()
                    .width(Size::Fill)
                    .height(Size::Fill)
                    .child(
                        rect().padding(8.0).spacing(4.0).children([
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
                                        .color(Color::from_hex(SELECT_COLOR).unwrap())
                                        .into(),
                                    label()
                                        .font_size(24.0)
                                        .font_weight(FontWeight::BOLD)
                                        .color(Color::from_hex(TEXT_COLOR).unwrap())
                                        .text("MINIMAP")
                                        .into(),
                                ])
                                .into(),
                            Setting::new(SettingType::Toggle(ToggleSettings {
                                value: minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .enabled,
                                on_change: Some(EventHandler::new(move |value: bool| {
                                    state_tx
                                        .unbounded_send(crate::ChannelSend::ToggleMinimap(value))
                                        .unwrap();
                                    minimap_settings_state
                                        .write()
                                        .settings
                                        .minimap_settings
                                        .enabled = value;
                                })),
                            }))
                            .text("ENABLED")
                            .into(),
                            Setting::new(SettingType::Dropdown(DropdownSettings {
                                selected: match minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .position
                                {
                                    Position::TopLeft => "Top Left".to_string(),
                                    Position::TopRight => "Top Right".to_string(),
                                    Position::BottomLeft => "Bottom Left".to_string(),
                                    Position::BottomRight => "Bottom Right".to_string(),
                                },
                                options: vec![
                                    DropdownOption {
                                        name: "Top Left".to_string(),
                                        on_select: Some(EventHandler::new(move |_| {
                                            minimap_settings_state
                                                .write()
                                                .settings
                                                .minimap_settings
                                                .position = Position::TopLeft;
                                        })),
                                        selected: (minimap_settings_state
                                            .read()
                                            .settings
                                            .minimap_settings
                                            .position
                                            == Position::TopLeft)
                                            .into(),
                                    },
                                    DropdownOption {
                                        name: "Top Right".to_string(),
                                        on_select: Some(EventHandler::new(move |_| {
                                            minimap_settings_state
                                                .write()
                                                .settings
                                                .minimap_settings
                                                .position = Position::TopRight;
                                        })),
                                        selected: (minimap_settings_state
                                            .read()
                                            .settings
                                            .minimap_settings
                                            .position
                                            == Position::TopRight)
                                            .into(),
                                    },
                                    DropdownOption {
                                        name: "Bottom Left".to_string(),
                                        on_select: Some(EventHandler::new(move |_| {
                                            minimap_settings_state
                                                .write()
                                                .settings
                                                .minimap_settings
                                                .position = Position::BottomLeft;
                                        })),
                                        selected: (minimap_settings_state
                                            .read()
                                            .settings
                                            .minimap_settings
                                            .position
                                            == Position::BottomLeft)
                                            .into(),
                                    },
                                    DropdownOption {
                                        name: "Bottom Right".to_string(),
                                        on_select: Some(EventHandler::new(move |_| {
                                            minimap_settings_state
                                                .write()
                                                .settings
                                                .minimap_settings
                                                .position = Position::BottomRight;
                                        })),
                                        selected: (minimap_settings_state
                                            .read()
                                            .settings
                                            .minimap_settings
                                            .position
                                            == Position::BottomRight)
                                            .into(),
                                    },
                                ],
                            }))
                            .text("POSITION")
                            .into(),
                            Setting::new(SettingType::Dropdown(DropdownSettings {
                                selected: match minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .shape
                                {
                                    Shape::Circle => "Circle".to_string(),
                                    Shape::Square => "Square".to_string(),
                                },
                                options: vec![
                                    DropdownOption {
                                        name: "Circle".to_string(),
                                        on_select: Some(EventHandler::new(move |_| {
                                            minimap_settings_state
                                                .write()
                                                .settings
                                                .minimap_settings
                                                .shape = Shape::Circle;
                                        })),
                                        selected: (minimap_settings_state
                                            .read()
                                            .settings
                                            .minimap_settings
                                            .shape
                                            == Shape::Circle)
                                            .into(),
                                    },
                                    DropdownOption {
                                        name: "Square".to_string(),
                                        on_select: Some(EventHandler::new(move |_| {
                                            minimap_settings_state
                                                .write()
                                                .settings
                                                .minimap_settings
                                                .shape = Shape::Square;
                                        })),
                                        selected: (minimap_settings_state
                                            .read()
                                            .settings
                                            .minimap_settings
                                            .shape
                                            == Shape::Square)
                                            .into(),
                                    },
                                ],
                            }))
                            .text("SHAPE")
                            .into(),
                            Setting::new(SettingType::Slider(SliderSettings {
                                value: minimap_settings_state.read().settings.minimap_settings.size,
                                min: 100.0,
                                max: 500.0,
                                step: 10.0,
                                on_change: Some(EventHandler::new(move |value: f32| {
                                    println!("New size: {}", value);
                                    minimap_settings_state
                                        .write()
                                        .settings
                                        .minimap_settings
                                        .size = value;
                                })),
                            }))
                            .text("SIZE")
                            .into(),
                            Setting::new(SettingType::Slider(SliderSettings {
                                value: minimap_settings_state.read().settings.minimap_settings.zoom,
                                min: MIN_ZOOM as f32,
                                max: MAX_ZOOM as f32,
                                step: 0.1,
                                on_change: Some(EventHandler::new(move |value: f32| {
                                    minimap_settings_state
                                        .write()
                                        .settings
                                        .minimap_settings
                                        .zoom = value;
                                })),
                            }))
                            .text("ZOOM")
                            .into(),
                            Setting::new(SettingType::Slider(SliderSettings {
                                value: minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .offset_x,
                                min: 0.0,
                                max: max_offset_x,
                                step: 1.0,
                                on_change: Some(EventHandler::new(move |value: f32| {
                                    minimap_settings_state
                                        .write()
                                        .settings
                                        .minimap_settings
                                        .offset_x = value;
                                })),
                            }))
                            .text("OFFSET_X")
                            .into(),
                            Setting::new(SettingType::Slider(SliderSettings {
                                value: minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .offset_y,
                                min: 0.0,
                                max: max_offset_y,
                                step: 1.0,
                                on_change: Some(EventHandler::new(move |value: f32| {
                                    minimap_settings_state
                                        .write()
                                        .settings
                                        .minimap_settings
                                        .offset_y = value;
                                })),
                            }))
                            .text("OFFSET_Y")
                            .into(),
                            Setting::new(SettingType::Slider(SliderSettings {
                                value: minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .opacity,
                                min: 1.0,
                                max: 100.0,
                                step: 1.0,
                                on_change: Some(EventHandler::new(move |value: f32| {
                                    minimap_settings_state
                                        .write()
                                        .settings
                                        .minimap_settings
                                        .opacity = value;
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
                                        .color(Color::from_hex(SELECT_COLOR).unwrap())
                                        .into(),
                                    label()
                                        .font_size(24.0)
                                        .font_weight(FontWeight::BOLD)
                                        .color(Color::from_hex(TEXT_COLOR).unwrap())
                                        .text("TOGGLES")
                                        .into(),
                                ])
                                .into(),
                            Setting::new(SettingType::Toggle(ToggleSettings {
                                value: minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .deaths,
                                on_change: Some(EventHandler::new(move |value: bool| {
                                    minimap_settings_state
                                        .write()
                                        .settings
                                        .minimap_settings
                                        .deaths = value;
                                })),
                            }))
                            .text("DEATHS")
                            .into(),
                            Setting::new(SettingType::Toggle(ToggleSettings {
                                value: minimap_settings_state.read().settings.minimap_settings.grid,
                                on_change: Some(EventHandler::new(move |value: bool| {
                                    minimap_settings_state
                                        .write()
                                        .settings
                                        .minimap_settings
                                        .grid = value;
                                })),
                            }))
                            .text("GRID")
                            .into(),
                            Setting::new(SettingType::Toggle(ToggleSettings {
                                value: minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .markers,
                                on_change: Some(EventHandler::new(move |value: bool| {
                                    minimap_settings_state
                                        .write()
                                        .settings
                                        .minimap_settings
                                        .markers = value;
                                })),
                            }))
                            .text("MARKERS")
                            .into(),
                            Setting::new(SettingType::Toggle(ToggleSettings {
                                value: minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .monuments,
                                on_change: Some(EventHandler::new(move |value: bool| {
                                    minimap_settings_state
                                        .write()
                                        .settings
                                        .minimap_settings
                                        .monuments = value;
                                })),
                            }))
                            .text("MONUMENTS")
                            .into(),
                            Setting::new(SettingType::Toggle(ToggleSettings {
                                value: minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .shops,
                                on_change: Some(EventHandler::new(move |value: bool| {
                                    minimap_settings_state
                                        .write()
                                        .settings
                                        .minimap_settings
                                        .shops = value;
                                })),
                            }))
                            .text("SHOPS")
                            .into(),
                        ]),
                    ),
            )
    }
}
