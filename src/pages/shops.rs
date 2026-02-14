use freya::{prelude::*, radio::use_radio};
use rustplus_rs::AppMarkerType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Data, DataChannel, colors, components::CachedImage};

static ITEM_DATA: &'static [u8] = include_bytes!("./../assets/item_data.json");

#[derive(PartialEq)]
enum OrderPartType {
    Selling,
    Price,
}

#[derive(PartialEq)]
struct OrderPart {
    icon_url: String,
    order_part_type: OrderPartType,
    amount: i32,

    width: Size,
}

impl OrderPart {
    fn new(icon_url: String, order_part_type: OrderPartType, amount: i32) -> Self {
        Self {
            icon_url: icon_url,
            order_part_type,
            amount,
            width: Size::default(),
        }
    }

    fn width(mut self, width: Size) -> Self {
        self.width = width;
        self
    }
}

impl Component for OrderPart {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(self.width.clone())
            .height(Size::Fill)
            .spacing(4.0)
            .direction(Direction::Horizontal)
            .cross_align(Alignment::Center)
            .child(
                CachedImage::new(self.icon_url.clone())
                    .width(Size::px(48.0))
                    .height(Size::px(48.0)),
            )
            .child(
                rect()
                    .width(Size::Fill)
                    .height(Size::Fill)
                    .padding(4.0)
                    .spacing(4.0)
                    .children([
                        label()
                            .color(Color::from_hex("#818181").unwrap())
                            .font_size(12.0)
                            .font_weight(FontWeight::BOLD)
                            .text(if self.order_part_type == OrderPartType::Selling {
                                "SELLING"
                            } else {
                                "COST"
                            })
                            .into(),
                        label()
                            .color(Color::from_hex(colors::TEXT).unwrap())
                            .font_size(16.0)
                            .font_weight(FontWeight::BOLD)
                            .text(self.amount.to_string())
                            .into(),
                    ]),
            )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    id: i32,
    #[serde(rename = "shortName")]
    short_name: String,
    #[serde(rename = "displayName")]
    display_name: String,
    description: Option<String>,
    #[serde(rename = "iconUrl")]
    icon_url: String,
}

#[derive(PartialEq)]
pub struct Shops {}
impl Component for Shops {
    fn render(&self) -> impl IntoElement {
        let map_markers_binding = use_radio::<Data, DataChannel>(DataChannel::MapMarkersUpdate);
        let map_markers = map_markers_binding.read().map_markers.clone();

        let items: Vec<Item> =
            serde_json::from_slice::<Vec<Item>>(ITEM_DATA).expect("Item data should be loaded");

        let map: HashMap<i32, Item> = items.into_iter().map(|item| (item.id, item)).collect();

        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .padding(8.0)
            .children([
                rect()
                    .width(Size::Fill)
                    .height(Size::px(48.0))
                    .background(Color::from_hex("#222222").unwrap())
                    .corner_radius(8.0)
                    //.children([Input::new().width(Size::Fill).into()])
                    .into(),
                ScrollView::new()
                    .width(Size::Fill)
                    .height(Size::Fill)
                    .spacing(8.0)
                    .children(map_markers.markers.iter().filter_map(|marker| {
                        if marker.r#type() != AppMarkerType::VendingMachine {
                            println!("Marker type: {:?}", marker.r#type());
                            return None;
                        }
                        Some(
                            rect()
                                .width(Size::Fill)
                                .background(Color::from_hex("#222222").unwrap())
                                .corner_radius(8.0)
                                .children([
                                    rect()
                                        .width(Size::Fill)
                                        .padding(Gaps::from((4.0, 8.0)))
                                        .direction(Direction::Horizontal)
                                        .main_align(Alignment::SpaceBetween)
                                        .cross_align(Alignment::Center)
                                        .color(Color::from_hex(colors::TEXT).unwrap())
                                        .font_size(15.0)
                                        .font_weight(FontWeight::BOLD)
                                        .children([
                                            label().text(marker.name.clone()).into(),
                                            label().text("G20").into(),
                                        ])
                                        .into(),
                                    rect()
                                        .width(Size::percent(50.0))
                                        .padding(8.0)
                                        .spacing(8.0)
                                        .children(marker.sell_orders.iter().filter_map(
                                            |sell_order| {
                                                let selling_item = map.get(&sell_order.item_id);

                                                let buying_item = map.get(&sell_order.currency_id);

                                                if selling_item.is_none() || buying_item.is_none() {
                                                    return None;
                                                }

                                                let selling_item = selling_item.unwrap();
                                                let buying_item = buying_item.unwrap();

                                                Some(
                                                    rect()
                                                        .width(Size::Fill)
                                                        .height(Size::px(56.0))
                                                        .background(
                                                            Color::from_hex("#5D5D5D").unwrap(),
                                                        )
                                                        .corner_radius(CornerRadius::new_all(8.0))
                                                        .direction(Direction::Horizontal)
                                                        .padding(4.0)
                                                        .spacing(4.0)
                                                        .content(Content::Flex)
                                                        .children([
                                                            OrderPart::new(
                                                                selling_item.icon_url.clone(),
                                                                OrderPartType::Selling,
                                                                sell_order.quantity,
                                                            )
                                                            .width(Size::flex(1.0))
                                                            .into(),
                                                            OrderPart::new(
                                                                buying_item.icon_url.clone(),
                                                                OrderPartType::Price,
                                                                sell_order.cost_per_item,
                                                            )
                                                            .width(Size::flex(1.0))
                                                            .into(),
                                                            rect()
                                                                .width(Size::flex(1.0))
                                                                .height(Size::Fill)
                                                                .main_align(Alignment::Center)
                                                                .cross_align(Alignment::Center)
                                                                .child(
                                                                    label()
                                                                        .color(
                                                                            Color::from_hex(
                                                                                "#FFFFFF",
                                                                            )
                                                                            .unwrap(),
                                                                        )
                                                                        .font_size(14.0)
                                                                        .font_weight(
                                                                            FontWeight::EXTRA_BOLD,
                                                                        )
                                                                        .text(format!(
                                                                            "{} IN STOCK",
                                                                            sell_order
                                                                                .amount_in_stock
                                                                        )),
                                                                )
                                                                .into(),
                                                        ])
                                                        .into(),
                                                )
                                            },
                                        ))
                                        .into(),
                                ])
                                .into(),
                        )
                    }))
                    .into(),
            ])
    }
}
