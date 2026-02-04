use freya::{
    prelude::*,
    radio::use_radio,
    tray::dpi::{PhysicalPosition, PhysicalSize},
};

use crate::{
    components::Map as MapComponent,
    {Data, DataChannel},
};

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum Shape {
    Circle,
    Square,
}

#[derive(PartialEq)]
pub struct Minimap {}

#[allow(dead_code)]
impl Minimap {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Minimap {
    fn render(&self) -> impl IntoElement {
        let minimap_settings_state =
            use_radio::<Data, DataChannel>(DataChannel::MinimapSettingsUpdate);

        use_side_effect(move || {
            let offset_x = minimap_settings_state
                .read()
                .settings
                .minimap_settings
                .offset_x
                .clone();

            let offset_y = minimap_settings_state
                .read()
                .settings
                .minimap_settings
                .offset_y
                .clone();

            Platform::get().with_window(None, move |window| {
                window.set_outer_position(PhysicalPosition::new(offset_x, offset_y));
            });
        });

        use_side_effect(move || {
            let size = minimap_settings_state
                .read()
                .settings
                .minimap_settings
                .size
                .clone();

            Platform::get().with_window(None, move |window| {
                let _ = window.request_inner_size(PhysicalSize::new(size, size));
            });
        });

        let map_state_binding = use_radio::<Data, DataChannel>(DataChannel::MapStateUpdate);
        let map_state = map_state_binding.read().map_state.clone();
        let info_state_binding = use_radio::<Data, DataChannel>(DataChannel::InfoStateUpdate);
        let info_state = info_state_binding.read().info_state.clone();
        let marker_state_binding = use_radio::<Data, DataChannel>(DataChannel::MapMarkersUpdate);
        let marker_state = marker_state_binding.read().map_markers.clone();
        let team_info_binding = use_radio::<Data, DataChannel>(DataChannel::TeamInfoUpdate);
        let team_info = team_info_binding.read().team_info.clone();

        if let (Some(map_state), Some(info_state), Some(marker_state), Some(team_info)) =
            (map_state, info_state, marker_state, team_info)
        {
            rect()
                .width(Size::percent(100.0))
                .height(Size::percent(100.0))
                .maybe(
                    minimap_settings_state
                        .read()
                        .settings
                        .minimap_settings
                        .shape
                        == Shape::Circle,
                    |rect| rect.corner_radius(1000.0),
                )
                .overflow(Overflow::Clip)
                .maybe_child({
                    Some(
                        MapComponent::new()
                            .interactable(false)
                            .center(true)
                            .background_opacity(
                                minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .opacity
                                    / 100.0,
                            )
                            .zoom(minimap_settings_state.read().settings.minimap_settings.zoom)
                            .grid(minimap_settings_state.read().settings.minimap_settings.grid)
                            .markers(
                                minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .markers,
                            )
                            .deaths(
                                minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .deaths,
                            )
                            .monuments(
                                minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .monuments,
                            )
                            .shops(
                                minimap_settings_state
                                    .read()
                                    .settings
                                    .minimap_settings
                                    .shops,
                            ),
                    )
                })
        } else {
            rect()
                .margin(8.0)
                .expanded()
                .background(Color::from_hex("#191919").unwrap())
                .corner_radius(CornerRadius::new_all(16.0))
                .center()
                .child(label().text("Map data is loading..."))
        }
    }
}
