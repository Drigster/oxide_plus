use freya::prelude::*;
use freya_radio::prelude::*;
use rustplus_rs::AppMarkerType;

use crate::{
    app::{Data, DataChannel},
    components::{DragableCanvas, Grid},
    utils::text_utils::normalize_monument_name,
};

#[derive(PartialEq)]
pub struct Map {
    center: bool,
    interactable: bool,

    grid: State<bool>,
    markers: State<bool>,
    deaths: State<bool>,
    monuments: State<bool>,
    shops: State<bool>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            center: true,
            interactable: true,
            grid: use_state(|| false),
            markers: use_state(|| false),
            deaths: use_state(|| false),
            monuments: use_state(|| false),
            shops: use_state(|| false),
        }
    }

    pub fn center(mut self, center: bool) -> Self {
        self.center = center;
        self
    }

    pub fn interactable(mut self, interactable: bool) -> Self {
        self.interactable = interactable;
        self
    }

    pub fn grid(mut self, grid: bool) -> Self {
        *self.grid.write() = grid;
        self
    }

    pub fn markers(mut self, markers: bool) -> Self {
        *self.markers.write() = markers;
        self
    }

    pub fn deaths(mut self, deaths: bool) -> Self {
        *self.deaths.write() = deaths;
        self
    }

    pub fn monuments(mut self, monuments: bool) -> Self {
        *self.monuments.write() = monuments;
        self
    }

    pub fn shops(mut self, shops: bool) -> Self {
        *self.shops.write() = shops;
        self
    }
}

impl Render for Map {
    fn render(&self) -> Element {
        let map_state_binding = use_radio::<Data, DataChannel>(DataChannel::MapStateUpdate);
        let map_state = map_state_binding
            .read()
            .map_state
            .clone()
            .expect("Map state should be loaded");
        let info_state_binding = use_radio::<Data, DataChannel>(DataChannel::InfoStateUpdate);
        let info_state = info_state_binding
            .read()
            .info_state
            .clone()
            .expect("Server info should be loaded");
        let marker_state_binding = use_radio::<Data, DataChannel>(DataChannel::MapMarkersUpdate);
        let marker_state = marker_state_binding
            .read()
            .map_markers
            .clone()
            .expect("Map markers should be loaded");

        let map_size = info_state.map_size;

        let map_state_clone = map_state.clone();
        let image_bytes: &'static [u8] = Box::leak(map_state_clone.jpg_image.into_boxed_slice());

        let scale_x = (map_state.width as f64 - map_state.ocean_margin as f64 * 2.0)
            / (info_state.map_size as f64);
        let scale_y =
            (map_state.height as f64 - map_state.ocean_margin as f64 * 2.0) / (map_size as f64);

        let mut zoom = use_state(|| 1.0_f32);

        rect()
            .corner_radius(8.0)
            .overflow_mode(OverflowMode::Clip)
            .width(Size::Fill)
            .height(Size::Fill)
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            .background(
                Color::from_hex(
                    map_state
                        .background
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or("#000000"),
                )
                .unwrap(),
            )
            .children([DragableCanvas::new()
                .zoom(0.5)
                .interactable(self.interactable)
                .size(CursorPoint::new(
                    map_state.width as f64,
                    map_state.height as f64,
                ))
                .on_zoom(move |v| {
                    zoom.set(v as f32);
                })
                .child(
                    rect()
                        .width(Size::px(map_state.width as f32))
                        .height(Size::px(map_state.height as f32))
                        .child(ImageViewer::new(("map", image_bytes))),
                )
                .maybe_child(if *self.grid.read() {
                    Some(
                        rect()
                            .width(Size::px(map_state.width as f32))
                            .height(Size::px(map_state.height as f32))
                            .layer(1)
                            .child(
                                Grid::new(
                                    map_state.width,
                                    map_state.height,
                                    map_size as f32,
                                    map_state.ocean_margin as f32,
                                )
                                .on_zoom(zoom()),
                            ),
                    )
                } else {
                    None
                })
                .maybe_child(if *self.monuments.read() {
                    Some(
                        rect()
                            .width(Size::px(map_state.width as f32))
                            .height(Size::px(map_state.height as f32))
                            .layer(2)
                            .children(
                                map_state
                                    .monuments
                                    .iter()
                                    .filter_map(|monument| {
                                        if monument.token == "train_tunnel_display_name" {
                                            Some(
                                                rect()
                                                    .width(Size::px(6.0))
                                                    .height(Size::px(6.0))
                                                    .corner_radius(CornerRadius::new_all(1000.0))
                                                    .background(Color::BLUE)
                                                    .position(
                                                        Position::new_absolute()
                                                            .left(
                                                                (monument.x * scale_x as f32)
                                                                    + map_state.ocean_margin as f32
                                                                    - 3.0,
                                                            )
                                                            .top(
                                                                ((map_size as f32 - monument.y)
                                                                    * scale_y as f32)
                                                                    + map_state.ocean_margin as f32
                                                                    - 3.0,
                                                            ),
                                                    )
                                                    .main_align(Alignment::Center)
                                                    .cross_align(Alignment::Center)
                                                    .into(),
                                            )
                                        } else if monument.token.starts_with("assets") {
                                            None
                                        } else {
                                            Some(
                                                rect()
                                                    .width(Size::px(6.0))
                                                    .height(Size::px(6.0))
                                                    .corner_radius(CornerRadius::new_all(1000.0))
                                                    .position(
                                                        Position::new_absolute()
                                                            .left(
                                                                (monument.x * scale_x as f32)
                                                                    + map_state.ocean_margin as f32
                                                                    - 3.0,
                                                            )
                                                            .top(
                                                                ((map_size as f32 - monument.y)
                                                                    * scale_y as f32)
                                                                    + map_state.ocean_margin as f32
                                                                    - 3.0,
                                                            ),
                                                    )
                                                    .main_align(Alignment::Center)
                                                    .cross_align(Alignment::Center)
                                                    .child(
                                                        label()
                                                            .width(Size::px(500.0))
                                                            .text_align(TextAlign::Center)
                                                            // Magic numbers :)
                                                            .font_size(8.864 / zoom() + 2.446)
                                                            .font_family("PermanentMarker")
                                                            .color(
                                                                Color::from_hex("#e6191919")
                                                                    .unwrap(),
                                                            )
                                                            .text(normalize_monument_name(
                                                                monument.token.clone(),
                                                            )),
                                                    )
                                                    .into(),
                                            )
                                        }
                                    })
                                    .collect::<Vec<Element>>(),
                            ),
                    )
                } else {
                    None
                })
                .child(
                    rect()
                        .width(Size::px(map_state.width as f32))
                        .height(Size::px(map_state.height as f32))
                        .layer(2)
                        .children_iter(marker_state.markers.iter().filter_map(|marker| {
                            match AppMarkerType::try_from(marker.marker_type) {
                                Ok(AppMarkerType::VendingMachine) => {
                                    if *self.shops.read() == false {
                                        return None;
                                    }
                                    Some(
                                        rect()
                                            .width(Size::px(12.0))
                                            .height(Size::px(12.0))
                                            .corner_radius(CornerRadius::new_all(1000.0))
                                            .background(Color::GREEN)
                                            .position(
                                                Position::new_absolute()
                                                    .left(
                                                        (marker.x * scale_x as f32)
                                                            + map_state.ocean_margin as f32
                                                            - 3.0,
                                                    )
                                                    .top(
                                                        ((map_size as f32 - marker.y)
                                                            * scale_y as f32)
                                                            + map_state.ocean_margin as f32
                                                            - 3.0,
                                                    ),
                                            )
                                            .main_align(Alignment::Center)
                                            .cross_align(Alignment::Center)
                                            .into(),
                                    )
                                }
                                Ok(marker_type) => {
                                    if *self.markers.read() == false {
                                        return None;
                                    }
                                    Some(
                                        rect()
                                            .width(Size::px(6.0))
                                            .height(Size::px(6.0))
                                            .corner_radius(CornerRadius::new_all(1000.0))
                                            .background(Color::YELLOW)
                                            .position(
                                                Position::new_absolute()
                                                    .left(
                                                        (marker.x * scale_x as f32)
                                                            + map_state.ocean_margin as f32
                                                            - 3.0,
                                                    )
                                                    .top(
                                                        ((map_size as f32 - marker.y)
                                                            * scale_y as f32)
                                                            + map_state.ocean_margin as f32
                                                            - 3.0,
                                                    ),
                                            )
                                            .main_align(Alignment::Center)
                                            .cross_align(Alignment::Center)
                                            .child(
                                                label()
                                                    .width(Size::px(500.0))
                                                    .text_align(TextAlign::Center)
                                                    // Magic numbers :)
                                                    .font_size(8.864 / zoom() + 2.446)
                                                    .font_family("PermanentMarker")
                                                    .color(Color::from_hex("#e6191919").unwrap())
                                                    .text(normalize_monument_name(
                                                        marker_type.to_string(),
                                                    )),
                                            )
                                            .into(),
                                    )
                                }
                                _ => None,
                            }
                        })),
                )
                .into()])
            .into()
    }
}
