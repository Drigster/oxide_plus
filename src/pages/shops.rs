use freya::prelude::*;
use freya_radio::prelude::*;
use rustplus_rs::AppMarkerType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    app::{Data, DataChannel},
    components::CachedImage,
};

static ITEM_DATA: &'static [u8] = include_bytes!("./../assets/item_data.json");

#[derive(PartialEq)]
enum OrderPartType {
    Stock(i32),
    Price,
}

#[derive(PartialEq)]
struct OrderPart {
    icon_url: String,
    order_part_type: OrderPartType,
    amount: i32,
}

impl OrderPart {
    fn new(icon_url: String, order_part_type: OrderPartType, amount: i32) -> Self {
        Self {
            icon_url: icon_url,
            order_part_type,
            amount,
        }
    }
}

impl Render for OrderPart {
    fn render(&self) -> Element {
        rect()
            .width(Size::percent(50.0))
            .height(Size::Fill)
            .spacing(4.0)
            .direction(Direction::Horizontal)
            .cross_align(Alignment::Center)
            .child(
                CachedImage::new(self.icon_url.clone())
                    .width(Size::px(48.0))
                    .height(Size::px(48.0)),
            )
            .maybe_child(
                if let OrderPartType::Stock(quantity) = self.order_part_type {
                    Some(
                        rect()
                            .width(Size::px(48.0))
                            .height(Size::px(48.0))
                            .position(Position::new_absolute())
                            .layer(5)
                            .main_align(Alignment::End)
                            .cross_align(Alignment::End)
                            .font_size(12.0)
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .color(Color::from_hex("#E4DAD1").unwrap()), // .child(format!("x{}", quantity)),
                    )
                } else {
                    None
                },
            )
            .child(
                rect()
                    .width(Size::Fill)
                    .height(Size::Fill)
                    .padding(4.0)
                    .spacing(4.0)
                    .children([
                        // label()
                        //     .color(Color::from_hex("#818181").unwrap())
                        //     .font_size(12.0)
                        //     .font_weight(FontWeight::BOLD)
                        //     .text(if matches!(self.order_part_type, OrderPartType::Stock(_)) {
                        //         "STOCK"
                        //     } else {
                        //         "COST"
                        //     })
                        //     .into(),
                        // label()
                        //     .color(Color::from_hex("#E4DAD1").unwrap())
                        //     .font_size(16.0)
                        //     .font_weight(FontWeight::BOLD)
                        //     .text(self.amount.to_string())
                        //     .into(),
                    ]),
            )
            .into()
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
impl Render for Shops {
    fn render(&self) -> Element {
        let map_markers_binding = use_radio::<Data, DataChannel>(DataChannel::MapMarkersUpdate);
        let map_markers = map_markers_binding
            .read()
            .map_markers
            .clone()
            .expect("Map markers should be loaded");

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
                    .children([Input::new().width(Size::Fill).into()])
                    .into(),
                ScrollView::new()
                    .width(Size::Fill)
                    .height(Size::Fill)
                    .spacing(8.0)
                    .children_iter(map_markers.markers.iter().filter_map(|marker| {
                        if AppMarkerType::try_from(marker.marker_type).unwrap_or_default()
                            != AppMarkerType::VendingMachine
                        {
                            println!("Marker type: {:?}", marker.marker_type);
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
                                        .color(Color::from_hex("#E4DAD1").unwrap())
                                        .font_size(15.0)
                                        .font_weight(FontWeight::BOLD)
                                        .children([
                                            label()
                                                .text(
                                                    marker
                                                        .name
                                                        .clone()
                                                        .unwrap_or(marker.id.to_string()),
                                                )
                                                .into(),
                                            label().text("G20").into(),
                                        ])
                                        .into(),
                                    rect()
                                        .width(Size::percent(50.0))
                                        .padding(8.0)
                                        .spacing(8.0)
                                        .children_iter(marker.sell_orders.iter().filter_map(
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
                                                        .children([
                                                            OrderPart::new(
                                                                selling_item.icon_url.clone(),
                                                                OrderPartType::Stock(
                                                                    sell_order.quantity,
                                                                ),
                                                                sell_order.amount_in_stock,
                                                            )
                                                            .into(),
                                                            OrderPart::new(
                                                                buying_item.icon_url.clone(),
                                                                OrderPartType::Price,
                                                                sell_order.cost_per_item,
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
            .into()
    }
}
