use freya::prelude::*;

use crate::components::Map as MapComponent;

#[derive(PartialEq)]
pub struct Minimap {}

impl Minimap {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for Minimap {
    fn render(&self) -> Element {
        let grid = use_state(|| true);
        let teammates = use_state(|| true);
        let deaths = use_state(|| true);
        let monuments = use_state(|| true);
        let shops = use_state(|| true);

        rect()
            .width(Size::percent(100.0))
            .height(Size::percent(100.0))
            .corner_radius(1000.0)
            .overflow_mode(OverflowMode::Clip)
            .child(
                MapComponent::new()
                    .interactable(false)
                    .center(true)
                    .grid(grid())
                    .teammates(teammates())
                    .deaths(deaths())
                    .monuments(monuments())
                    .shops(shops()),
            )
            .into()
    }
}
