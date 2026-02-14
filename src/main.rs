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
        dpi::{PhysicalPosition, PhysicalSize},
        menu::{Menu, MenuEvent, MenuItem},
    },
    webview::WebViewPlugin,
    winit::window::{ WindowId, WindowLevel},
};
use futures_lite::StreamExt;
use rand::Rng;
use rustplus_rs::{
    AppInfo, AppMap, AppMapMarkers, AppMarker, AppTeamInfo,
    app_map::Monument,
    app_team_info::{Note},
};

mod app;
mod components;
mod layouts;
mod pages;
mod utils;
mod colors;

use crate::{
    components::{Timeout, Toast}, pages::{MapSettings, Minimap, MinimapSettings, UserData}, utils::{Poller, ServerData, load_minimap_settings}
};
use app::MyApp;

const ICON: &[u8] = include_bytes!("./assets/oxide_plus_icon.png");

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

                radio_station.write_channel(DataChannel::MinimapSettingsUpdate).settings.minimap_settings = match load_minimap_settings() {
                    Ok(minimap_settings) => {
                        match minimap_settings {
                            Some(minimap_settings) => minimap_settings,
                            None => MinimapSettings::default(),
                        }
                    },
                    Err(err) => {
                        println!("Error loading minimap settings: {:?}", err);
                        let mut rng = rand::rng();
                            let toast_id: u64 = rng.next_u64();
                            radio_station.write_channel(DataChannel::ToastsUpdate).toasts.insert(
                                toast_id,
                                Toast {
                                    id: toast_id,
                                    title: "Error loading minimap settings".to_string(),
                                    message: "Failed to load minimap settings.".to_string(),
                                    timeout: Timeout::Default,
                                    on_press: None,
                                },
                            );
                        MinimapSettings::default()
                    }
                };

                while let Some(channel_data) = state_rx.next().await {
                    match channel_data {
                        ChannelSend::UserDataUpdate(user_data) => {
                            radio_station
                                .write_channel(DataChannel::UserDataUpdate)
                                .user_data = user_data;
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
                                .map_state = if let Some(map_state) = map_state {
                                    MapState {
                                        width: map_state.width,
                                        height: map_state.height,
                                        jpg_image: map_state.jpg_image,
                                        ocean_margin: map_state.ocean_margin,
                                        monuments: map_state.monuments,
                                        background: Some(map_state.background),
                                    }
                                } else {
                                    MapState::default()
                                };
                        }
                        ChannelSend::MapMarkersUpdate(map_markers) => {
                            let old_map_markers = radio_station
                                .read()
                                .map_markers
                                .clone();

                            if let Some(map_markers) = &map_markers {
                                if old_map_markers.markers != map_markers.markers {
                                    radio_station
                                        .write_channel(DataChannel::MapMarkersUpdate)
                                        .map_markers = MapMarkers {
                                            markers: map_markers.markers.clone(),
                                        }
                                }
                            }
                            else {
                                radio_station
                                    .write_channel(DataChannel::MapMarkersUpdate)
                                    .map_markers = MapMarkers::default();
                            }
                        }
                        ChannelSend::TeamInfoUpdate(team_info) => {
                            let old_team_info = radio_station.read().team_info.clone();

                            if let Some(team_info) = &team_info {
                                if old_team_info.leader_steam_id != Some(team_info.leader_steam_id) {
                                    radio_station
                                        .write_channel(DataChannel::TeamLeaderUpdate)
                                        .team_info
                                        .leader_steam_id = Some(team_info.leader_steam_id);
                                }
                                if old_team_info.map_notes != team_info.map_notes {
                                    radio_station
                                        .write_channel(DataChannel::MapNotesUpdate)
                                        .team_info
                                        .map_notes = team_info.map_notes.clone();
                                }
                                if old_team_info.leader_map_notes != team_info.leader_map_notes {
                                    radio_station
                                        .write_channel(DataChannel::MapNotesUpdate)
                                        .team_info
                                        .leader_map_notes = team_info.leader_map_notes.clone();
                                }
                                let old_members = old_team_info.members.clone();

                                for member in &team_info.members {
                                    let old_member = old_members.get(&member.steam_id);
                                    if old_member.is_none() {
                                        radio_station
                                            .write_channel(DataChannel::TeamMembersUpdate)
                                            .team_info
                                            .members
                                            .insert(member.steam_id, TeamMember {
                                            steam_id: member.steam_id,
                                            name: member.name.clone(),
                                            x: member.x,
                                            y: member.y,
                                            is_online: member.is_online,
                                            spawn_time: member.spawn_time,
                                            is_alive: member.is_alive,
                                            death_time: member.death_time,
                                            profile_icon: None,
                                        });
                                        continue;
                                    }
                                    let old_member = old_member.unwrap();
                                    if member.steam_id == old_member.steam_id
                                    && member.name == old_member.name
                                    && member.x == old_member.x
                                    && member.y == old_member.y
                                    && member.is_online == old_member.is_online
                                    && member.spawn_time == old_member.spawn_time
                                    && member.is_alive == old_member.is_alive
                                    && member.death_time == old_member.death_time {
                                        continue;
                                    }
                                    
                                    radio_station
                                        .write_channel(DataChannel::TeamMemberUpdate(member.steam_id))
                                        .team_info
                                        .members
                                        .insert(member.steam_id, TeamMember {
                                        steam_id: member.steam_id,
                                        name: member.name.clone(),
                                        x: member.x,
                                        y: member.y,
                                        is_online: member.is_online,
                                        spawn_time: member.spawn_time,
                                        is_alive: member.is_alive,
                                        death_time: member.death_time,
                                        profile_icon: old_member.profile_icon.clone(),
                                    });
                                }
                            }
                            else {
                                radio_station
                                    .write_channel(DataChannel::TeamMembersUpdate)
                                    .team_info
                                    .leader_map_notes = Vec::new();
                                radio_station
                                    .write_channel(DataChannel::MapNotesUpdate)
                                    .team_info
                                    .map_notes = Vec::new();
                                radio_station
                                    .write_channel(DataChannel::MapNotesUpdate)
                                    .team_info
                                    .leader_map_notes = Vec::new();
                                radio_station
                                    .write_channel(DataChannel::TeamLeaderUpdate)
                                    .team_info
                                    .leader_steam_id = None;
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
                        ChannelSend::AddToast(toast_data) => {
                            let mut rng = rand::rng();
                            let toast_id: u64 = rng.next_u64();
                            radio_station.write_channel(DataChannel::ToastsUpdate).toasts.insert(
                                toast_id,
                                Toast {
                                    id: toast_id,
                                    title: toast_data.title,
                                    message: toast_data.message,
                                    timeout: toast_data.timeout,
                                    on_press: toast_data.on_press.map(EventHandler::new),
                                },
                            );
                        },
                    }
                }
            })
        .with_plugin(WebViewPlugin::new())
        .with_tray(tray_icon, tray_handler)
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

#[derive(Default, Clone)]
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

#[derive(Clone)]
pub struct TeamMember {
    pub steam_id: u64,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub is_online: bool,
    pub spawn_time: u32,
    pub is_alive: bool,
    pub death_time: u32,
    pub profile_icon: Option<String>,
}

#[derive(Default, Clone)]
pub struct TeamInfo {
    pub leader_steam_id: Option<u64>,
    pub members: HashMap<u64, TeamMember>,
    pub map_notes: Vec<Note>,
    pub leader_map_notes: Vec<Note>,
}

#[derive(Default, Clone, Debug)]
pub struct MapState {
    pub width: u32,
    pub height: u32,
    pub jpg_image: Vec<u8>,
    pub ocean_margin: i32,
    pub monuments: Vec<Monument>,
    pub background: Option<String>,
}

#[derive(Default, Clone, Debug)]
pub struct MapMarkers {
    pub markers: Vec<AppMarker>,
}

#[derive(Default, Clone)]
pub struct Data {
    pub user_data: UserData,
    pub servers: Vec<ServerData>,
    pub selected_server: Option<ServerData>,
    pub loading_state: String,
    pub settings: Settings,

    pub info_state: InfoState,
    pub map_state: MapState,
    pub map_markers: MapMarkers,
    pub team_info: TeamInfo,

    pub state_tx: Option<futures_channel::mpsc::UnboundedSender<ChannelSend>>,
    pub minimap_window_id: Option<WindowId>,

    pub monitor_size: Option<PhysicalSize<u32>>,

    pub toasts: HashMap<u64, Toast>,
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
    TeamLeaderUpdate,
    MapNotesUpdate,
    TeamMembersUpdate,
    TeamMemberUpdate(u64),
    SettingsUpdate,
    MapSettingsUpdate,
    MinimapSettingsUpdate,
    MonitorSizeUpdate,
    ToastsUpdate,
}

impl RadioChannel<Data> for DataChannel {}

pub struct ToastData {
    pub title: String,
    pub message: String,
    pub timeout: Timeout,
    pub on_press: Option<Box<dyn FnMut(()) + Send + 'static>>,
}

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
    AddToast(ToastData),
}
