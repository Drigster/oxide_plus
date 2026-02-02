use freya::{prelude::*, radio::use_radio};

use crate::{
    components::Map as MapComponent,
    {Data, DataChannel},
};

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum Shape {
    Circle,
    Square,
}

#[derive(PartialEq)]
pub struct Minimap {}

#[allow(dead_code)]
impl Minimap {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Minimap {
    fn render(&self) -> impl IntoElement {
        let minimap_settings_state =
            use_radio::<Data, DataChannel>(DataChannel::MinimapSettingsUpdate);

        // use_side_effect(move || {
        //     let offset_x = minimap_settings_state
        //         .read()
        //         .settings
        //         .minimap_settings
        //         .offset_x
        //         .clone();

        //     let offset_y = minimap_settings_state
        //         .read()
        //         .settings
        //         .minimap_settings
        //         .offset_y
        //         .clone();

        //     Platform::get().with_window(None, move |window| {
        //         window.set_outer_position(PhysicalPosition::new(offset_x, offset_y));
        //     });
        // });

        println!("redraw 1");

        rect()
            .width(Size::percent(100.0))
            .height(Size::percent(100.0))
            .maybe(
                minimap_settings_state
                    .read()
                    .settings
                    .minimap_settings
                    .shape
                    == Shape::Circle,
                |rect| rect.corner_radius(1000.0),
            )
            .overflow(Overflow::Clip)
            .child(
                MapComponent::new()
                    .interactable(false)
                    .center(true)
                    .background_opacity(
                        minimap_settings_state
                            .read()
                            .settings
                            .minimap_settings
                            .opacity
                            / 100.0,
                    )
                    .zoom(minimap_settings_state.read().settings.minimap_settings.zoom)
                    .grid(minimap_settings_state.read().settings.minimap_settings.grid)
                    .markers(
                        minimap_settings_state
                            .read()
                            .settings
                            .minimap_settings
                            .markers,
                    )
                    .deaths(
                        minimap_settings_state
                            .read()
                            .settings
                            .minimap_settings
                            .deaths,
                    )
                    .monuments(
                        minimap_settings_state
                            .read()
                            .settings
                            .minimap_settings
                            .monuments,
                    )
                    .shops(
                        minimap_settings_state
                            .read()
                            .settings
                            .minimap_settings
                            .shops,
                    ),
            )
    }
}
