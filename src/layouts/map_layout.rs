use freya::prelude::*;
use freya_radio::hooks::use_radio;
use freya_router::prelude::{RouterContext, outlet};

use crate::{
    app::{Data, DataChannel, Route},
    components::Button,
};

#[derive(PartialEq)]
pub struct MapLayout {}
impl Render for MapLayout {
    fn render(&self) -> Element {
        let mut map_settings_binding = use_radio::<Data, DataChannel>(DataChannel::MapStateUpdate);
        let settings = &map_settings_binding.read().settings.clone();
        let map_settings = settings.map_settings.clone();

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
                                RouterContext::get().replace(Route::Map);
                            })
                            .active(RouterContext::get().current::<Route>() == Route::Map)
                            .into(),
                        Button::new()
                            .height(Size::Fill)
                            .icon(freya_icons::lucide::grid_2x2())
                            .on_press(move |_| {
                                map_settings_binding.write().settings.map_settings.grid =
                                    !map_settings.grid;
                            })
                            .active(map_settings.grid)
                            .into(),
                        Button::new()
                            .height(Size::Fill)
                            .icon(freya_icons::lucide::map_pin())
                            .on_press(move |_| {
                                map_settings_binding.write().settings.map_settings.markers =
                                    !map_settings.markers;
                            })
                            .active(map_settings.markers)
                            .into(),
                        Button::new()
                            .height(Size::Fill)
                            .icon(freya_icons::lucide::skull())
                            .on_press(move |_| {
                                map_settings_binding.write().settings.map_settings.deaths =
                                    !map_settings.deaths;
                            })
                            .active(map_settings.deaths)
                            .into(),
                        Button::new()
                            .height(Size::Fill)
                            .icon(freya_icons::lucide::factory())
                            .on_press(move |_| {
                                map_settings_binding.write().settings.map_settings.monuments =
                                    !map_settings.monuments;
                            })
                            .active(map_settings.monuments)
                            .into(),
                        Button::new()
                            .height(Size::Fill)
                            .icon(freya_icons::lucide::store())
                            .on_press(move |_| {
                                map_settings_binding.write().settings.map_settings.shops =
                                    !map_settings.shops;
                            })
                            .active(map_settings.shops)
                            .into(),
                        rect().into(),
                        Button::new()
                            .width(Size::px(110.0))
                            .height(Size::Fill)
                            .align(Alignment::Center)
                            .icon(freya_icons::lucide::locate_fixed())
                            .text("MINIMAP")
                            .on_press(move |_| {
                                RouterContext::get().replace(Route::MinimapSettingsPage);
                            })
                            .active(
                                RouterContext::get().current::<Route>()
                                    == Route::MinimapSettingsPage,
                            )
                            .into(),
                    ])
                    .into(),
                outlet::<Route>(),
            ])
            .into()
    }
}
