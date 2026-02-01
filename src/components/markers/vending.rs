use freya::prelude::*;
use rustplus_rs::AppMarker;

use crate::components::markers::{Align, base_marker};

#[derive(PartialEq)]
pub struct VendingMachine {
    pub marker: AppMarker,
    pub map_size: f32,
    pub margin: f32,
}

impl VendingMachine {
    pub fn new(marker: AppMarker, map_size: f32, margin: f32) -> Self {
        Self {
            marker,
            map_size,
            margin,
        }
    }
}

impl Component for VendingMachine {
    fn render(&self) -> impl IntoElement {
        base_marker(
            self.marker.x,
            self.marker.y,
            26.0,
            self.margin,
            self.map_size,
            Align::Center,
        )
        .corner_radius(CornerRadius::new_all(1000.0))
        .background(
            Color::from_hex(if self.marker.out_of_stock {
                "#d36516"
            } else {
                "#96d32c"
            })
            .unwrap(),
        )
        .border(
            Border::new()
                .width(1.0)
                .fill(Color::from_hex("#000000DC").unwrap()),
        )
        .main_align(Alignment::Center)
        .cross_align(Alignment::Center)
        .child(
            svg(Bytes::from_static(include_bytes!(
                "../../assets/MDI/cart-variant.svg"
            )))
            .width(Size::px(18.0))
            .height(Size::px(18.0))
            .fill(Color::from_hex("#000000DC").unwrap()),
        )
    }
}
