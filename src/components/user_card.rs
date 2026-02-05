use freya::{prelude::*, radio::use_radio};

use crate::{Data, DataChannel, TEXT_COLOR, components::CachedImage, utils::Profile};

#[derive(PartialEq)]
pub struct UserCard {}

impl UserCard {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for UserCard {
    fn render(&self) -> impl IntoElement {
        let user_data_binding = use_radio::<Data, DataChannel>(DataChannel::UserDataUpdate);
        let user_data = user_data_binding
            .read()
            .user_data
            .clone()
            .expect("User data must be loaded");

        let steam_id: u64 = user_data.steam_id.parse().unwrap();

        let steam_profiles_binding =
            use_radio::<Data, DataChannel>(DataChannel::SteamProfileUpdate(steam_id.clone()));
        let steam_profile = steam_profiles_binding
            .read()
            .steam_profiles
            .get(&steam_id)
            .cloned()
            .unwrap_or(Profile {
                avatar_full: "".to_string(),
                avatar_medium: "".to_string(),
                avatar_icon: "".to_string(),
                username: "Unknown".to_string(),
            });

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
                            .color(Color::from_hex(TEXT_COLOR).unwrap())
                            .text(steam_profile.username)
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
                    .child(CachedImage::new(steam_profile.avatar_icon.clone()))
                    .into(),
            ])
    }
}
