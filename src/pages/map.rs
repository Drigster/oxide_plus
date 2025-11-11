use freya::prelude::*;
use freya_radio::hooks::use_radio;

use crate::{
    app::{Data, DataChannel},
    components::{Button, DragableCanvas, Grid, Setting, SettingType},
    pages::{shops, team},
    utils::text_utils::normalize_monument_name,
};

#[derive(PartialEq)]
pub struct Map {}
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
        let map_size = info_state.map_size;

        let map_state_clone = map_state.clone();
        let image_bytes: &'static [u8] = Box::leak(map_state_clone.jpg_image.into_boxed_slice());

        let scale_x = (map_state.width as f64 - map_state.ocean_margin as f64 * 2.0)
            / (info_state.map_size as f64);
        let scale_y =
            (map_state.height as f64 - map_state.ocean_margin as f64 * 2.0) / (map_size as f64);

        let mut minimap_page = use_state(|| false);
        let mut minimap = use_state(|| false);
        let mut zoom = use_state(|| 1.0_f32);

        let mut grid = use_state(|| false);
        let mut teammates = use_state(|| false);
        let mut deaths = use_state(|| false);
        let mut monuments = use_state(|| false);
        let mut shops = use_state(|| false);

        use_side_effect(move || {
            if minimap() {
                EventNotifier::get().launch_window(WindowConfig::new(move || {
                    rect()
                        .width(Size::Fill)
                        .height(Size::Fill)
                        .background(Color::BLACK)
                        .child("I'm a minimap".to_string())
                        .into()
                }));
            }
        });

        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .children([
                rect()
                    .width(Size::Fill)
                    .height(Size::px(48.0))
                    .padding(4.0)
                    .spacing(4.0)
                    .background(Color::BLACK)
                    .direction(Direction::Horizontal)
                    .children([
                        Button::new()
                            .width(Size::px(110.0))
                            .height(Size::Fill)
                            .align(Alignment::Center)
                            .icon(freya_icons::lucide::map())
                            .text("MAP")
                            .on_press(move |_| {
                                if minimap_page() {
                                    minimap_page.set(false);
                                }
                            })
                            .active(!minimap_page())
                            .into(),
                        Button::new()
                            .height(Size::Fill)
                            .icon(freya_icons::lucide::grid_2x2())
                            .on_press(move |_| {
                                grid.set(!grid());
                            })
                            .active(grid())
                            .into(),
                        Button::new()
                            .height(Size::Fill)
                            .icon(freya_icons::lucide::users_round())
                            .on_press(move |_| {
                                teammates.set(!teammates());
                            })
                            .active(teammates())
                            .into(),
                        Button::new()
                            .height(Size::Fill)
                            .icon(freya_icons::lucide::skull())
                            .on_press(move |_| {
                                deaths.set(!deaths());
                            })
                            .active(deaths())
                            .into(),
                        Button::new()
                            .height(Size::Fill)
                            .icon(freya_icons::lucide::factory())
                            .on_press(move |_| {
                                monuments.set(!monuments());
                            })
                            .active(monuments())
                            .into(),
                        Button::new()
                            .height(Size::Fill)
                            .icon(freya_icons::lucide::store())
                            .on_press(move |_| {
                                shops.set(!shops());
                            })
                            .active(shops())
                            .into(),
                        rect().into(),
                        Button::new()
                            .width(Size::px(110.0))
                            .height(Size::Fill)
                            .align(Alignment::Center)
                            .icon(freya_icons::lucide::locate_fixed())
                            .text("MINIMAP")
                            .on_press(move |_| {
                                if !minimap_page() {
                                    minimap_page.set(true);
                                }
                            })
                            .active(minimap_page())
                            .into(),
                    ])
                    .into(),
                if minimap_page() {
                    rect()
                        .width(Size::Fill)
                        .height(Size::Fill)
                        .padding(8.0)
                        .spacing(4.0)
                        .children([
                            rect()
                                .width(Size::Fill)
                                .height(Size::px(48.0))
                                .padding(8.0)
                                .spacing(8.0)
                                .direction(Direction::Horizontal)
                                .cross_align(Alignment::Center)
                                .children([
                                    svg(freya_icons::lucide::map())
                                        .height(Size::Fill)
                                        .color(Color::from_hex("#5D7238").unwrap())
                                        .into(),
                                    label()
                                        .font_size(24.0)
                                        .font_weight(FontWeight::BOLD)
                                        .color(Color::from_hex("#E4DAD1").unwrap())
                                        .text("POSITION")
                                        .into(),
                                ])
                                .into(),
                            Setting::new(SettingType::Toggle)
                                .text("ENABLED")
                                .on_change(move |state| {
                                    minimap.set(state);
                                })
                                .into(),
                            Setting::new(SettingType::Toggle).text("POSITION").into(),
                            Setting::new(SettingType::Toggle).text("SIZE").into(),
                            Setting::new(SettingType::Toggle).text("OFFSET").into(),
                            rect()
                                .width(Size::Fill)
                                .height(Size::px(48.0))
                                .padding(8.0)
                                .spacing(8.0)
                                .direction(Direction::Horizontal)
                                .cross_align(Alignment::Center)
                                .children([
                                    svg(freya_icons::lucide::toggle_left())
                                        .height(Size::Fill)
                                        .color(Color::from_hex("#5D7238").unwrap())
                                        .into(),
                                    label()
                                        .font_size(24.0)
                                        .font_weight(FontWeight::BOLD)
                                        .color(Color::from_hex("#E4DAD1").unwrap())
                                        .text("TOGGLES")
                                        .into(),
                                ])
                                .into(),
                            Setting::new(SettingType::Toggle).text("GRID").into(),
                            Setting::new(SettingType::Toggle).text("TEAMMATES").into(),
                            Setting::new(SettingType::Toggle).text("MONUMENTS").into(),
                            Setting::new(SettingType::Toggle).text("MARKERS").into(),
                            Setting::new(SettingType::Toggle).text("DEATHS").into(),
                        ])
                        .into()
                } else {
                    rect()
                        .padding(8.0)
                        .child(
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
                                    .on_zoom(move |v| {
                                        zoom.set(v as f32);
                                    })
                                    .child(
                                        rect()
                                            .width(Size::px(map_state.width as f32))
                                            .height(Size::px(map_state.height as f32))
                                            .child(ImageViewer::new(("map", image_bytes))),
                                    )
                                    .maybe_child(if grid() {
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
                                    .maybe_child(if monuments() {
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
                                                            if monument.token
                                                                == "train_tunnel_display_name"
                                                            {
                                                                Some(rect()
                                                                .width(Size::px(6.0))
                                                                .height(Size::px(6.0))
                                                                .corner_radius(
                                                                    CornerRadius::new_all(1000.0),
                                                                )
                                                                .background(Color::BLUE)
                                                                .position(
                                                                    Position::new_absolute()
                                                                        .left(
                                                                            (monument.x
                                                                                * scale_x as f32)
                                                                                + map_state
                                                                                    .ocean_margin
                                                                                    as f32
                                                                                - 3.0,
                                                                        )
                                                                        .top(
                                                                            ((map_size as f32
                                                                                - monument.y)
                                                                                * scale_y as f32)
                                                                                + map_state
                                                                                    .ocean_margin
                                                                                    as f32
                                                                                - 3.0,
                                                                        ),
                                                                )
                                                                .main_align(Alignment::Center)
                                                                .cross_align(Alignment::Center)
                                                                .into())
                                                            } else if monument
                                                                .token
                                                                .starts_with("assets")
                                                            {
                                                                None
                                                            } else {
                                                                Some(rect()
                                                                .width(Size::px(6.0))
                                                                .height(Size::px(6.0))
                                                                .corner_radius(
                                                                    CornerRadius::new_all(1000.0),
                                                                )
                                                                // .background(Color::RED)
                                                                .position(
                                                                    Position::new_absolute()
                                                                        .left(
                                                                            (monument.x
                                                                                * scale_x as f32)
                                                                                + map_state
                                                                                    .ocean_margin
                                                                                    as f32
                                                                                - 3.0,
                                                                        )
                                                                        .top(
                                                                            ((map_size as f32
                                                                                - monument.y)
                                                                                * scale_y as f32)
                                                                                + map_state
                                                                                    .ocean_margin
                                                                                    as f32
                                                                                - 3.0,
                                                                        ),
                                                                )
                                                                .main_align(Alignment::Center)
                                                                .cross_align(Alignment::Center)
                                                                .child(
                                                                    label()
                                                                        .width(Size::px(500.0))
                                                                        .text_align(
                                                                            TextAlign::Center,
                                                                        )
                                                                        // Magic numbers :)
                                                                        .font_size(
                                                                            8.864 / zoom() + 2.446,
                                                                        )
                                                                        .font_family(
                                                                            "PermanentMarker",
                                                                        )
                                                                        .color(
                                                                            Color::from_hex(
                                                                                "#e6191919",
                                                                            )
                                                                            .unwrap(),
                                                                        )
                                                                        .text(
                                                                            normalize_monument_name(
                                                                                monument
                                                                                    .token
                                                                                    .clone(),
                                                                            ),
                                                                        ),
                                                                )
                                                                .into())
                                                            }
                                                        })
                                                        .collect::<Vec<Element>>(),
                                                ),
                                        )
                                    } else {
                                        None
                                    })
                                    .into()]),
                        )
                        .into()
                },
            ])
            .into()
    }
}

fn get_text_size_concise(scale: f32) -> f32 {
    return 8.864 / scale + 2.446;
}
