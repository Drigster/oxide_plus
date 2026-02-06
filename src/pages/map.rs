use freya::{prelude::*, radio::use_radio};

use crate::{
    components::Map as MapComponent,
    {Data, DataChannel},
};

#[derive(Clone)]
pub struct MapSettings {
    pub center: bool,
    pub grid: bool,
    pub markers: bool,
    pub deaths: bool,
    pub monuments: bool,
    pub shops: bool,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            center: false,
            grid: true,
            markers: true,
            deaths: true,
            monuments: true,
            shops: true,
        }
    }
}

#[derive(PartialEq)]
pub struct Map {}

impl Component for Map {
    fn render(&self) -> impl IntoElement {
        let map_settings_binding = use_radio::<Data, DataChannel>(DataChannel::MapSettingsUpdate);
        let map_settings = map_settings_binding.read().settings.map_settings.clone();

        let map_state_binding = use_radio::<Data, DataChannel>(DataChannel::MapStateUpdate);
        let map_state = map_state_binding.read().map_state.clone();
        let marker_state_binding = use_radio::<Data, DataChannel>(DataChannel::MapMarkersUpdate);
        let marker_state = marker_state_binding.read().map_markers.clone();

        let mut zoom: State<f32> = use_state(|| 1.0);

        if let (Some(map_state), Some(marker_state)) = (map_state, marker_state) {
            rect().child(
                MapComponent::new()
                    .grid(map_settings.grid)
                    .markers(map_settings.markers)
                    .deaths(map_settings.deaths)
                    .monuments(map_settings.monuments)
                    .shops(map_settings.shops)
                    .zoom(zoom())
                    .center(map_settings.center)
                    .on_zoom(move |v| {
                        zoom.set(v);
                    })
                    .on_center_cancel({
                        let mut map_settings_binding = map_settings_binding.clone();
                        move |_| {
                            map_settings_binding.write().settings.map_settings.center = false;
                        }
                    }),
            )
        } else {
            rect()
                .expanded()
                .background(Color::from_hex("#191919e6").unwrap())
                .center()
                .child(label().text("Map data is loading..."))
        }
    }
}
