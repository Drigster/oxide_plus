use freya::{prelude::*, radio::use_radio};

use crate::{Data, DataChannel, colors, components::CachedImage};

#[derive(PartialEq)]
pub struct UserCard {}

impl UserCard {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for UserCard {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio::<Data, DataChannel>(DataChannel::UserDataUpdate);
        let user_data = radio.slice_current(|s| &s.user_data);

        let steam_id = user_data.read().steam_id.clone();

        if steam_id.is_none() {
            return rect().into();
        }

        let steam_id: u64 = steam_id.unwrap().parse().unwrap_or(0);

        let team_members = radio.slice(DataChannel::TeamMemberUpdate(steam_id), |s| {
            &s.team_info.members
        });
        let team_member = team_members.read().get(&steam_id).cloned();

        if team_member.is_none() {
            return rect().into();
        }

        let team_member = team_member.unwrap();

        rect()
            .direction(Direction::Horizontal)
            .padding(8.0)
            .spacing(8.0)
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            .children([
                rect()
                    .main_align(Alignment::Center)
                    .children([
                        label()
                            .font_size(12.0)
                            .color(Color::from_hex(colors::TEXT).unwrap())
                            .text(team_member.name)
                            .into(),
                        label()
                            .font_size(10.0)
                            .color(Color::from_hex("#8D8D8D").unwrap())
                            .text(format!("{}", steam_id))
                            .into(),
                    ])
                    .into(),
                rect()
                    .corner_radius(CornerRadius::new_all(1000.0))
                    .height(Size::px(32.0))
                    .width(Size::px(32.0))
                    .overflow(Overflow::Clip)
                    .maybe_child(if let Some(profile_icon) = &team_member.profile_icon {
                        Some(CachedImage::new(profile_icon.clone()))
                    } else {
                        None
                    })
                    .into(),
            ])
    }
}
