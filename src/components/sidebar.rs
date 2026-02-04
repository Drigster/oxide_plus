use freya::prelude::*;
use freya_router::prelude::RouterContext;

use crate::ACCENT_COLOR;
use crate::app::Route;
use crate::components::Button;

#[derive(Clone, PartialEq)]
pub struct Sidebar {}

impl Sidebar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Sidebar {
    fn render(&self) -> impl IntoElement {
        rect()
            .height(Size::percent(100.0))
            .width(Size::px(250.0))
            .padding(8.0)
            .spacing(4.0)
            .background_linear_gradient(
                LinearGradient::new()
                    .angle(0.0)
                    .stop((Color::from_hex(ACCENT_COLOR).unwrap().with_a(13), 5.0))
                    .stop((Color::from_hex(ACCENT_COLOR).unwrap(), 100.0)),
            )
            .children([
                Button::new()
                    .width(Size::Fill)
                    .height(Size::px(40.0))
                    .icon(freya_icons::lucide::info())
                    .text("INFO")
                    .on_press(move |_| {
                        RouterContext::get().replace(Route::Info);
                    })
                    .active(RouterContext::get().current::<Route>() == Route::Info)
                    .into(),
                Button::new()
                    .width(Size::Fill)
                    .height(Size::px(40.0))
                    .icon(freya_icons::lucide::map())
                    .text("MAP")
                    .on_press(move |_| {
                        RouterContext::get().replace(Route::Map);
                    })
                    .active(RouterContext::get().full_route_string().starts_with("/map"))
                    .into(),
                // Button::new()
                //     .width(Size::Fill)
                //     .height(Size::px(40.0))
                //     .icon(freya_icons::lucide::store())
                //     .text("SHOPS")
                //     .on_press(move |_| {
                //         RouterContext::get().replace(Route::Shops);
                //     })
                //     .active(RouterContext::get().current::<Route>() == Route::Shops)
                //     .into(),
                // Button::new()
                //     .width(Size::Fill)
                //     .height(Size::px(40.0))
                //     .icon(freya_icons::lucide::users_round())
                //     .text("TEAM")
                //     .on_press(move |_| {
                //         RouterContext::get().replace(Route::Team);
                //     })
                //     .active(RouterContext::get().current::<Route>() == Route::Team)
                //     .into(),
            ])
    }
}
