use euclid::Point2D;
use freya::{prelude::*, radio::use_radio};
use rustplus_rs::AppMarkerType;
use serde::de::value;

use crate::{
    Data, DataChannel,
    components::{DragableCanvas, Grid, markers},
};

#[derive(PartialEq)]
pub struct Map {
    center: bool,
    interactable: bool,

    grid: bool,
    markers: bool,
    deaths: bool,
    monuments: bool,
    shops: bool,

    background_opacity: f32,
    zoom: f32,

    on_zoom: Option<EventHandler<f32>>,
    on_center_cancel: Option<EventHandler<()>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            center: false,
            interactable: true,

            grid: false,
            markers: false,
            deaths: false,
            monuments: false,
            shops: false,

            background_opacity: 1.0,
            zoom: 1.0,

            on_zoom: None,
            on_center_cancel: None,
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
        self.grid = grid;
        self
    }

    pub fn markers(mut self, markers: bool) -> Self {
        self.markers = markers;
        self
    }

    pub fn deaths(mut self, deaths: bool) -> Self {
        self.deaths = deaths;
        self
    }

    pub fn monuments(mut self, monuments: bool) -> Self {
        self.monuments = monuments;
        self
    }

    pub fn shops(mut self, shops: bool) -> Self {
        self.shops = shops;
        self
    }

    pub fn background_opacity(mut self, background_opacity: f32) -> Self {
        self.background_opacity = background_opacity;
        self
    }

    pub fn zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom;
        self
    }

    pub fn on_zoom(mut self, on_zoom: impl Into<EventHandler<f32>>) -> Self {
        self.on_zoom = Some(on_zoom.into());
        self
    }

    pub fn on_center_cancel(mut self, on_center_cancel: impl Into<EventHandler<()>>) -> Self {
        self.on_center_cancel = Some(on_center_cancel.into());
        self
    }
}

impl Component for Map {
    fn render(&self) -> impl IntoElement {
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
        let team_info_binding = use_radio::<Data, DataChannel>(DataChannel::TeamInfoUpdate);
        let team_info = team_info_binding
            .read()
            .team_info
            .clone()
            .expect("Team info should be loaded");
        let user_data_binding = use_radio::<Data, DataChannel>(DataChannel::UserDataUpdate);
        let user_data = user_data_binding
            .read()
            .user_data
            .clone()
            .expect("User data should be loaded");

        let scale = info_state.map_size as f32
            / (map_state.width as f32 - map_state.ocean_margin as f32 * 2.0);

        let map_size = map_state.width as f32 * scale;
        let margin = map_state.ocean_margin as f32 * scale;

        let mut pos: State<Point2D<f32, ()>> =
            use_state(|| Point2D::new(map_size / 2.0, map_size / 2.0));

        let mut me_pos = use_state(|| Point2D::new(0.0, 0.0));

        let map_state_clone = map_state.clone();
        let image_bytes: &'static [u8] = Box::leak(map_state_clone.jpg_image.into_boxed_slice());

        rect()
            .corner_radius(8.0)
            .overflow(Overflow::Clip)
            .width(Size::Fill)
            .height(Size::Fill)
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            .background(
                Color::from_hex(&map_state.background)
                    .unwrap()
                    .with_a((self.background_opacity * 255.0).round() as u8),
            )
            .children([DragableCanvas::new()
                .interactable(self.interactable)
                .children_size(Point2D::new(map_size, map_size))
                .on_zoom({
                    let on_zoom = self.on_zoom.clone();
                    move |v| {
                        if let Some(on_zoom) = &on_zoom {
                            on_zoom.call(v);
                        }
                    }
                })
                .on_pos_update({
                    let on_center_cancel = self.on_center_cancel.clone();
                    move |(value, is_drag)| {
                        if is_drag {
                            if let Some(on_center_cancel) = &on_center_cancel {
                                on_center_cancel.call(());
                            }
                        }
                        pos.set(value);
                    }
                })
                .maybe(true, |rect| {
                    if self.center {
                        rect.pos_centered(*me_pos.read())
                    } else {
                        rect.pos(*pos.read())
                    }
                })
                .child(
                    rect()
                        .background(Color::BLUE)
                        .width(Size::px(map_size))
                        .height(Size::px(map_size))
                        .opacity(self.background_opacity)
                        .child(
                            ImageViewer::new(("map", image_bytes))
                                .width(Size::px(map_size))
                                .height(Size::px(map_size)),
                        ),
                )
                .maybe_child(if self.grid {
                    Some(
                        rect()
                            .width(Size::px(map_size))
                            .height(Size::px(map_size))
                            .layer(1)
                            .child(Grid::new(map_size, margin).zoom(self.zoom)),
                    )
                } else {
                    None
                })
                .maybe_child(if self.monuments {
                    Some(
                        rect()
                            .width(Size::px(map_size))
                            .height(Size::px(map_size))
                            .layer(2)
                            .children(
                                map_state
                                    .monuments
                                    .iter()
                                    .filter_map(|monument| {
                                        if monument.token == "train_tunnel_display_name" {
                                            None
                                        } else if monument.token.starts_with("assets") {
                                            None
                                        } else {
                                            Some(
                                                markers::Monument::new(
                                                    monument.clone(),
                                                    map_size,
                                                    margin,
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
                .maybe_child(if self.markers || self.deaths {
                    Some(
                        rect()
                            .width(Size::px(map_size))
                            .height(Size::px(map_size))
                            .layer(2)
                            .children(team_info.map_notes.iter().filter_map(|map_note| {
                                match map_note.r#type {
                                    0 => {
                                        if self.deaths == false {
                                            return None;
                                        }
                                        Some(
                                            markers::Death::new(map_note.clone(), map_size, margin)
                                                .into(),
                                        )
                                    }
                                    1 => {
                                        if self.markers == false {
                                            return None;
                                        }
                                        Some(
                                            markers::MapNote::new(
                                                map_note.clone(),
                                                map_size,
                                                margin,
                                            )
                                            .into(),
                                        )
                                    }
                                    _ => {
                                        println!("Unused marker: {:?}", map_note);
                                        None
                                    }
                                }
                            })),
                    )
                } else {
                    None
                })
                .child(
                    rect()
                        .width(Size::px(map_size))
                        .height(Size::px(map_size))
                        .layer(2)
                        .children(team_info.members.iter().filter_map(|member| {
                            let me = member.steam_id
                                == user_data
                                    .steam_id
                                    .parse::<u64>()
                                    .expect("Steam ID should be a u64");

                            if me {
                                let point: Point2D<f32, ()> = Point2D::new(
                                    -(member.x + margin),
                                    -(map_size - member.y - margin),
                                );
                                me_pos.set_if_modified(point);
                            }

                            if member.is_alive == false && member.death_time == 0 {
                                return None;
                            }

                            Some(markers::Player::new(member.clone(), map_size, margin, me).into())
                        })),
                )
                .child(
                    rect()
                        .width(Size::px(map_size))
                        .height(Size::px(map_size))
                        .layer(2)
                        .children(marker_state.markers.iter().filter_map(|marker| {
                            match marker.r#type() {
                                AppMarkerType::VendingMachine => {
                                    if self.shops == false {
                                        return None;
                                    }
                                    Some(
                                        markers::VendingMachine::new(
                                            marker.clone(),
                                            map_size,
                                            margin,
                                        )
                                        .into(),
                                    )
                                }
                                AppMarkerType::Player => None,
                                AppMarkerType::CargoShip => Some(
                                    markers::CargoShip::new(marker.clone(), map_size, margin)
                                        .into(),
                                ),
                                AppMarkerType::GenericRadius => None,
                                marker_type => {
                                    if self.markers == false {
                                        return None;
                                    }
                                    println!("Unknown marker: {:?}", marker);
                                    Some(
                                        rect()
                                            .width(Size::px(4.0))
                                            .height(Size::px(4.0))
                                            .corner_radius(CornerRadius::new_all(1000.0))
                                            .background(Color::YELLOW)
                                            .position(
                                                Position::new_absolute()
                                                    .left((marker.x) - 2.0)
                                                    .top((map_size - marker.y) - 2.0),
                                            )
                                            .main_align(Alignment::Center)
                                            .cross_align(Alignment::Center)
                                            .child(
                                                label()
                                                    .width(Size::px(500.0))
                                                    .text_align(TextAlign::Center)
                                                    // Magic numbers :)
                                                    .font_size(8.864 / self.zoom + 2.446)
                                                    .font_family("PermanentMarker")
                                                    .color(Color::from_hex("#191919e6").unwrap())
                                                    .text(format!("{:?}", marker_type)),
                                            )
                                            .into(),
                                    )
                                }
                            }
                        })),
                )
                .into()])
    }
}
