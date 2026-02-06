#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, hash::Hash};

use freya::{
    prelude::*,
    radio::{RadioChannel, RadioStation},
    tray::{
        TrayEvent, TrayIconBuilder,
        dpi::PhysicalPosition,
        menu::{Menu, MenuEvent, MenuItem},
    },
    webview::WebViewPlugin,
    winit::window::{Icon, WindowId, WindowLevel},
};
use futures_lite::StreamExt;
use rustplus_rs::{
    AppInfo, AppMap, AppMapMarkers, AppTeamInfo,
    app_team_info::{Member, Note},
};

mod app;
mod components;
mod layouts;
mod pages;
mod utils;

use crate::{
    pages::{MapSettings, Minimap, MinimapSettings, UserData},
    utils::{Poller, Profile, ServerData, get_profile_pic},
};
use app::MyApp;

const ICON: &[u8] = include_bytes!("./assets/oxide_plus_icon.png");
const SELECT_COLOR: &str = "#135C4F";
const ACCENT_COLOR: &str = "#13455C";
const SIDEBAR_BUTTON_BACKGROUND: &str = "#00000066";
const SIDEBAR_BUTTON_BACKGROUND_HOVER: &str = "#FFFFFF1A";
const TEXT_COLOR: &str = "#E4DAD1";
const ICON_COLOR: &str = "#605B55";
const BORDER_COLOR: &str = "#393834";

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
            .with_tooltip("Oxide+")
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
            main_window_id = Some(ctx.launch_window(
            WindowConfig::new_app(MyApp { radio_station })
                .with_size(1200.0, 800.0)
                .with_resizable(false)
                .with_title("Oxide+")
                .with_icon(LaunchConfig::window_icon(ICON))
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
                })
            ));
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
                                .info_state = if let Some(info_state) = info_state {
                                    InfoState {
                                        name: Some(info_state.name),
                                        header_image: Some(info_state.header_image),
                                        url: Some(info_state.url),
                                        map: Some(info_state.map),
                                        map_size: Some(info_state.map_size),
                                        wipe_time: Some(info_state.wipe_time),
                                        players: Some(info_state.players),
                                        max_players: Some(info_state.max_players),
                                        queued_players: Some(info_state.queued_players),
                                        seed: Some(info_state.seed),
                                        salt: Some(info_state.salt),
                                        logo_image: Some(info_state.logo_image),
                                        nexus: Some(info_state.nexus),
                                        nexus_id: Some(info_state.nexus_id),
                                        nexus_zone: Some(info_state.nexus_zone),
                                        cameras_enabled: Some(info_state.cameras_enabled),
                                    }
                                } else {
                                    InfoState::default()
                                };
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
                                .team_info = if let Some(team_info) = &team_info {
                                    TeamInfo {
                                        leader_steam_id: Some(team_info.leader_steam_id),
                                        members: team_info.members.clone(),
                                        map_notes: team_info.map_notes.clone(),
                                        leader_map_notes: team_info.leader_map_notes.clone(),
                                    }
                                } else {
                                    TeamInfo::default()
                                };

                            if let Some(team_info) = &team_info {
                                let steam_profiles = radio_station.read().steam_profiles.clone();
                                for member in &team_info.members {
                                    if steam_profiles.contains_key(&member.steam_id) {
                                        continue;
                                    }
                                    let steam_profile = get_profile_pic(member.steam_id).await;

                                    if let Ok(steam_profile) = steam_profile {
                                        radio_station
                                            .write_channel(DataChannel::SteamProfileUpdate(member.steam_id))
                                            .steam_profiles
                                            .insert(member.steam_id, steam_profile);
                                    }
                                }
                            }
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
                                            .with_icon(LaunchConfig::window_icon(ICON))
                                            .with_window_attributes({
                                                move |mut attributes, _| {
                                                    attributes = attributes
                                                        .with_window_level(WindowLevel::AlwaysOnTop)
                                                        .with_position(PhysicalPosition::new(
                                                            minimap_settings.offset_x,
                                                            minimap_settings.offset_y
                                                        ));

                                                    #[cfg(not(target_os = "linux"))]
                                                    {
                                                        use freya::winit::platform::windows::WindowAttributesExtWindows;

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
        .with_window(
            WindowConfig::new_app(MyApp { radio_station })
        .with_size(1200.0, 800.0)
        .with_resizable(false)
        .with_title("Oxide+")
            .with_icon(LaunchConfig::window_icon(ICON))
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
            })
        ),
    );
}

#[derive(Default, Clone)]
pub struct Settings {
    pub map_settings: MapSettings,
    pub minimap_settings: MinimapSettings,
}

#[derive(Default)]
pub struct InfoState {
    pub name: Option<String>,
    pub header_image: Option<String>,
    pub url: Option<String>,
    pub map: Option<String>,
    pub map_size: Option<u32>,
    pub wipe_time: Option<u32>,
    pub players: Option<u32>,
    pub max_players: Option<u32>,
    pub queued_players: Option<u32>,
    pub seed: Option<u32>,
    pub salt: Option<u32>,
    pub logo_image: Option<String>,
    pub nexus: Option<String>,
    pub nexus_id: Option<i32>,
    pub nexus_zone: Option<String>,
    pub cameras_enabled: Option<bool>,
}

#[derive(Default)]
pub struct TeamInfo {
    pub leader_steam_id: Option<u64>,
    pub members: Vec<Member>,
    pub map_notes: Vec<Note>,
    pub leader_map_notes: Vec<Note>,
}

#[derive(Default)]
pub struct Data {
    pub user_data: Option<UserData>,
    pub servers: Vec<ServerData>,
    pub selected_server: Option<ServerData>,
    pub loading_state: String,
    pub settings: Settings,

    pub info_state: InfoState,
    pub map_state: Option<AppMap>,
    pub map_markers: Option<AppMapMarkers>,
    pub team_info: TeamInfo,
    pub steam_profiles: HashMap<u64, Profile>,

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
    SteamProfileUpdate(u64),
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
