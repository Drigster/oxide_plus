use freya::prelude::*;
use rustplus_rs::AppMarker;

use crate::components::markers::{Align, base_marker};

#[derive(PartialEq)]
pub struct CargoShip {
    pub marker: AppMarker,
    pub map_size: f32,
    pub margin: f32,
}

impl CargoShip {
    pub fn new(marker: AppMarker, map_size: f32, margin: f32) -> Self {
        Self {
            marker,
            map_size,
            margin,
        }
    }
}

impl Component for CargoShip {
    fn render(&self) -> impl IntoElement {
        base_marker(
            self.marker.x,
            self.marker.y,
            4.0,
            self.margin,
            self.map_size,
            Align::Center,
        )
        .corner_radius(CornerRadius::new_all(1000.0))
        .background(Color::YELLOW)
        .center()
        .child(
            svg(Bytes::from_static(include_bytes!(
                "../../assets/CargoShip.svg"
            )))
            .width(Size::px(50.))
            .height(Size::px(50.))
            .rotate(self.marker.rotation),
        )
    }
}
