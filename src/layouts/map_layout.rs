use freya::{prelude::*, radio::use_radio, router::prelude::Outlet};
use freya_router::prelude::RouterContext;

use crate::{Data, DataChannel, app::Route, components::Button};

#[derive(PartialEq)]
pub struct MapLayout {}
impl Component for MapLayout {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio::<Data, DataChannel>(DataChannel::MapSettingsUpdate);

        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .padding(8.0)
            .spacing(4.0)
            .children([
                rect()
                    .width(Size::Fill)
                    .height(Size::px(40.0))
                    .spacing(4.0)
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
                        rect()
                            .spacing(4.0)
                            .direction(Direction::Horizontal)
                            .children([
                                Button::new()
                                    .height(Size::Fill)
                                    .icon(freya_icons::lucide::grid_2x2())
                                    .on_press(move |_| {
                                        let grid = radio.read().settings.map_settings.grid;
                                        radio.write().settings.map_settings.grid = !grid;
                                    })
                                    .active(radio.read().settings.map_settings.grid)
                                    .into(),
                                Button::new()
                                    .height(Size::Fill)
                                    .icon(freya_icons::lucide::map_pin())
                                    .on_press(move |_| {
                                        let markers = radio.read().settings.map_settings.markers;
                                        radio.write().settings.map_settings.markers = !markers;
                                    })
                                    .active(radio.read().settings.map_settings.markers)
                                    .into(),
                                Button::new()
                                    .height(Size::Fill)
                                    .icon(freya_icons::lucide::skull())
                                    .on_press(move |_| {
                                        let deaths = radio.read().settings.map_settings.deaths;
                                        radio.write().settings.map_settings.deaths = !deaths;
                                    })
                                    .active(radio.read().settings.map_settings.deaths)
                                    .into(),
                                Button::new()
                                    .height(Size::Fill)
                                    .icon(freya_icons::lucide::factory())
                                    .on_press(move |_| {
                                        let monuments =
                                            radio.read().settings.map_settings.monuments;
                                        radio.write().settings.map_settings.monuments = !monuments;
                                    })
                                    .active(radio.read().settings.map_settings.monuments)
                                    .into(),
                                Button::new()
                                    .height(Size::Fill)
                                    .icon(freya_icons::lucide::store())
                                    .on_press(move |_| {
                                        let shops = radio.read().settings.map_settings.shops;
                                        radio.write().settings.map_settings.shops = !shops;
                                    })
                                    .active(radio.read().settings.map_settings.shops)
                                    .into(),
                                Button::new()
                                    .height(Size::Fill)
                                    .icon(freya_icons::lucide::locate_fixed())
                                    .on_press(move |_| {
                                        let center = radio.read().settings.map_settings.center;
                                        radio.write().settings.map_settings.center = !center;
                                    })
                                    .active(radio.read().settings.map_settings.center)
                                    .into(),
                            ])
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
                Outlet::<Route>::new().into(),
            ])
    }
}
