use freya::prelude::*;
use rustplus_rs::app_map::Monument as RustPlusMonument;

use crate::{
    components::markers::{Align, base_marker},
    utils::normalize_monument_name,
};

#[derive(PartialEq)]
pub struct Monument {
    pub monument: RustPlusMonument,
    pub map_size: f32,
    pub margin: f32,
}

impl Monument {
    pub fn new(monument: RustPlusMonument, map_size: f32, margin: f32) -> Self {
        Self {
            monument,
            map_size,
            margin,
        }
    }
}

impl Component for Monument {
    fn render(&self) -> impl IntoElement {
        base_marker(
            self.monument.x,
            self.monument.y,
            6.0,
            self.margin,
            self.map_size,
            Align::Center,
        )
        .corner_radius(CornerRadius::new_all(1000.0))
        .main_align(Alignment::Center)
        .cross_align(Alignment::Center)
        .child(
            label()
                .font_size(8.0)
                .max_lines(1)
                .font_family("PermanentMarker")
                .color(Color::from_hex("#191919e6").unwrap())
                .text(normalize_monument_name(self.monument.token.clone())),
        )
    }
}
