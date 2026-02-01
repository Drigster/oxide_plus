use freya::prelude::*;
use rustplus_rs::{AppMarker, app_team_info::Member};

use crate::components::markers::{Align, base_marker};

#[derive(PartialEq)]
pub struct Player {
    pub member: Member,
    pub map_size: f32,
    pub margin: f32,
    pub me: bool,
}

impl Player {
    pub fn new(member: Member, map_size: f32, margin: f32, me: bool) -> Self {
        Self {
            member,
            map_size,
            margin,
            me: me,
        }
    }
}

impl Component for Player {
    fn render(&self) -> impl IntoElement {
        if self.me {
            base_marker(
                self.member.x,
                self.member.y,
                12.0,
                self.margin,
                self.map_size,
                Align::Center,
            )
            .corner_radius(CornerRadius::new_all(1000.0))
            .background(Color::from_hex("#f3c86d").unwrap())
            .border(
                Border::new()
                    .fill(Color::from_hex("#000000d0").unwrap())
                    .width(2.0),
            )
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
        } else {
            base_marker(
                self.member.x,
                self.member.y,
                9.0,
                self.margin,
                self.map_size,
                Align::Center,
            )
            .corner_radius(CornerRadius::new_all(1000.0))
            .background(Color::from_hex("#aaee32").unwrap())
            .border(
                Border::new()
                    .fill(Color::from_hex("#000000d0").unwrap())
                    .width(1.5),
            )
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            .child(
                rect().offset_y(-10.0).child(
                    label()
                        .font_family("Roboto Condensed")
                        .font_weight(FontWeight::SEMI_BOLD)
                        .color(Color::from_hex("#aaee32").unwrap())
                        .font_size(6.0)
                        .max_lines(1)
                        .text(format!("{}", self.member.name)),
                ),
            )
        }
        // .child(
        //     rect().offset_y(4.0).child(
        //         label()
        //             .width(Size::px(500.0))
        //             .text_align(TextAlign::Center)
        //             // Magic numbers :)
        //             //.font_size(8.864 / self.zoom + 2.446)
        //             .font_size(12.0)
        //             .font_family("PermanentMarker")
        //             .color(Color::from_hex("#191919e6").unwrap())
        //             .text(format!(
        //                 "{}",
        //                 self.team_members
        //                     .iter()
        //                     .find(|member| member.steam_id == self.marker.steam_id)
        //                     .unwrap()
        //                     .name
        //             )),
        //     ),
        // )
    }
}
