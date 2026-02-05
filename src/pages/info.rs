use chrono::prelude::*;
use freya::{prelude::*, radio::use_radio};
use timeago::Formatter;

use crate::{
    Data, DataChannel, ICON_COLOR, TEXT_COLOR,
    components::{CachedImage, PlayerCard},
    pages::team,
};

#[derive(PartialEq)]
pub struct Info {}
impl Component for Info {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio::<Data, DataChannel>(DataChannel::InfoStateUpdate);
        let team_info_binding = use_radio::<Data, DataChannel>(DataChannel::TeamInfoUpdate);

        let info_state = radio.read().info_state.clone();
        if info_state.is_none() {
            return rect();
        }
        let info_state = info_state.unwrap();

        let team_info = team_info_binding.read().team_info.clone();
        if team_info.is_none() {
            return rect();
        }
        let team_info = team_info.unwrap();

        let formatter = Formatter::new();
        let timestamp = DateTime::from_timestamp(info_state.wipe_time.into(), 0).unwrap();
        let now = Utc::now();
        let wipe_time = formatter.convert_chrono(timestamp, now);

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
                            .maybe_child(if !info_state.header_image.is_empty() {
                                Some(
                                    CachedImage::new(info_state.header_image)
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
                            .background(Color::from_hex("#1D1D1B").unwrap().with_a(191))
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
                                    info_state.map,
                                    format!("Map {}K", info_state.map_size as f32 / 1000.0),
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
                                    .background(Color::from_hex("#1D1D1B").unwrap())
                                    .corner_radius(8.0)
                                    .main_align(Alignment::SpaceBetween)
                                    .children([
                                        label()
                                            .max_width(Size::percent(90.0))
                                            .font_family("WDXL Lubrifont")
                                            .font_size(24.0)
                                            .font_weight(FontWeight::BOLD)
                                            .color(Color::from_hex(TEXT_COLOR).unwrap())
                                            .max_lines(1)
                                            .text_overflow(TextOverflow::Custom("...".to_string()))
                                            .text(info_state.name)
                                            .into(),
                                        label()
                                            .font_family("WDXL Lubrifont")
                                            .font_size(16.0)
                                            .font_weight(FontWeight::BOLD)
                                            .color(Color::from_hex(ICON_COLOR).unwrap())
                                            .text(format!("Wiped {}", wipe_time))
                                            .into(),
                                    ])
                                    .into(),
                                rect()
                                    .width(Size::flex(1.0))
                                    .height(Size::Fill)
                                    .padding(8.0)
                                    .background(Color::from_hex("#1D1D1B").unwrap())
                                    .corner_radius(8.0)
                                    .main_align(Alignment::SpaceBetween)
                                    .cross_align(Alignment::End)
                                    .children([
                                        rect()
                                            .position(
                                                Position::new_absolute().top(-55.0).left(-55.0),
                                            )
                                            .background(Color::from_hex("#0e0e0d").unwrap())
                                            .corner_radius(1000.0)
                                            .child(
                                                rect()
                                                    .margin(8.0)
                                                    .width(Size::px(70.0))
                                                    .height(Size::px(70.0))
                                                    .background(Color::from_hex("#1D1D1B").unwrap())
                                                    .corner_radius(1000.0)
                                                    .overflow(Overflow::Clip)
                                                    .maybe_child(
                                                        if !info_state.logo_image.is_empty() {
                                                            Some(
                                                                CachedImage::new(
                                                                    info_state.logo_image,
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
                                            .color(Color::from_hex(TEXT_COLOR).unwrap())
                                            .max_lines(1)
                                            .text_overflow(TextOverflow::Custom("...".to_string()))
                                            .text(format!(
                                                "{}/{} Players",
                                                info_state.players, info_state.max_players
                                            ))
                                            .into(),
                                        label()
                                            .font_family("WDXL Lubrifont")
                                            .font_size(16.0)
                                            .font_weight(FontWeight::BOLD)
                                            .color(Color::from_hex(ICON_COLOR).unwrap())
                                            .text(format!("{} Queued", info_state.queued_players))
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
                                    .background(Color::from_hex("#1D1D1B").unwrap())
                                    .corner_radius(8.0)
                                    .child(
                                        ScrollView::new().child(
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
                                                            team_info
                                                                .members
                                                                .iter()
                                                                .enumerate()
                                                                .filter_map(|(i, member)| {
                                                                    if i % 2 == 1 {
                                                                        None
                                                                    } else {
                                                                        Some(
                                                                            PlayerCard::new(
                                                                                member.name.clone(),
                                                                                member.steam_id,
                                                                                member.is_online,
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
                                                            team_info
                                                                .members
                                                                .iter()
                                                                .enumerate()
                                                                .filter_map(|(i, member)| {
                                                                    if i % 2 == 0 {
                                                                        None
                                                                    } else {
                                                                        Some(
                                                                            PlayerCard::new(
                                                                                member.name.clone(),
                                                                                member.steam_id,
                                                                                member.is_online,
                                                                            )
                                                                            .into(),
                                                                        )
                                                                    }
                                                                })
                                                                .collect::<Vec<Element>>(),
                                                        )
                                                        .into(),
                                                ]),
                                        ),
                                    )
                                    .into(),
                                rect()
                                    .width(Size::flex(1.0))
                                    .height(Size::Fill)
                                    .background(Color::from_hex("#1D1D1B").unwrap())
                                    .corner_radius(8.0)
                                    .child(
                                        ScrollView::new().child(
                                            label()
                                                .font_size(16.0)
                                                .font_weight(FontWeight::BOLD)
                                                .color(Color::from_hex(TEXT_COLOR).unwrap())
                                                .margin(8.0)
                                                .text(
                                                    radio
                                                        .read()
                                                        .selected_server
                                                        .clone()
                                                        .unwrap()
                                                        .desc,
                                                ),
                                        ),
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
                    .background(Color::from_hex("#1D1D1B").unwrap())
                    .corner_radius(4.0)
                    .child(
                        svg(self.icon.clone())
                            .width(Size::Fill)
                            .height(Size::Fill)
                            .color(Color::from_hex(TEXT_COLOR).unwrap()),
                    )
                    .into(),
                rect()
                    .spacing(2.0)
                    .children([
                        label()
                            .font_size(12.0)
                            .font_weight(FontWeight::BOLD)
                            .color(Color::from_hex(TEXT_COLOR).unwrap())
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
