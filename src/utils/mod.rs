mod auth_utils;

#[cfg(target_os = "linux")]
use std::env;

pub use auth_utils::*;
mod settings;
pub use settings::*;
mod image_utils;
pub use image_utils::*;
mod rustplus_poller;
pub use rustplus_poller::*;
mod text_utils;
pub use text_utils::*;
mod steam_utils;
pub use steam_utils::*;

#[cfg(target_os = "linux")]
pub enum SystemType {
    Wayland,
    X11,
    Headless,
}

#[cfg(target_os = "linux")]
pub fn get_system_type() -> SystemType {
    let is_wayland = env::var("WAYLAND_DISPLAY").is_ok();
    let is_x11 = env::var("DISPLAY").is_ok();

    match (is_wayland, is_x11) {
        (true, _) => SystemType::Wayland,
        (false, true) => SystemType::X11,
        _ => SystemType::Headless,
    }
}
