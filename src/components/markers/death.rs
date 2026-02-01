use freya::prelude::*;
use freya_icons::lucide;
use rustplus_rs::app_team_info::Note;

use crate::components::markers::{Align, base_marker};

#[derive(PartialEq)]
pub struct Death {
    pub map_note: Note,
    pub map_size: f32,
    pub margin: f32,
}

impl Death {
    pub fn new(map_note: Note, map_size: f32, margin: f32) -> Self {
        Self {
            map_note,
            map_size,
            margin,
        }
    }
}

impl Component for Death {
    fn render(&self) -> impl IntoElement {
        base_marker(
            self.map_note.x,
            self.map_note.y,
            12.0,
            self.margin,
            self.map_size,
            Align::Center,
        )
        .layer(Layer::Relative(1))
        .main_align(Alignment::Center)
        .cross_align(Alignment::Center)
        .child(
            svg(lucide::skull())
                .width(Size::px(8.0))
                .height(Size::px(8.0))
                .color(Color::RED),
        )
    }
}
