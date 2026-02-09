use std::{collections::HashMap};

use euclid::Point2D;
use freya::{prelude::*, radio::use_radio};
use rustplus_rs::{
    AppMarker, AppMarkerType,
    app_map::Monument,
    app_team_info::{ Note},
};

use crate::{
    Data, DataChannel, TeamMember,
    components::{
        DragableCanvas, Grid,
        markers::{self},
    },
};

#[derive(PartialEq)]
pub struct Map {
    center: Readable<bool>,
    interactable: bool,

    grid: Readable<bool>,
    markers: Readable<bool>,
    deaths: Readable<bool>,
    monuments: Readable<bool>,
    shops: Readable<bool>,

    background_opacity: Readable<f32>,
    zoom: Option<Writable<f32>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            center: false.into(),
            interactable: true,

            grid: false.into(),
            markers: false.into(),
            deaths: false.into(),
            monuments: false.into(),
            shops: false.into(),

            background_opacity: 100.0.into(),
            zoom: None,
        }
    }

    pub fn center(mut self, center: impl Into<Readable<bool>>) -> Self {
        self.center = center.into();
        self
    }

    pub fn interactable(mut self, interactable: bool) -> Self {
        self.interactable = interactable;
        self
    }

    pub fn grid(mut self, grid: impl Into<Readable<bool>>) -> Self {
        self.grid = grid.into();
        self
    }

    pub fn markers(mut self, markers: impl Into<Readable<bool>>) -> Self {
        self.markers = markers.into();
        self
    }

    pub fn deaths(mut self, deaths: impl Into<Readable<bool>>) -> Self {
        self.deaths = deaths.into();
        self
    }

    pub fn monuments(mut self, monuments: impl Into<Readable<bool>>) -> Self {
        self.monuments = monuments.into();
        self
    }

    pub fn shops(mut self, shops: impl Into<Readable<bool>>) -> Self {
        self.shops = shops.into();
        self
    }

    pub fn background_opacity(mut self, background_opacity: impl Into<Readable<f32>>) -> Self {
        self.background_opacity = background_opacity.into();
        self
    }

    pub fn zoom(mut self, zoom: impl Into<Writable<f32>>) -> Self {
        self.zoom = Some(zoom.into());
        self
    }
}

impl Component for Map {
    fn render(&self) -> impl IntoElement {
        let radio = use_radio::<Data, DataChannel>(DataChannel::InfoStateUpdate);
        let info_state = radio.slice_current(|s| &s.info_state);
        let map_state = radio.slice(DataChannel::MapStateUpdate, |s| &s.map_state);
        let monuments = radio.slice(DataChannel::MapStateUpdate, |s| &s.map_state.monuments);
        let map_notes = radio.slice(DataChannel::MapNotesUpdate, |s| &s.team_info.map_notes);
        let team_members = radio.slice(DataChannel::TeamMembersUpdate, |s| &s.team_info.members);
        let user_data = radio.slice(DataChannel::UserDataUpdate, |s| &s.user_data);
        let marker_state = radio.slice(DataChannel::MapMarkersUpdate, |s| &s.map_markers.markers);

        let (map_size, margin) = match info_state.read().map_size {
            Some(map_size) => {
                let scale = map_size as f32
                    / (map_state.read().width as f32 - map_state.read().ocean_margin as f32 * 2.0);

                let map_size = map_state.read().width as f32 * scale;
                let margin = map_state.read().ocean_margin as f32 * scale;
                (map_size, margin)
            }
            None => (0.0, 0.0),
        };

        let mut pos: State<Point2D<f32, ()>> =
            use_state(|| Point2D::new(map_size / 2.0, map_size / 2.0));
        let zoom = use_hook(|| self.zoom.clone().unwrap_or_else(|| State::create(1.0).into()));

        let me_steam_id = user_data.read().steam_id.clone().unwrap_or("0".to_string()).parse().unwrap_or(0);
        let me_radio = radio.slice(DataChannel::TeamMemberUpdate(me_steam_id), |s| &s.team_info.members);
        use_side_effect({
            let center = self.center.clone();
            let zoom = zoom.clone();
            move || {
            let binding = me_radio.read();
            let me = binding.get(&me_steam_id);
            if me.is_none() {
                return;
            }

            let me = me.unwrap();

            let point: Point2D<f32, ()> = Point2D::new(
                -(me.x + margin),
                -(map_size - me.y - margin),
            );
            if *center.read() {
                // Scale the position and compensate for scale offset
                // When content is scaled from top-left, we need to offset
                // by half the size change to keep the target centered
                let current_zoom = *zoom.read();
                let scale_offset = map_size * (current_zoom - 1.0) / 2.0;
                let scaled_point = Point2D::new(
                    point.x * current_zoom + scale_offset,
                    point.y * current_zoom + scale_offset,
                );
                *pos.write() = scaled_point;
            }
        }});

        let image_bytes: &'static [u8] =
            Box::leak(map_state.read().jpg_image.clone().into_boxed_slice());

        rect()
            .corner_radius(8.0)
            .overflow(Overflow::Clip)
            .width(Size::Fill)
            .height(Size::Fill)
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            // .background(
            //     Color::from_hex(&map_state.read().background)
            //         .unwrap()
            //         .with_a((*self.background_opacity.read() * 255.0).round() as u8),
            // )
            .children([DragableCanvas::new()
                .interactable(self.interactable)
                .children_size(Point2D::new(map_size, map_size))
                .zoom(zoom.clone())
                .pos(pos)
                .child(ImageLayer {
                    image_bytes,
                    map_size,
                    background_opacity: self.background_opacity.clone().into(),
                })
                .child(GridLayer {
                    grid: self.grid.clone(),
                    map_size,
                    margin,
                    zoom: zoom.clone().into(),
                })
                .child(MonumentLayer {
                    monuments: self.monuments.clone(),

                    map_size,
                    margin,
                    monument_list: monuments.into_readable(),
                })
                .child(MapNoteLayer {
                    markers: self.markers.clone(),
                    deaths: self.deaths.clone(),
                    map_size: map_size,
                    margin: margin,
                    map_notes: map_notes.into_readable(),
                })
                .child(TeamLayer {
                    members: team_members.into_readable(),

                    map_size,
                    margin,
                    zoom: zoom.clone().into(),
                    center: self.center.clone(),
                })
                .child(MarkerLayer {
                    shops: self.shops.clone(),
                    markers: self.markers.clone(),

                    map_size,
                    margin,

                    zoom: zoom.into(),

                    marker_list: marker_state.into_readable(),
                })
                .into()])
    }
}

#[derive(PartialEq)]
struct ImageLayer {
    image_bytes: &'static [u8],
    map_size: f32,
    background_opacity: Readable<f32>,
}

impl Component for ImageLayer {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::px(self.map_size))
            .height(Size::px(self.map_size))
            .opacity(*self.background_opacity.read() / 100.0)
            .child(
                ImageViewer::new(("map", self.image_bytes))
                    .width(Size::px(self.map_size))
                    .height(Size::px(self.map_size)),
            )
    }
}

#[derive(PartialEq)]
struct GridLayer {
    grid: Readable<bool>,
    map_size: f32,
    margin: f32,
    zoom: Readable<f32>,
}

impl Component for GridLayer {
    fn render(&self) -> impl IntoElement {
        if *self.grid.read() == false {
            return rect().into();
        }

        rect()
            .width(Size::px(self.map_size))
            .height(Size::px(self.map_size))
            .layer(1)
            .child(Grid::new(self.map_size, self.margin).zoom(self.zoom.clone()))
    }
}

#[derive(PartialEq)]
struct MonumentLayer {
    monuments: Readable<bool>,

    map_size: f32,
    margin: f32,
    monument_list: Readable<Vec<Monument>>,
}

impl Component for MonumentLayer {
    fn render(&self) -> impl IntoElement {
        if *self.monuments.read() == false {
            return rect().into();
        }

        rect()
            .width(Size::px(self.map_size))
            .height(Size::px(self.map_size))
            .layer(2)
            .children(
                self.monument_list
                    .read()
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
                                    self.map_size,
                                    self.margin,
                                )
                                .into(),
                            )
                        }
                    })
                    .collect::<Vec<Element>>(),
            )
    }
}

#[derive(PartialEq)]
struct MapNoteLayer {
    markers: Readable<bool>,
    deaths: Readable<bool>,

    map_size: f32,
    margin: f32,
    map_notes: Readable<Vec<Note>>,
}

impl Component for MapNoteLayer {
    fn render(&self) -> impl IntoElement {
        if *self.markers.read() == false && *self.deaths.read() == false {
            return rect().into();
        }

        rect()
            .width(Size::px(self.map_size))
            .height(Size::px(self.map_size))
            .layer(2)
            .children(
                self.map_notes
                    .read()
                    .iter()
                    .filter_map(|map_note| match map_note.r#type {
                        0 => {
                            if *self.deaths.read() == false {
                                return None;
                            }
                            Some(
                                markers::Death::new(map_note.clone(), self.map_size, self.margin)
                                    .into(),
                            )
                        }
                        1 => {
                            if *self.markers.read() == false {
                                return None;
                            }
                            Some(
                                markers::MapNote::new(map_note.clone(), self.map_size, self.margin)
                                    .into(),
                            )
                        }
                        _ => {
                            println!("Unused marker: {:?}", map_note);
                            None
                        }
                    }),
            )
    }
}

#[derive(PartialEq)]
struct TeamLayer {
    members: Readable<HashMap<u64, TeamMember>>,

    map_size: f32,
    margin: f32,
    zoom: Readable<f32>,
    center: Readable<bool>,
}

impl Component for TeamLayer {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::px(self.map_size))
            .height(Size::px(self.map_size))
            .layer(2)
            .children(
                self.members
                    .read()
                    .values()
                    .into_iter()
                    .filter_map(|member| {
                        if member.is_alive == false && member.death_time == 0 {
                            return None;
                        }

                        Some(
                            markers::Player::new(member.steam_id, self.map_size, self.margin)
                                .into(),
                        )
                    }),
            )
    }
}

#[derive(PartialEq)]
struct MarkerLayer {
    shops: Readable<bool>,
    markers: Readable<bool>,

    map_size: f32,
    margin: f32,

    zoom: Readable<f32>,

    marker_list: Readable<Vec<AppMarker>>,
}

impl Component for MarkerLayer {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::px(self.map_size))
            .height(Size::px(self.map_size))
            .layer(2)
            .children(self.marker_list.read().iter().filter_map(|marker| {
                match marker.r#type() {
                    AppMarkerType::VendingMachine => {
                        if *self.shops.read() == false {
                            return None;
                        }
                        Some(
                            markers::VendingMachine::new(
                                marker.clone(),
                                self.map_size,
                                self.margin,
                            )
                            .into(),
                        )
                    }
                    AppMarkerType::Player => None,
                    AppMarkerType::CargoShip => Some(
                        markers::CargoShip::new(marker.clone(), self.map_size, self.margin).into(),
                    ),
                    AppMarkerType::GenericRadius => None,
                    marker_type => {
                        if *self.markers.read() == false {
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
                                        .top((self.map_size - marker.y) - 2.0),
                                )
                                .main_align(Alignment::Center)
                                .cross_align(Alignment::Center)
                                .child(
                                    label()
                                        .width(Size::px(500.0))
                                        .text_align(TextAlign::Center)
                                        // Magic numbers :)
                                        .font_size(8.864 / *self.zoom.read() + 2.446)
                                        .font_family("PermanentMarker")
                                        .color(Color::from_hex("#191919e6").unwrap())
                                        .text(format!("{:?}", marker_type)),
                                )
                                .into(),
                        )
                    }
                }
            }))
    }
}
