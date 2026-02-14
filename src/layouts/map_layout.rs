use freya::{
    animation::{AnimNum, Ease, use_animation},
    prelude::*,
    radio::use_radio,
    router::Outlet,
};
use freya_router::prelude::RouterContext;

use crate::{
    Data, DataChannel,
    app::Route,
    components::{Button, Timeout},
    utils::{create_toast, save_minimap_settings},
};

#[derive(PartialEq)]
pub struct MapLayout {}
impl Component for MapLayout {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio::<Data, DataChannel>(DataChannel::MapSettingsUpdate);

        let mut animation = use_animation(|_| AnimNum::new(0., 100.).ease(Ease::InOut).time(200));

        use_side_effect(move || {
            if RouterContext::get().current::<Route>() == Route::Map {
                if *animation.has_run_yet().peek() == false {
                    animation.finish();
                } else if animation.peek().value() != 100.0 {
                    animation.start();
                }
            } else if animation.peek().value() != 0.0 {
                animation.reverse();
            }
        });

        let value = animation.read().value();

        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .padding(8.0)
            .spacing(8.0)
            .children([
                rect()
                    .width(Size::Fill)
                    .height(Size::px(40.0))
                    .spacing(4.0)
                    .direction(Direction::Horizontal)
                    .content(Content::Flex)
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
                            .padding(Gaps::new(0.0, 4.0, 0.0, 0.0))
                            .spacing(4.0)
                            .direction(Direction::Horizontal)
                            .overflow(Overflow::Clip)
                            .visible_width(VisibleSize::inner_percent(value))
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
                        rect().width(Size::flex(1.0)).into(),
                        rect()
                            .overflow(Overflow::Clip)
                            .visible_width(VisibleSize::inner_percent(100.0 - value))
                            .child(
                                Button::new()
                                    .width(Size::px(110.0))
                                    .height(Size::Fill)
                                    .align(Alignment::Center)
                                    .icon(freya_icons::lucide::save())
                                    .text("SAVE")
                                    .on_press(move |_| {
                                        let minimap_settings =
                                            radio.read().settings.minimap_settings.clone();
                                        let toasts = radio
                                            .slice_mut(DataChannel::ToastsUpdate, |s| {
                                                &mut s.toasts
                                            });
                                        match save_minimap_settings(&minimap_settings) {
                                            Ok(_) => {
                                                create_toast(
                                                    toasts.into_writable(),
                                                    "Minimap settings saved".to_string(),
                                                    "Successfully saved minimap settings"
                                                        .to_string(),
                                                    Timeout::Default,
                                                    None::<fn(())>,
                                                );
                                            }
                                            Err(err) => {
                                                create_toast(
                                                    toasts.into_writable(),
                                                    "Minimap settings error".to_string(),
                                                    "Error saving minimap settings".to_string(),
                                                    Timeout::Default,
                                                    None::<fn(())>,
                                                );
                                                eprintln!(
                                                    "Error saving minimap settings: {:?}",
                                                    err
                                                );
                                            }
                                        };
                                    }),
                            )
                            .into(),
                    ])
                    .into(),
                Outlet::<Route>::new().into(),
            ])
    }
}
