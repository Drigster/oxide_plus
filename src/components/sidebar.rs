use freya::prelude::*;
use freya_router::prelude::*;

use crate::app::Route;

#[derive(Clone, PartialEq)]
pub struct SidebarButton {
    pub icon: (&'static str, &'static [u8]),
    pub text: String,
    pub target_route: Route,
}

impl SidebarButton {
    pub fn new(icon: (&'static str, &'static [u8]), text: String, target_route: Route) -> Self {
        Self {
            icon,
            text,
            target_route,
        }
    }
}

impl Render for SidebarButton {
    fn render(&self) -> Element {
        let target_route = self.target_route.clone();
        let mut hovered = use_state(|| false);

        let background_color = if RouterContext::get().current::<Route>() == target_route {
            "#5D7238"
        } else if hovered() {
            "#2DFFFFFF"
        } else {
            "#0DFFFFFF"
        };

        rect()
            .width(Size::Fill)
            .height(Size::px(40.0))
            .background(Color::from_hex(background_color).unwrap())
            .direction(Direction::Horizontal)
            .padding(8.0)
            .spacing(8.0)
            .cross_align(Alignment::Center)
            .on_pointer_enter(move |_| {
                *hovered.write() = true;
            })
            .on_pointer_leave(move |_| {
                *hovered.write() = false;
            })
            .on_press(move |_| {
                RouterContext::get().replace(target_route.clone());
            })
            .children([
                ImageViewer::new(self.icon).into(),
                label()
                    .font_size(15.0)
                    .font_weight(FontWeight::BOLD)
                    .color(Color::from_hex("#E4DAD1").unwrap())
                    .text(self.text.clone())
                    .into(),
            ])
            .into()
    }
}

#[derive(Clone, PartialEq)]
pub struct Sidebar {}

impl Sidebar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for Sidebar {
    fn render(&self) -> Element {
        rect()
            .height(Size::percent(100.0))
            .width(Size::px(250.0))
            .padding(8.0)
            .spacing(4.0)
            .background_linear_gradient(
                LinearGradient::new()
                    .angle(0.0)
                    .stop((Color::from_hex("#51241C00").unwrap(), 5.0))
                    .stop((Color::from_hex("#51241C").unwrap(), 100.0)),
            )
            .children([
                SidebarButton::new(INFO, "SERVER".to_string(), Route::Settings).into(),
                SidebarButton::new(MAP_LIGHT, "MAP".to_string(), Route::Map).into(),
                SidebarButton::new(STORE, "SHOPS".to_string(), Route::Settings).into(),
                SidebarButton::new(USERS_ROUND, "TEAM".to_string(), Route::Settings).into(),
            ])
            .into()
    }
}

static INFO: (&'static str, &'static [u8]) = (
    "info_dark",
    include_bytes!("./../assets/lucide/info_dark.png"),
);
static MAP_DARK: (&'static str, &'static [u8]) = (
    "map_dark",
    include_bytes!("./../assets/lucide/map_dark.png"),
);
static MAP_LIGHT: (&'static str, &'static [u8]) = (
    "map_light",
    include_bytes!("./../assets/lucide/map_light.png"),
);
static STORE: (&'static str, &'static [u8]) = (
    "store_dark",
    include_bytes!("./../assets/lucide/store_dark.png"),
);
static USERS_ROUND: (&'static str, &'static [u8]) = (
    "users-round_dark",
    include_bytes!("./../assets/lucide/users-round_dark.png"),
);
