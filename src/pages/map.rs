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
    pub center: bool,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            grid: true,
            markers: true,
            deaths: true,
            monuments: true,
            shops: true,
            center: false,
        }
    }
}

#[derive(PartialEq)]
pub struct Map {}

impl Render for Map {
    fn render(&self) -> Element {
        let map_settings_state = use_radio::<Data, DataChannel>(DataChannel::MapSettingsUpdate);

        rect()
            .padding(8.0)
            .child(
                MapComponent::new()
                    .grid(map_settings_state.read().settings.map_settings.grid)
                    .markers(map_settings_state.read().settings.map_settings.markers)
                    .deaths(map_settings_state.read().settings.map_settings.deaths)
                    .monuments(map_settings_state.read().settings.map_settings.monuments)
                    .shops(map_settings_state.read().settings.map_settings.shops)
                    .center(map_settings_state.read().settings.map_settings.center)
                    .interactable(!map_settings_state.read().settings.map_settings.center),
            )
            .into()
    }
}
