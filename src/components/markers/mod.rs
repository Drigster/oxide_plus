use freya::{prelude::*, radio::use_radio};

mod player;
pub use player::*;
mod map_note;
pub use map_note::*;
mod monument;
pub use monument::*;
mod death;
pub use death::*;
mod vending;
pub use vending::*;
mod cargo;
pub use cargo::*;

use crate::{Data, DataChannel};

#[derive(PartialEq)]
pub enum Align {
    Center,
    Bottom,
}

pub fn base_marker(x: f32, y: f32, size: f32, margin: f32, map_size: f32, align: Align) -> Rect {
    rect()
        .width(Size::px(size))
        .height(Size::px(size))
        .position(Position::new_absolute().left(x - size / 2.0 + margin).top(
            map_size
                - y
                - if align == Align::Bottom {
                    size
                } else {
                    size / 2.0
                }
                - margin,
        ))
}
