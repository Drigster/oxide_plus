use freya::prelude::*;
use freya_radio::hooks::use_radio;

use crate::{
    app::{Data, DataChannel},
    components::DragableCanvas,
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
                .children([
                    rect()
                        .width(Size::px(map_state.width as f32))
                        .height(Size::px(map_state.height as f32))
                        .child(ImageViewer::new(("map", image_bytes)))
                        .into(),
                    rect()
                        .width(Size::px(map_state.width as f32))
                        .height(Size::px(map_state.height as f32))
                        .layer(1)
                        .children(
                            map_state
                                .monuments
                                .iter()
                                .map(|monument| {
                                    rect()
                                        .width(Size::px(10.0))
                                        .height(Size::px(10.0))
                                        .corner_radius(CornerRadius::new_all(1000.0))
                                        .background(Color::RED)
                                        .position(
                                            Position::new_absolute()
                                                .left(
                                                    (monument.x * scale_x as f32)
                                                        + map_state.ocean_margin as f32,
                                                )
                                                .top(
                                                    ((map_size as f32 - monument.y)
                                                        * scale_y as f32)
                                                        + map_state.ocean_margin as f32,
                                                ),
                                        )
                                        .main_align(Alignment::Center)
                                        .cross_align(Alignment::Center)
                                        .children([label()
                                            .width(Size::px(500.0))
                                            .text_align(TextAlign::Center)
                                            .text(monument.token.clone())
                                            .into()])
                                        .into()
                                })
                                .collect::<Vec<Element>>(),
                        )
                        .into(),
                ])
                .into()])
            .into()
    }
}
