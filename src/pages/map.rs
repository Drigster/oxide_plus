use freya::{prelude::*, radio::use_radio};

use crate::{Data, DataChannel, TEXT_COLOR, components::Map as MapComponent};

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
        let radio = use_radio::<Data, DataChannel>(DataChannel::MapSettingsUpdate);
        let grid = radio.slice_current(|s| &s.settings.map_settings.grid);
        let markers = radio.slice_current(|s| &s.settings.map_settings.markers);
        let deaths = radio.slice_current(|s| &s.settings.map_settings.deaths);
        let monuments = radio.slice_current(|s| &s.settings.map_settings.monuments);
        let shops = radio.slice_current(|s| &s.settings.map_settings.shops);
        let center = radio.slice_current(|s| &s.settings.map_settings.center);

        let zoom: State<f32> = use_state(|| 1.0);

        rect().overflow(Overflow::Clip).corner_radius(8.0).child(
            MapComponent::new()
                .grid(grid.into_readable())
                .markers(markers.into_readable())
                .deaths(deaths.into_readable())
                .monuments(monuments.into_readable())
                .shops(shops.into_readable())
                .zoom(zoom)
                .center(center.into_readable()),
        )
    }
}
