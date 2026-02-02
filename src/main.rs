#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    prelude::*,
    radio::{RadioChannel, RadioStation},
    tray::{
        TrayEvent, TrayIconBuilder,
        dpi::PhysicalPosition,
        menu::{Menu, MenuEvent, MenuItem},
    },
    webview::plugin::WebViewPlugin,
    winit::window::{WindowId, WindowLevel},
};
use futures_lite::StreamExt;
use rustplus_rs::{AppInfo, AppMap, AppMapMarkers, AppTeamInfo};

mod app;
mod components;
mod layouts;
mod pages;
mod utils;

use crate::{
    pages::{MapSettings, Minimap, MinimapSettings, UserData},
    utils::{Poller, ServerData},
};
use app::App;

const ICON: &[u8] = include_bytes!("./freya_icon.png");

fn main() {
    let mut radio_station = RadioStation::create_global(Data::default());
    let mut main_window_id: Option<WindowId> = Option::None;

    let (state_tx, mut state_rx) = futures_channel::mpsc::unbounded::<ChannelSend>();

    radio_station.write_channel(DataChannel::NoUpdate).state_tx = Some(state_tx.clone());

    let tray_icon = || {
        let tray_menu = Menu::new();
        let _ = tray_menu.append(&MenuItem::with_id("show", "Show", true, None));
        let _ = tray_menu.append(&MenuItem::with_id(
            "toggle_minimap",
            "Toggle Minimap",
            true,
            None,
        ));
        let _ = tray_menu.append(&MenuItem::with_id("exit", "Exit", true, None));
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("Freya Tray")
            .with_icon(LaunchConfig::tray_icon(ICON))
            .build()
            .unwrap()
    };
    let tray_handler = move |ev, mut ctx: RendererContext| match ev {
        TrayEvent::Menu(MenuEvent { id }) if id == "show" => {
            if let Some(id) = main_window_id {
                match ctx.windows.get(&id) {
                    Some(window) => {
                        window.window().focus_window();
                        return;
                    }
                    None => {
                        main_window_id = None;
                    }
                };
            }
            main_window_id = Some(ctx.launch_window(WindowConfig::new(AppComponent::new(App { radio_station }))
                .with_size(1200.0, 800.0)
                .with_resizable(false)
                .with_title("Oxide+")
                .with_window_attributes(|mut attributes, _| {
                    #[cfg(target_os = "linux")]
                    {
                        use crate::utils::{SystemType, get_system_type};

                        match get_system_type() {
                            SystemType::Wayland => {
                                use freya::winit::platform::wayland::WindowAttributesExtWayland;
                                attributes = attributes.with_name("oxide_plus", "oxide_plus2");
                            }
                            SystemType::X11 => {
                                use freya::winit::platform::x11::WindowAttributesExtX11;
                                attributes = attributes.with_name("oxide_plus", "oxide_plus2");
                            }
                            _ => {}
                        }
                    }

                    attributes
                }
            )));
        }
        TrayEvent::Menu(MenuEvent { id }) if id == "toggle_minimap" => {
            let minimap_state = radio_station.read().settings.minimap_settings.enabled;
            radio_station
                .read()
                .state_tx
                .clone()
                .unwrap()
                .unbounded_send(ChannelSend::ToggleMinimap(!minimap_state))
                .unwrap();
        }
        TrayEvent::Menu(MenuEvent { id }) if id == "exit" => {
            ctx.exit();
        }
        _ => {}
    };

    launch(
        LaunchConfig::new()
            .with_font(
                "WDXL Lubrifont",
                Bytes::from_static(include_bytes!("./assets/WDXLLubrifontSC-Regular.ttf")),
            )
            .with_font(
                "PermanentMarker",
                Bytes::from_static(include_bytes!("./assets/PermanentMarker-Regular.ttf")),
            )
            .with_font(
                "Roboto Condensed",
                Bytes::from_static(include_bytes!("./assets/RobotoCondensed-Variable.ttf")),
            )
            .with_default_font("WDXL Lubrifont")
            .with_future(move |proxy| async move {
                let mut poller = Poller::new(None, state_tx.clone());

                let mut window_id: Option<WindowId> = None;

                while let Some(channel_data) = state_rx.next().await {
                    match channel_data {
                        ChannelSend::UserDataUpdate(user_data) => {
                            radio_station
                                .write_channel(DataChannel::UserDataUpdate)
                                .user_data = Some(user_data);
                        }
                        ChannelSend::LoadingStateUpdate(loading_state) => {
                            radio_station
                                .write_channel(DataChannel::LoadingStateUpdate)
                                .loading_state = loading_state;
                        }
                        ChannelSend::ServersUpdate(servers) => {
                            radio_station
                                .write_channel(DataChannel::ServersUpdate)
                                .servers = servers;
                        }
                        ChannelSend::AddServer(server) => {
                            radio_station
                                .write_channel(DataChannel::ServersUpdate)
                                .servers
                                .push(server);
                        }
                        ChannelSend::SelectedServerUpdate(selected_server) => {
                            match &selected_server {
                                Some(server) => {
                                    poller.update_details(Some(server.clone()));
                                }
                                None => {
                                    poller.update_details(None);
                                }
                            }
                            radio_station
                                .write_channel(DataChannel::SelectedServerUpdate)
                                .selected_server = selected_server;
                        }
                        ChannelSend::InfoStateUpdate(info_state) => {
                            radio_station
                                .write_channel(DataChannel::InfoStateUpdate)
                                .info_state = info_state;
                        }
                        ChannelSend::MapStateUpdate(map_state) => {
                            radio_station
                                .write_channel(DataChannel::MapStateUpdate)
                                .map_state = map_state;
                        }
                        ChannelSend::MapMarkersUpdate(map_markers) => {
                            radio_station
                                .write_channel(DataChannel::MapMarkersUpdate)
                                .map_markers = map_markers;
                        }
                        ChannelSend::TeamInfoUpdate(team_info) => {
                            radio_station
                                .write_channel(DataChannel::TeamInfoUpdate)
                                .team_info = team_info;
                        }
                        ChannelSend::ToggleMinimap(toggle) => {
                            radio_station
                                .write_channel(DataChannel::MinimapSettingsUpdate)
                                .settings.minimap_settings.enabled = toggle;
                            if toggle {
                                if let Some(id) = window_id {
                                    match proxy.with(move |ctx| {
                                        match ctx.windows().get(&id) {
                                            Some(window) => {
                                                window.window().focus_window();
                                            }
                                            None => {
                                                println!("Window not found");
                                            }
                                        };
                                    }).await {
                                        Ok(_) => {},
                                        Err(err) => {
                                            println!("Error focusing window: {}", err);
                                        }
                                    };
                                } else {
                                    match proxy.with(move |ctx| {
                                        let minimap_settings = radio_station.read().settings.minimap_settings.clone();
                                        ctx.launch_window(
                                            WindowConfig::new(move || {
                                                use_provide_context(move || radio_station);

                                                Minimap::new()
                                            })
                                            .with_size(minimap_settings.size as f64, minimap_settings.size as f64)
                                            .with_background(Color::TRANSPARENT)
                                            .with_transparency(true)
                                            .with_decorations(false)
                                            .with_resizable(false)
                                            .with_title("Oxide+ - Minimap")
                                            .with_window_attributes({
                                                move |mut attributes, _| {
                                                    attributes = attributes
                                                        .with_window_level(WindowLevel::AlwaysOnTop)
                                                        .with_position(PhysicalPosition::new(
                                                            0.0, 0.0,
                                                        ));

                                                    #[cfg(not(target_os = "linux"))]
                                                    {
                                                        use freya::winit::{dpi::PhysicalPosition, platform::windows::WindowAttributesExtWindows, window::WindowLevel};

                                                        attributes = attributes
                                                            .with_skip_taskbar(true);
                                                    }

                                                    #[cfg(target_os = "linux")]
                                                    {
                                                        use crate::utils::{SystemType, get_system_type};

                                                        match get_system_type() {
                                                            SystemType::Wayland => {
                                                                use freya::winit::platform::wayland::WindowAttributesExtWayland;
                                                                attributes = attributes.with_name("oxide_plus", "oxide_plus2");
                                                            }
                                                            SystemType::X11 => {
                                                                use freya::winit::platform::x11::WindowAttributesExtX11;
                                                                attributes = attributes.with_name("oxide_plus", "oxide_plus2");
                                                            }
                                                            _ => {}
                                                        }
                                                    }

                                                    attributes
                                                }
                                            }),
                                        )
                                    }).await {
                                        Ok(new_window_id) => {
                                            window_id = Some(new_window_id);
                                        }
                                        Err(err) => {
                                            panic!("Could not create window. Error: {:?}", err);
                                        }
                                    };
                                }
                            } else {
                                if let Some(id) = window_id {
                                    match proxy.with(move |ctx| {
                                        match ctx.windows.remove(&id) {
                                            Some(_) => {
                                                println!("Window removed");
                                                Ok(())
                                            }
                                            None => {
                                                println!("Window not found");
                                                Err(())
                                            }
                                        }
                                    }).await {
                                        Ok(_) => {
                                            window_id = None;
                                        },
                                        Err(err) => {
                                            println!("Error focusing window: {}", err);
                                        }
                                    };
                                }
                            }
                        }
                    }
                }
            })
        .with_plugin(WebViewPlugin::new())
        //.with_tray(tray_icon, tray_handler)
        .with_window(WindowConfig::new(AppComponent::new(App { radio_station }))
        .with_size(1200.0, 800.0)
        .with_resizable(false)
        .with_title("Oxide+")
        .with_window_attributes(|mut attributes, _| {
            #[cfg(target_os = "linux")]
            {
                use crate::utils::{SystemType, get_system_type};

                match get_system_type() {
                    SystemType::Wayland => {
                        use freya::winit::platform::wayland::WindowAttributesExtWayland;
                        attributes = attributes.with_name("oxide_plus", "oxide_plus");
                    }
                    SystemType::X11 => {
                        use freya::winit::platform::x11::WindowAttributesExtX11;
                        attributes = attributes.with_name("oxide_plus", "oxide_plus");
                    }
                    _ => {}
                }
            }

            attributes
        })),
    );
}

#[derive(Default, Clone)]
pub struct Settings {
    pub map_settings: MapSettings,
    pub minimap_settings: MinimapSettings,
}

#[derive(Default)]
pub struct Data {
    pub user_data: Option<UserData>,
    pub servers: Vec<ServerData>,
    pub selected_server: Option<ServerData>,
    pub loading_state: String,
    pub settings: Settings,

    pub info_state: Option<AppInfo>,
    pub map_state: Option<AppMap>,
    pub map_markers: Option<AppMapMarkers>,
    pub team_info: Option<AppTeamInfo>,

    pub state_tx: Option<futures_channel::mpsc::UnboundedSender<ChannelSend>>,
    pub minimap_window_id: Option<WindowId>,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum DataChannel {
    NoUpdate,
    StateTxUpdate,
    UserDataUpdate,
    LoadingStateUpdate,
    ServersUpdate,
    SelectedServerUpdate,
    InfoStateUpdate,
    MapStateUpdate,
    MapMarkersUpdate,
    TeamInfoUpdate,
    SettingsUpdate,
    MapSettingsUpdate,
    MinimapSettingsUpdate,
}

impl RadioChannel<Data> for DataChannel {}

pub enum ChannelSend {
    UserDataUpdate(UserData),
    LoadingStateUpdate(String),
    ServersUpdate(Vec<ServerData>),
    AddServer(ServerData),
    SelectedServerUpdate(Option<ServerData>),
    InfoStateUpdate(Option<AppInfo>),
    MapStateUpdate(Option<AppMap>),
    MapMarkersUpdate(Option<AppMapMarkers>),
    TeamInfoUpdate(Option<AppTeamInfo>),
    ToggleMinimap(bool),
}
