#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod components;

use freya::prelude::*;
use app::App;

fn main() {
    launch_cfg(
        LaunchConfig::new().with_window(
            WindowConfig::new(App)
                .with_size(1200.0, 800.0)
                .on_setup(|window| {
                    window.set_title("RustPlus Login");
                    window.set_resizable(true);
                })
                .with_window_attributes(|attributes| attributes.with_resizable(false)),
        ),
    );
}