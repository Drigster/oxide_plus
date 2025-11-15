use freya::prelude::*;
use freya_radio::hooks::use_radio;

use crate::{
    app::{Data, DataChannel},
    components::Map as MapComponent,
};

#[derive(Clone)]
pub struct MapSettings {
    pub grid: bool,
    pub markers: bool,
    pub deaths: bool,
    pub monuments: bool,
    pub shops: bool,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
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

impl Render for Map {
    fn render(&self) -> Element {
        let map_settings_binding = use_radio::<Data, DataChannel>(DataChannel::MapStateUpdate);
        let map_settings = &map_settings_binding.read().settings.map_settings;

        rect()
            .padding(8.0)
            .child(
                MapComponent::new()
                    .grid(map_settings.grid)
                    .markers(map_settings.markers)
                    .deaths(map_settings.deaths)
                    .monuments(map_settings.monuments)
                    .shops(map_settings.shops),
            )
            .into()
    }
}

fn get_text_size_concise(scale: f32) -> f32 {
    return 8.864 / scale + 2.446;
}
