use chrono::prelude::*;
use freya::{prelude::*, radio::use_radio};
use timeago::Formatter;

use crate::{
    Data, DataChannel, colors,
    components::{CachedImage, PlayerCard},
};

#[derive(PartialEq)]
pub struct Info {}
impl Component for Info {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio::<Data, DataChannel>(DataChannel::InfoStateUpdate);

        let binding = radio.slice_current(|s| &s.info_state);
        let info_state = binding.read();
        let team_members = radio.slice(DataChannel::TeamMembersUpdate, |s| &s.team_info.members);

        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .padding(8.0)
            .spacing(8.0)
            .children([
                rect()
                    .width(Size::Fill)
                    .height(Size::px(420.0))
                    .child(
                        rect()
                            .width(Size::Fill)
                            .height(Size::Fill)
                            .corner_radius(8.0)
                            .main_align(Alignment::Center)
                            .cross_align(Alignment::Center)
                            .overflow(Overflow::Clip)
                            .child(
                                rect()
                                    .width(Size::Fill)
                                    .height(Size::Fill)
                                    .position(Position::new_absolute())
                                    .background(Color::from_hex("#00000080").unwrap()),
                            )
                            .maybe_child(if let Some(header_image) = &info_state.header_image {
                                Some(
                                    CachedImage::new(header_image.to_string())
                                        .width(Size::Fill)
                                        .height(Size::Fill)
                                        .aspect_ratio(AspectRatio::Max),
                                )
                            } else {
                                None
                            }),
                    )
                    .child(
                        rect()
                            .position(Position::new_absolute())
                            .padding(8.0)
                            .spacing(8.0)
                            .layer(Layer::Relative(2))
                            .background(Color::from_hex(colors::BACKGROUND).unwrap().with_a(191))
                            .corner_radius(CornerRadius {
                                top_left: 8.0,
                                top_right: 0.0,
                                bottom_left: 0.0,
                                bottom_right: 8.0,
                                smoothing: 0.0,
                            })
                            .children([
                                InfoCard::new(
                                    "00:00".to_string(),
                                    "Server Time".to_string(),
                                    Bytes::from_static(include_bytes!("../assets/MDI/clock.svg")),
                                )
                                .into(),
                                InfoCard::new(
                                    info_state
                                        .map
                                        .clone()
                                        .unwrap_or("Retrieving...".to_string()),
                                    if let Some(map_size) = &info_state.map_size {
                                        format!("Map {}K", *map_size as f32 / 1000.0)
                                    } else {
                                        "Retrieving...".to_string()
                                    },
                                    Bytes::from_static(include_bytes!("../assets/MDI/map.svg")),
                                )
                                .into(),
                            ]),
                    )
                    .into(),
                rect()
                    .width(Size::Fill)
                    .height(Size::Fill)
                    .corner_radius(8.0)
                    .spacing(8.0)
                    .children([
                        rect()
                            .width(Size::Fill)
                            .height(Size::px(70.0))
                            .spacing(8.0)
                            .direction(Direction::Horizontal)
                            .content(Content::Flex)
                            .children([
                                rect()
                                    .width(Size::flex(1.0))
                                    .height(Size::Fill)
                                    .padding(8.0)
                                    .background(Color::from_hex(colors::BACKGROUND).unwrap())
                                    .corner_radius(8.0)
                                    .main_align(Alignment::SpaceBetween)
                                    .children([
                                        label()
                                            .max_width(Size::percent(90.0))
                                            .font_family("WDXL Lubrifont")
                                            .font_size(24.0)
                                            .font_weight(FontWeight::BOLD)
                                            .color(Color::from_hex(colors::TEXT).unwrap())
                                            .max_lines(1)
                                            .text_overflow(TextOverflow::Custom("...".to_string()))
                                            .text(
                                                info_state
                                                    .name
                                                    .clone()
                                                    .unwrap_or("Retrieving...".to_string()),
                                            )
                                            .into(),
                                        label()
                                            .font_family("WDXL Lubrifont")
                                            .font_size(16.0)
                                            .font_weight(FontWeight::BOLD)
                                            .color(Color::from_hex(colors::ICON).unwrap())
                                            .text(if let Some(wipe_time) = &info_state.wipe_time {
                                                let formatter = Formatter::new();
                                                let timestamp = DateTime::from_timestamp(
                                                    (*wipe_time).into(),
                                                    0,
                                                )
                                                .unwrap();
                                                let now = Utc::now();
                                                formatter.convert_chrono(timestamp, now)
                                            } else {
                                                "Retrieving...".to_string()
                                            })
                                            .into(),
                                    ])
                                    .into(),
                                rect()
                                    .width(Size::flex(1.0))
                                    .height(Size::Fill)
                                    .padding(8.0)
                                    .background(Color::from_hex(colors::BACKGROUND).unwrap())
                                    .corner_radius(8.0)
                                    .main_align(Alignment::SpaceBetween)
                                    .cross_align(Alignment::End)
                                    .children([
                                        rect()
                                            .position(
                                                Position::new_absolute().top(-55.0).left(-55.0),
                                            )
                                            .background(
                                                Color::from_hex(colors::BACKGROUND_DARK).unwrap(),
                                            )
                                            .corner_radius(1000.0)
                                            .child(
                                                rect()
                                                    .margin(8.0)
                                                    .width(Size::px(70.0))
                                                    .height(Size::px(70.0))
                                                    .background(
                                                        Color::from_hex(colors::BACKGROUND)
                                                            .unwrap(),
                                                    )
                                                    .corner_radius(1000.0)
                                                    .overflow(Overflow::Clip)
                                                    .maybe_child(
                                                        if let Some(logo_image) =
                                                            &info_state.logo_image
                                                        {
                                                            Some(
                                                                CachedImage::new(
                                                                    logo_image.to_string(),
                                                                )
                                                                .width(Size::px(70.0))
                                                                .height(Size::px(70.0)),
                                                            )
                                                        } else {
                                                            None
                                                        },
                                                    ),
                                            )
                                            .into(),
                                        label()
                                            .max_width(Size::percent(90.0))
                                            .font_family("WDXL Lubrifont")
                                            .font_size(24.0)
                                            .font_weight(FontWeight::BOLD)
                                            .color(Color::from_hex(colors::TEXT).unwrap())
                                            .max_lines(1)
                                            .text_overflow(TextOverflow::Custom("...".to_string()))
                                            .text(if let Some(players) = &info_state.players {
                                                format!(
                                                    "{}/{} Players",
                                                    players,
                                                    info_state.max_players.unwrap_or(0)
                                                )
                                            } else {
                                                "Retrieving...".to_string()
                                            })
                                            .into(),
                                        label()
                                            .font_family("WDXL Lubrifont")
                                            .font_size(16.0)
                                            .font_weight(FontWeight::BOLD)
                                            .color(Color::from_hex(colors::ICON).unwrap())
                                            .text(
                                                if let Some(queued_players) =
                                                    &info_state.queued_players
                                                {
                                                    format!("{} Queued", queued_players)
                                                } else {
                                                    "Retrieving...".to_string()
                                                },
                                            )
                                            .into(),
                                    ])
                                    .into(),
                            ])
                            .into(),
                        rect()
                            .width(Size::Fill)
                            .height(Size::Fill)
                            .spacing(8.0)
                            .direction(Direction::Horizontal)
                            .content(Content::Flex)
                            .children([
                                rect()
                                    .width(Size::flex(1.0))
                                    .height(Size::Fill)
                                    .background(Color::from_hex(colors::BACKGROUND).unwrap())
                                    .corner_radius(8.0)
                                    .child(if team_members.read().len() > 0 {
                                        ScrollView::new()
                                            .child(
                                                rect()
                                                    .padding(8.0)
                                                    .spacing(8.0)
                                                    .direction(Direction::Horizontal)
                                                    .content(Content::Flex)
                                                    .children([
                                                        rect()
                                                            .width(Size::flex(1.0))
                                                            .spacing(8.0)
                                                            .children(
                                                                team_members
                                                                    .read()
                                                                    .values()
                                                                    .into_iter()
                                                                    .enumerate()
                                                                    .filter_map(|(i, member)| {
                                                                        if i % 2 == 1 {
                                                                            None
                                                                        } else {
                                                                            Some(
                                                                                PlayerCard::new(
                                                                                    member
                                                                                        .name
                                                                                        .clone(),
                                                                                    member.steam_id,
                                                                                    member
                                                                                        .is_online,
                                                                                )
                                                                                .into(),
                                                                            )
                                                                        }
                                                                    })
                                                                    .collect::<Vec<Element>>(),
                                                            )
                                                            .into(),
                                                        rect()
                                                            .width(Size::flex(1.0))
                                                            .spacing(8.0)
                                                            .children(
                                                                team_members
                                                                    .read()
                                                                    .values()
                                                                    .into_iter()
                                                                    .enumerate()
                                                                    .filter_map(|(i, member)| {
                                                                        if i % 2 == 0 {
                                                                            None
                                                                        } else {
                                                                            Some(
                                                                                PlayerCard::new(
                                                                                    member
                                                                                        .name
                                                                                        .clone(),
                                                                                    member.steam_id,
                                                                                    member
                                                                                        .is_online,
                                                                                )
                                                                                .into(),
                                                                            )
                                                                        }
                                                                    })
                                                                    .collect::<Vec<Element>>(),
                                                            )
                                                            .into(),
                                                    ]),
                                            )
                                            .into_element()
                                    } else {
                                        rect()
                                            .expanded()
                                            .center()
                                            .child(
                                                label()
                                                    .color(Color::from_hex(colors::TEXT).unwrap())
                                                    .text("Retrieving..."),
                                            )
                                            .into_element()
                                    })
                                    .into(),
                                rect()
                                    .width(Size::flex(1.0))
                                    .height(Size::Fill)
                                    .background(Color::from_hex(colors::BACKGROUND).unwrap())
                                    .corner_radius(8.0)
                                    .child(
                                        if let Some(selected_server) = &radio.read().selected_server
                                        {
                                            ScrollView::new()
                                                .child(
                                                    label()
                                                        .font_size(16.0)
                                                        .font_weight(FontWeight::BOLD)
                                                        .color(
                                                            Color::from_hex(colors::TEXT).unwrap(),
                                                        )
                                                        .margin(8.0)
                                                        .text(selected_server.desc.clone()),
                                                )
                                                .into_element()
                                        } else {
                                            rect()
                                                .expanded()
                                                .center()
                                                .child(
                                                    label()
                                                        .color(
                                                            Color::from_hex(colors::TEXT).unwrap(),
                                                        )
                                                        .text("Retrieving..."),
                                                )
                                                .into_element()
                                        },
                                    )
                                    .into(),
                            ])
                            .into(),
                    ])
                    .into(),
            ])
    }
}

#[derive(PartialEq)]
struct InfoCard {
    title: String,
    sub_title: String,
    icon: Bytes,
}

impl InfoCard {
    fn new(title: String, sub_title: String, icon: Bytes) -> Self {
        Self {
            title,
            sub_title,
            icon,
        }
    }
}

impl Component for InfoCard {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::px(150.0))
            .spacing(6.0)
            .direction(Direction::Horizontal)
            .cross_align(Alignment::Center)
            .children([
                rect()
                    .width(Size::px(36.0))
                    .height(Size::px(36.0))
                    .padding(6.0)
                    .background(Color::from_hex(colors::BACKGROUND).unwrap())
                    .corner_radius(4.0)
                    .child(
                        svg(self.icon.clone())
                            .width(Size::Fill)
                            .height(Size::Fill)
                            .color(Color::from_hex(colors::TEXT).unwrap()),
                    )
                    .into(),
                rect()
                    .spacing(2.0)
                    .children([
                        label()
                            .font_size(12.0)
                            .font_weight(FontWeight::BOLD)
                            .color(Color::from_hex(colors::TEXT).unwrap())
                            .text(self.title.clone())
                            .into(),
                        label()
                            .font_size(10.0)
                            .font_weight(FontWeight::BOLD)
                            .color(Color::from_hex("#6b655f").unwrap())
                            .text(self.sub_title.clone())
                            .into(),
                    ])
                    .into(),
            ])
    }
}
