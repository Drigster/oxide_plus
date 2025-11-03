use freya::prelude::*;
use freya_radio::prelude::*;
use rustplus_rs::AppMarkerType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::app::{Data, DataChannel};

static ITEM_DATA: &'static [u8] = include_bytes!("./../assets/item_data.json");

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    id: i32,
    #[serde(rename = "shortName")]
    short_name: Option<String>,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    description: Option<String>,
    #[serde(rename = "iconUrl")]
    icon_url: Option<String>,
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
                    .spacing(6.)
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
                                        .children_iter(marker.sell_orders.iter().map(
                                            |sell_order| {
                                                let selling_item = map
                                                    .get(&sell_order.item_id)
                                                    .cloned()
                                                    .unwrap_or(Item {
                                                        short_name: None,
                                                        id: sell_order.item_id,
                                                        display_name: None,
                                                        description: None,
                                                        icon_url: None,
                                                    });

                                                let selling_item_uri = selling_item.clone().icon_url;

                                                let buying_item = map
                                                    .get(&sell_order.currency_id)
                                                    .cloned()
                                                    .unwrap_or(Item {
                                                        short_name: None,
                                                        id: sell_order.item_id,
                                                        display_name: None,
                                                        description: None,
                                                        icon_url: None,
                                                    });

                                                let buying_item_uri = buying_item.clone().icon_url;

                                                rect()
                                                    .width(Size::Fill)
                                                    .height(Size::px(56.0))
                                                    .background(Color::from_hex("#5D5D5D").unwrap())
                                                    .corner_radius(CornerRadius::new_all(8.0))
                                                    .direction(Direction::Horizontal)
                                                    .padding(4.0)
                                                    .spacing(4.0)
                                                    .children([rect()
                                                        .width(Size::percent(50.0))
                                                        .height(Size::Fill)
                                                        .spacing(4.0)
                                                        .direction(Direction::Horizontal)
                                                        .cross_align(Alignment::Center)
                                                        .children([
                                                            rect()
                                                                .width(Size::px(48.0))
                                                                .height(Size::px(48.0))
                                                                .background(
                                                                    Color::from_hex(
                                                                        "#000000",
                                                                    )
                                                                    .unwrap(),
                                                                )
                                                                .maybe(
                                                                    selling_item_uri
                                                                        .is_some(),
                                                                    |rect| {
                                                                        let selling_item_uri_static: &'static str = Box::leak(selling_item_uri.unwrap().into_boxed_str());

                                                                        rect.child(ImageViewer::new(selling_item_uri_static))
                                                                    },
                                                                )
                                                                .child(
                                                                    rect()
                                                                        .width(Size::Fill)
                                                                        .height(Size::Fill)
                                                                        .position(
                                                                            Position::new_absolute()
                                                                                .bottom(0.0)
                                                                                .right(3.0),
                                                                        )
                                                                        .main_align(Alignment::End)
                                                                        .cross_align(Alignment::End)
                                                                        .child(
                                                                            label()
                                                                                .font_size(12.0)
                                                                                .font_weight(FontWeight::EXTRA_BOLD)
                                                                                .color(Color::from_hex("#E4DAD1").unwrap())
                                                                                .text(format!("x{}", sell_order.quantity))
                                                                        )
                                                                )
                                                                .into(),
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
                                                                        .text("STOCK")
                                                                        .into(),
                                                                    label()
                                                                        .color(Color::from_hex("#E4DAD1").unwrap())
                                                                        .font_size(16.0)
                                                                        .font_weight(FontWeight::BOLD)
                                                                        .text(
                                                                            sell_order
                                                                                .amount_in_stock
                                                                                .to_string(),
                                                                        )
                                                                        .into(),
                                                                ])
                                                                .into(),
                                                        ])
                                                        .into(),
                                                        rect()
                                                        .width(Size::percent(50.0))
                                                        .height(Size::Fill)
                                                        .spacing(4.0)
                                                        .direction(Direction::Horizontal)
                                                        .cross_align(Alignment::Center)
                                                        .children([
                                                            rect()
                                                                .width(Size::px(48.0))
                                                                .height(Size::px(48.0))
                                                                .background(
                                                                    Color::from_hex(
                                                                        "#000000",
                                                                    )
                                                                    .unwrap(),
                                                                )
                                                                .maybe(
                                                                    buying_item_uri
                                                                        .is_some(),
                                                                    |rect| {
                                                                        let buying_item_uri_static: &'static str = Box::leak(buying_item_uri.unwrap().into_boxed_str());

                                                                        rect.child(ImageViewer::new(buying_item_uri_static))
                                                                    },
                                                                )
                                                                .into(),
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
                                                                        .text("COST")
                                                                        .into(),
                                                                    label()
                                                                        .color(Color::from_hex("#E4DAD1").unwrap())
                                                                        .font_size(16.0)
                                                                        .font_weight(FontWeight::BOLD)
                                                                        .text(
                                                                            sell_order
                                                                                .cost_per_item
                                                                                .to_string(),
                                                                        )
                                                                        .into(),
                                                                ])
                                                                .into(),
                                                        ])
                                                        .into()])
                                                    .into()
                                            },
                                        ))
                                        .into(),
                                ])
                                .into(),
                        )
                    }))
                    .into(),
                // rect()
                //     .width(Size::Fill)
                //     .height(Size::Fill)
                //     .children(
                //         map_markers
                //             .markers
                //             .iter()
                //             .filter_map(|marker| {
                //                 if AppMarkerType::try_from(marker.marker_type)
                //                     .unwrap_or_default()
                //                     != AppMarkerType::VendingMachine
                //                 {
                //                     println!("Marker type: {:?}", marker.marker_type);
                //                     return None;
                //                 }
                //                 Some(
                //                     rect()
                //                         .width(Size::Fill)
                //                         .height(Size::Fill)
                //                         .corner_radius(CornerRadius::new_all(8.0))
                //                         .background(Color::RED)
                //                         .main_align(Alignment::Center)
                //                         .cross_align(Alignment::Center)
                //                         .children([
                //                             rect()
                //                                 .width(Size::Fill)
                //                                 .padding(Gaps::new(4.0, 8.0, 4.0, 8.0))
                //                                 .main_align(Alignment::SpaceBetween)
                //                                 .cross_align(Alignment::Center)
                //                                 .children([])
                //                                 .into(),
                //                             ScrollView::new()
                //                                 .width(Size::Fill)
                //                                 .height(Size::Fill)
                //                                 .spacing(6.)
                //                                 .children_iter(marker
                //                                         .sell_orders
                //                                         .iter()
                //                                         .map(|sell_order| {
                //                                             let item = map
                //                                                 .get(&sell_order.item_id)
                //                                                 .cloned()
                //                                                 .unwrap_or(Item {
                //                                                     short_name: None,
                //                                                     id: sell_order.item_id,
                //                                                     display_name: None,
                //                                                     description: None,
                //                                                     icon_url: None,
                //                                                 });

                //                                             let image_uri = item.clone().icon_url;

                //                                             rect()
                //                                                 .width(Size::Fill)
                //                                                 .height(Size::px(56.0))
                //                                                 .corner_radius(
                //                                                     CornerRadius::new_all(8.0),
                //                                                 )
                //                                                 .padding(4.0)
                //                                                 .background(Color::BLUE)
                //                                                 .children([rect()
                //                                                     .width(Size::percent(50.0))
                //                                                     .height(Size::Fill)
                //                                                     .children([rect()
                //                                                         .background(
                //                                                             Color::from_hex(
                //                                                                 "#000000",
                //                                                             )
                //                                                             .unwrap(),
                //                                                         )
                //                                                         .maybe(
                //                                                             image_uri
                //                                                                 .is_some(),
                //                                                             |rect| {
                //                                                                 let image_uri_static: &'static str = Box::leak(image_uri.unwrap().into_boxed_str());
                //                                                                 rect.children([
                //                                                                     ImageViewer::new(image_uri_static).into(),
                //                                                                     label()
                //                                                                         .text(
                //                                                                             item.display_name
                //                                                                                 .unwrap_or(item.id.to_string()),
                //                                                                         ).into()
                //                                                                 ])
                //                                                             },
                //                                                         )
                //                                                         .into()]).into()
                //                                                     ])
                //                                                 .into()
                //                                         })
                //                                     )
                //                                 .into(),
                //                         ])
                //                         .into(),
                //                 )
                //             })
                //             .collect::<Vec<Element>>(),
                //     )
                //     .into()
            ])
            .into()
    }
}
