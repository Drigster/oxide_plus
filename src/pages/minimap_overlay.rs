use freya::prelude::*;
use freya_radio::hooks::use_radio;

use crate::{
    app::{Data, DataChannel},
    components::Map as MapComponent,
};

#[derive(PartialEq, Clone, Debug)]
pub enum Shape {
    Circle,
    Square,
}

#[derive(PartialEq)]
pub struct Minimap {}

impl Minimap {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for Minimap {
    fn render(&self) -> Element {
        let minimap_settings_state = use_radio::<Data, DataChannel>(DataChannel::MinimapSettingsUpdate);

        let grid = use_state(|| true);
        let markers = use_state(|| true);
        let deaths = use_state(|| true);
        let monuments = use_state(|| true);
        let shops = use_state(|| true);

        rect()
            .width(Size::percent(100.0))
            .height(Size::percent(100.0))
            .maybe(minimap_settings_state.read().settings.minimap_settings.shape == Shape::Circle, |rect| {
                rect.corner_radius(1000.0)
            })
            .overflow_mode(OverflowMode::Clip)
            .child(
                MapComponent::new()
                    .interactable(false)
                    .center(true)
                    .grid(grid())
                    .markers(markers())
                    .deaths(deaths())
                    .monuments(monuments())
                    .shops(shops()),
            )
            .into()
    }
}
