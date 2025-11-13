use freya::prelude::*;
use freya_router::prelude::{RouterContext, outlet};

use crate::{app::Route, components::Button};

#[derive(PartialEq)]
pub struct MapLayout {}
impl Render for MapLayout {
    fn render(&self) -> Element {
        let mut grid = use_state(|| true);
        let mut teammates = use_state(|| true);
        let mut deaths = use_state(|| true);
        let mut monuments = use_state(|| true);
        let mut shops = use_state(|| true);

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
                                RouterContext::get().replace(Route::MinimapSettings);
                            })
                            .active(
                                RouterContext::get().current::<Route>() == Route::MinimapSettings,
                            )
                            .into(),
                    ])
                    .into(),
                outlet::<Route>(),
            ])
            .into()
    }
}

fn get_text_size_concise(scale: f32) -> f32 {
    return 8.864 / scale + 2.446;
}
