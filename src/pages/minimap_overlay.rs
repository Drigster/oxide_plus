use freya::prelude::*;

use crate::components::Map as MapComponent;

#[derive(PartialEq, Clone)]
pub enum Shape {
    Circle,
    Square,
}

#[derive(PartialEq)]
pub struct Minimap {
    shape_state: Shape,
}

impl Minimap {
    pub fn new() -> Self {
        Self {
            shape_state: Shape::Circle,
        }
    }

    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape_state = shape;
        self
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
            .maybe(self.shape_state == Shape::Circle, |rect| {
                rect.corner_radius(1000.0)
            })
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
