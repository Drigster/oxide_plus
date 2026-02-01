use freya::prelude::*;
use rustplus_rs::app_team_info::Note;

use crate::{
    components::markers::{Align, base_marker},
    utils::{index_to_color, index_to_icon},
};

#[derive(PartialEq)]
pub struct MapNote {
    pub map_note: Note,
    pub map_size: f32,
    pub margin: f32,
}

impl MapNote {
    pub fn new(map_note: Note, map_size: f32, margin: f32) -> Self {
        Self {
            map_note,
            map_size,
            margin,
        }
    }
}

impl Component for MapNote {
    fn render(&self) -> impl IntoElement {
        let (color, color_muted) = index_to_color(self.map_note.colour_index);

        base_marker(
            self.map_note.x,
            self.map_note.y - 2.0,
            24.0,
            self.margin,
            self.map_size,
            Align::Bottom,
        )
        .main_align(Alignment::Center)
        .cross_align(Alignment::Center)
        .child(match self.map_note.icon {
            0 => {
                println!("Note: {:?}", self.map_note);
                svg(index_to_icon(self.map_note.icon))
                    .position(Position::new_absolute())
                    .width(Size::px(24.0))
                    .height(Size::px(24.0))
                    .color(color)
                    .stroke(Color::BLACK)
                    .into_element()
            }
            _ => rect()
                .position(Position::new_absolute())
                .width(Size::px(24.0))
                .height(Size::px(24.0))
                .corner_radius(CornerRadius::new_all(1000.0))
                .background(color_muted)
                .border(Border::new().width(2.0).fill(color))
                .border(Border::new().width(1.0).fill(Color::BLACK))
                .center()
                .child(
                    svg(index_to_icon(self.map_note.icon))
                        .width(Size::px(7.0))
                        .height(Size::px(7.0))
                        .color(color),
                )
                .into_element(),
        })
        .child(
            rect().offset_y(21.0).child(
                rect()
                    .background(Color::from_hex("#0000004f").unwrap())
                    .padding(2.0)
                    .min_width(Size::px(28.0))
                    .center()
                    .child(
                        label()
                            .font_family("Roboto Condensed")
                            .font_weight(FontWeight::BLACK)
                            .color(Color::from_hex("#d6cbc1").unwrap())
                            .font_size(18.0)
                            .max_lines(1)
                            .text(self.map_note.label.clone()),
                    ),
            ),
        )
    }
}

// rect()
//     .width(Size::px(14.0))
//     .height(Size::px(14.0))
//     .maybe(map_note.icon != 0, |rect| {
//         rect.corner_radius(
//             CornerRadius::new_all(1000.0),
//         )
//         .background(color_muted)
//         .border(
//             Border::new()
//                 .width(2.0)
//                 .fill(color),
//         )
//         .border(
//             Border::new()
//                 .width(1.0)
//                 .fill(Color::BLACK),
//         )
//     })
//     .position(
//         Position::new_absolute()
//             .left(
//                 (map_note.x )
//                     + map_state.ocean_margin
//                         as f32
//                     - 6.0,
//             )
//             .top(
//                 ((map_size as f32
//                     - map_note.y)
//                     )
//                     + map_state.ocean_margin
//                         as f32
//                     - 6.0,
//             ),
//     )
//     .layer(Layer::Relative(1))
//     .main_align(Alignment::Center)
//     .cross_align(Alignment::Center)
//     .child(match map_note.icon {
//         0 => svg(index_to_icon(map_note.icon))
//             .width(Size::px(16.0))
//             .height(Size::px(16.0))
//             .color(color)
//             .stroke(Color::BLACK),
//         _ => {
//             svg(index_to_icon(map_note.icon))
//                 .width(Size::px(7.0))
//                 .height(Size::px(7.0))
//                 .color(color) // label()
//             //     .position(Position::new_absolute().top(16.0))
//             //     .width(Size::px(500.0))
//             //     .text_align(TextAlign::Center)
//             //     // Magic numbers :)
//             //     .font_size(8.864 / zoom() + 2.446)
//             //     .font_family("PermanentMarker")
//             //     .color(Color::from_hex("#191919e6").unwrap())
//             //     .text(format!("{:?}", map_note.label)),
//         }
//     })
//     .child(
//         rect().offset_y(12.0).child(
//             rect()
//                 .width(Size::Inner)
//                 .height(Size::Inner)
//                 .position(
//                     Position::new_absolute(),
//                 )
//                 .background(Color::RED)
//                 .center()
//                 .child(
//                     label()
//                         .font_family("Roboto")
//                         .font_weight(
//                             FontWeight::BLACK,
//                         )
//                         .font_size(12.0)
//                         .max_lines(1)
//                         .text(
//                             map_note
//                                 .label
//                                 .clone(),
//                         ),
//                 ),
//         ),
//     )
//     .into(),
