#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod components;
mod layouts;
mod pages;
mod utils;

use app::App;
use freya::prelude::*;

#[cfg(target_os = "linux")]
use freya::winit::platform::wayland::WindowAttributesExtWayland;

fn main() {
    launch(
        LaunchConfig::new()
            .with_default_font("WDXL Lubrifont")
            .with_font(
                "WDXL Lubrifont",
                Bytes::from_static(include_bytes!("./assets/WDXLLubrifontSC-Regular.ttf")),
            )
            .with_font(
                "PermanentMarker",
                Bytes::from_static(include_bytes!("./assets/PermanentMarker-Regular.ttf")),
            )
            .with_window(
                WindowConfig::new(App)
                    .with_size(1200.0, 800.0)
                    .with_resizable(false)
                    .with_title("Oxide+")
                    .with_window_attributes(|window_attributes| {
                        if cfg!(target_os = "linux") {
                            return window_attributes.with_name("oxide_plus", "oxide_plus");
                        }
                        window_attributes
                    }),
            ),
    );
}
