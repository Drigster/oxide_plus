use freya::{
    prelude::*,
    radio::use_radio,
    tray::dpi::{PhysicalPosition, PhysicalSize},
};

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
        let radio = use_radio::<Data, DataChannel>(DataChannel::MinimapSettingsUpdate);
        let minimap_settings = radio.slice_current(|s| &s.settings.minimap_settings);
        let grid = radio.slice_current(|s| &s.settings.minimap_settings.grid);
        let markers = radio.slice_current(|s| &s.settings.minimap_settings.markers);
        let deaths = radio.slice_current(|s| &s.settings.minimap_settings.deaths);
        let monuments = radio.slice_current(|s| &s.settings.minimap_settings.monuments);
        let shops = radio.slice_current(|s| &s.settings.minimap_settings.shops);
        let zoom = radio.slice_mut_current(|s| &mut s.settings.minimap_settings.zoom);
        let opacity = radio.slice_current(|s| &s.settings.minimap_settings.opacity);

        let minimap_size = radio.slice(DataChannel::MinimapSettingsUpdate, |s| &s.settings.minimap_settings.size);
        let monitor_size = radio.slice(DataChannel::MonitorSizeUpdate, |s| &s.monitor_size);

        use_side_effect({
            let minimap_settings = minimap_settings.clone();
            move || {
                let monitor_size = monitor_size.read().unwrap_or(PhysicalSize::new(1920, 1080));

                let (offset_x, offset_y) = match minimap_settings.read().position {
                    super::Position::TopLeft => {
                        (
                            minimap_settings.read().offset_x.clone(), 
                            minimap_settings.read().offset_y.clone()
                        )
                    },
                    super::Position::TopRight => {
                        (
                            monitor_size.width as f32 - minimap_size.read().clone() as f32 - minimap_settings.read().offset_x.clone(), 
                            minimap_settings.read().offset_y.clone()
                        )
                    },
                    super::Position::BottomLeft => {
                        (
                            minimap_settings.read().offset_x.clone(), 
                            monitor_size.height as f32 - minimap_size.read().clone() as f32 - minimap_settings.read().offset_y.clone()
                        )
                    },
                    super::Position::BottomRight => {
                        (
                            monitor_size.width as f32 - minimap_size.read().clone() as f32 - minimap_settings.read().offset_x.clone(), 
                            monitor_size.height as f32 - minimap_size.read().clone() as f32 - minimap_settings.read().offset_y.clone()
                        )
                    },
                };
                

                Platform::get().with_window(None, move |window| {
                    window.set_outer_position(PhysicalPosition::new(offset_x, offset_y));
                });
            }
        });

        use_side_effect({
            let minimap_settings = minimap_settings.clone();
            move || {
                let size = minimap_settings.read().size.clone();

                Platform::get().with_window(None, move |window| {
                    let _ = window.request_inner_size(PhysicalSize::new(size, size));
                });
            }
        });

        rect()
            .width(Size::percent(100.0))
            .height(Size::percent(100.0))
            .maybe(minimap_settings.read().shape == Shape::Circle, |rect| {
                rect.corner_radius(1000.0)
            })
            .overflow(Overflow::Clip)
            .maybe_child({
                Some(
                    MapComponent::new()
                        .interactable(false)
                        .center(true)
                        .background_opacity(opacity.into_readable())
                        .zoom(zoom.into_writable())
                        .grid(grid.into_readable())
                        .markers(markers.into_readable())
                        .deaths(deaths.into_readable())
                        .monuments(monuments.into_readable())
                        .shops(shops.into_readable()),
                )
            })
    }
}
