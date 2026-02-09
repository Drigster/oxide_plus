use freya::{prelude::*, radio::use_radio};

use crate::{
    DataChannel, components::markers::{Align, base_marker}
};

#[derive(PartialEq)]
pub struct Player {
    pub member_id: u64,
    pub map_size: f32,
    pub margin: f32,
}

impl Player {
    pub fn new(
        member_id: u64,
        map_size: f32,
        margin: f32,
    ) -> Self {
        Self {
            member_id: member_id,
            map_size,
            margin,
        }
    }
}

impl Component for Player {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio::<crate::Data, crate::DataChannel>(crate::DataChannel::TeamMemberUpdate(self.member_id));
        let members = radio.slice_current(|s| &s.team_info.members);
        let member = members.read().get(&self.member_id).cloned();

        let user_data = radio.slice(DataChannel::UserDataUpdate, |s| &s.user_data);

        if member.is_none() {
            return rect().into();
        }

        let steam_id: u64 = user_data.read().steam_id.clone().unwrap_or("0".to_string()).parse().unwrap_or(0);
        let me = steam_id == self.member_id;
        let member = member.unwrap();

        if me {
            base_marker(
                member.x,
                member.y,
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
                member.x,
                member.y,
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
                        .text(format!("{}", member.name)),
                ),
            )
        }
    }
}
