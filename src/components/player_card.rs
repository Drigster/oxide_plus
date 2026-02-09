use freya::{prelude::*, radio::use_radio};

use crate::{BORDER_COLOR, Data, DataChannel, ICON_COLOR, TEXT_COLOR, components::CachedImage};

#[derive(PartialEq)]
pub struct PlayerCard {
    pub username: String,
    pub steam_id: u64,
    pub is_online: bool,
}

impl PlayerCard {
    pub fn new(username: String, steam_id: u64, is_online: bool) -> Self {
        Self {
            username,
            steam_id,
            is_online,
        }
    }
}

impl Component for PlayerCard {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio::<Data, DataChannel>(DataChannel::TeamMemberUpdate(self.steam_id));
        let steam_profile = radio.read().team_info.members.get(&self.steam_id).cloned();

        rect()
            .width(Size::Fill)
            .height(Size::px(48.0))
            .direction(Direction::Horizontal)
            .padding(8.0)
            .spacing(4.0)
            .cross_align(Alignment::Center)
            .background(Color::from_hex("#181818").unwrap())
            .corner_radius(CornerRadius::new_all(8.0))
            .children([
                rect()
                    .corner_radius(CornerRadius::new_all(1000.0))
                    .height(Size::px(32.0))
                    .width(Size::px(32.0))
                    .overflow(Overflow::Clip)
                    .border(
                        Border::new()
                            .width(1.0)
                            .fill(Color::from_hex(BORDER_COLOR).unwrap())
                            .alignment(BorderAlignment::Outer),
                    )
                    .child(if let Some(steam_profile) = &steam_profile {
                        if let Some(profile_icon) = &steam_profile.profile_icon {
                            CachedImage::new(profile_icon.clone()).into_element()
                        } else {
                            rect().into_element()
                        }
                    } else {
                        rect().into_element()
                    })
                    .into(),
                rect()
                    .main_align(Alignment::SpaceBetween)
                    .children([
                        label()
                            .font_size(16.0)
                            .color(Color::from_hex(TEXT_COLOR).unwrap())
                            .text(if let Some(steam_profile) = &steam_profile {
                                steam_profile.name.clone()
                            } else {
                                self.username.clone()
                            })
                            .into(),
                        rect()
                            .cross_align(Alignment::Center)
                            .direction(Direction::Horizontal)
                            .spacing(2.0)
                            .child(
                                rect()
                                    .width(Size::px(5.0))
                                    .height(Size::px(5.0))
                                    .corner_radius(1000.0)
                                    .background(
                                        Color::from_hex(if self.is_online {
                                            "#aaee32"
                                        } else {
                                            ICON_COLOR
                                        })
                                        .unwrap(),
                                    ),
                            )
                            .child(
                                label()
                                    .font_size(8.0)
                                    .font_weight(FontWeight::BOLD)
                                    .color(
                                        Color::from_hex(if self.is_online {
                                            "#aaee32"
                                        } else {
                                            ICON_COLOR
                                        })
                                        .unwrap(),
                                    )
                                    .text(if self.is_online { "ONLINE" } else { "OFFLINE" }),
                            )
                            .into(),
                    ])
                    .into(),
            ])
    }
}
