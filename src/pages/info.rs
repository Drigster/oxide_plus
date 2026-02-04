use chrono::prelude::*;
use freya::{prelude::*, radio::use_radio};
use timeago::Formatter;

use crate::{Data, DataChannel, TEXT_COLOR, components::CachedImage};

#[derive(PartialEq)]
pub struct Info {}
impl Component for Info {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio::<Data, DataChannel>(DataChannel::InfoStateUpdate);

        let info_state = radio.read().info_state.clone();
        if info_state.is_none() {
            return rect();
        }
        let info_state = info_state.unwrap();

        let formatter = Formatter::new();
        let timestamp = DateTime::from_timestamp(info_state.wipe_time.into(), 0).unwrap();
        let now = Utc::now();
        let wipe_time = formatter.convert_chrono(timestamp, now);

        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            .child(
                rect()
                    .corner_radius(8.0)
                    .background(Color::from_hex("#1D1D1B").unwrap())
                    .maybe_child(if !info_state.header_image.is_empty() {
                        Some(
                            CachedImage::new(info_state.header_image)
                                .width(Size::px(400.0))
                                .height(Size::px(200.0)),
                        )
                    } else {
                        None
                    })
                    .children([
                        rect()
                            .width(Size::percent(100.0))
                            .padding(Gaps::from((8.0, 16.0)))
                            .spacing(8.0)
                            .cross_align(Alignment::Center)
                            .children([
                                label()
                                    .font_size(24.0)
                                    .font_weight(FontWeight::BOLD)
                                    .color(Color::from_hex(TEXT_COLOR).unwrap())
                                    .text(info_state.name)
                                    .into(),
                                label()
                                    .font_size(16.0)
                                    .font_weight(FontWeight::BOLD)
                                    .color(Color::from_hex(TEXT_COLOR).unwrap())
                                    .text(format!(
                                        "{}/{} Players",
                                        info_state.players, info_state.max_players
                                    ))
                                    .into(),
                            ])
                            .into(),
                        rect()
                            .width(Size::percent(100.0))
                            .padding(Gaps::from((8.0, 16.0)))
                            .spacing(8.0)
                            .cross_align(Alignment::Center)
                            .children([
                                rect()
                                    .direction(Direction::Horizontal)
                                    .spacing(8.0)
                                    .content(Content::Flex)
                                    .children([
                                        rect()
                                            .width(Size::flex(1.0))
                                            .padding(4.0)
                                            .spacing(6.0)
                                            .direction(Direction::Horizontal)
                                            .children([
                                                rect()
                                                    .width(Size::px(32.0))
                                                    .height(Size::px(32.0))
                                                    .corner_radius(4.0)
                                                    .background(Color::from_hex("#5D5D5D").unwrap())
                                                    .into(),
                                                rect()
                                                    .spacing(2.0)
                                                    .children([
                                                        label()
                                                            .font_size(12.0)
                                                            .font_weight(FontWeight::BOLD)
                                                            .color(
                                                                Color::from_hex(TEXT_COLOR)
                                                                    .unwrap(),
                                                            )
                                                            .text(
                                                                info_state
                                                                    .queued_players
                                                                    .to_string(),
                                                            )
                                                            .into(),
                                                        label()
                                                            .font_size(10.0)
                                                            .font_weight(FontWeight::BOLD)
                                                            .color(
                                                                Color::from_hex("#605B55").unwrap(),
                                                            )
                                                            .text("Queued Players")
                                                            .into(),
                                                    ])
                                                    .into(),
                                            ])
                                            .into(),
                                        rect()
                                            .width(Size::flex(1.0))
                                            .padding(4.0)
                                            .spacing(6.0)
                                            .direction(Direction::Horizontal)
                                            .children([
                                                rect()
                                                    .width(Size::px(32.0))
                                                    .height(Size::px(32.0))
                                                    .corner_radius(4.0)
                                                    .background(Color::from_hex("#5D5D5D").unwrap())
                                                    .into(),
                                                rect()
                                                    .spacing(2.0)
                                                    .children([
                                                        label()
                                                            .font_size(12.0)
                                                            .font_weight(FontWeight::BOLD)
                                                            .color(
                                                                Color::from_hex(TEXT_COLOR)
                                                                    .unwrap(),
                                                            )
                                                            .text("00:00")
                                                            .into(),
                                                        label()
                                                            .font_size(10.0)
                                                            .font_weight(FontWeight::BOLD)
                                                            .color(
                                                                Color::from_hex("#605B55").unwrap(),
                                                            )
                                                            .text("Server Time")
                                                            .into(),
                                                    ])
                                                    .into(),
                                            ])
                                            .into(),
                                    ])
                                    .into(),
                                rect()
                                    .direction(Direction::Horizontal)
                                    .spacing(8.0)
                                    .content(Content::Flex)
                                    .children([
                                        rect()
                                            .width(Size::flex(1.0))
                                            .padding(4.0)
                                            .spacing(6.0)
                                            .direction(Direction::Horizontal)
                                            .children([
                                                rect()
                                                    .width(Size::px(32.0))
                                                    .height(Size::px(32.0))
                                                    .corner_radius(4.0)
                                                    .background(Color::from_hex("#5D5D5D").unwrap())
                                                    .into(),
                                                rect()
                                                    .spacing(2.0)
                                                    .children([
                                                        label()
                                                            .font_size(12.0)
                                                            .font_weight(FontWeight::BOLD)
                                                            .color(
                                                                Color::from_hex(TEXT_COLOR)
                                                                    .unwrap(),
                                                            )
                                                            .text(wipe_time)
                                                            .into(),
                                                        label()
                                                            .font_size(10.0)
                                                            .font_weight(FontWeight::BOLD)
                                                            .color(
                                                                Color::from_hex("#605B55").unwrap(),
                                                            )
                                                            .text("Last Wiped")
                                                            .into(),
                                                    ])
                                                    .into(),
                                            ])
                                            .into(),
                                        rect()
                                            .width(Size::flex(1.0))
                                            .padding(4.0)
                                            .spacing(6.0)
                                            .direction(Direction::Horizontal)
                                            .children([
                                                rect()
                                                    .width(Size::px(32.0))
                                                    .height(Size::px(32.0))
                                                    .corner_radius(4.0)
                                                    .background(Color::from_hex("#5D5D5D").unwrap())
                                                    .into(),
                                                rect()
                                                    .spacing(2.0)
                                                    .children([
                                                        label()
                                                            .font_size(12.0)
                                                            .font_weight(FontWeight::BOLD)
                                                            .color(
                                                                Color::from_hex(TEXT_COLOR)
                                                                    .unwrap(),
                                                            )
                                                            .text(info_state.map)
                                                            .into(),
                                                        label()
                                                            .font_size(10.0)
                                                            .font_weight(FontWeight::BOLD)
                                                            .color(
                                                                Color::from_hex("#605B55").unwrap(),
                                                            )
                                                            .text(format!(
                                                                "Map {}",
                                                                info_state.map_size
                                                            ))
                                                            .into(),
                                                    ])
                                                    .into(),
                                            ])
                                            .into(),
                                    ])
                                    .into(),
                            ])
                            .into(),
                    ]),
            )
    }
}
