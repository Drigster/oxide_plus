use std::sync::Arc;

use freya::prelude::*;
use freya_radio::prelude::*;
use freya_router::prelude::{Routable, RouterConfig, RouterContext, router};
use rustplus_rs::{AppInfo, AppMap, AppMapMarkers, AppTeamInfo, RustPlus};

use crate::{
    layouts::{LoginLayout, MainLayout, MapLayout},
    pages::{
        Info, Map, MapSettings, MinimapSettings, MinimapSettingsPage, ServerSelect, Shops, Team,
    },
    utils::settings::{ServerData, load_servers},
};

#[derive(Default, Clone)]
pub struct Settings {
    pub map_settings: MapSettings,
    pub minimap_settings: MinimapSettings,
}

#[derive(Default)]
pub struct Data {
    pub map_state: Option<AppMap>,
    pub info_state: Option<AppInfo>,
    pub map_markers: Option<AppMapMarkers>,
    pub team_info: Option<AppTeamInfo>,
    pub connection_state: String,
    pub error_state: String,
    pub servers: Vec<ServerData>,

    pub settings: Settings,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum DataChannel {
    NoUpdate,
    MapStateUpdate,
    InfoStateUpdate,
    MapMarkersUpdate,
    TeamInfoUpdate,
    ConnectionStateUpdate,
    ErrorStateUpdate,
    ServersUpdate,
    MapSettingsUpdate,
    MinimapSettingsUpdate,
}

impl RadioChannel<Data> for DataChannel {}

#[derive(PartialEq)]
struct Loading {}
impl Render for Loading {
    fn render(&self) -> Element {
        rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            .children([
                label()
                    .font_size(20.0)
                    .font_weight(FontWeight::BOLD)
                    .color(Color::from_hex("#E4DAD1").unwrap())
                    .text("Loading...")
                    .into(),
                Button::new()
                    .on_press(|_| {
                        RouterContext::get().replace(Route::ServerSelect);
                    })
                    .child("Go Map")
                    .into(),
            ])
            .into()
    }
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(LoginLayout)]
        #[route("/")]
        Loading,
        #[route("/select_server")]
        ServerSelect,
    #[end_layout]
    #[layout(MainLayout)]
        #[route("/info")]
        Info,
        #[nest("/map")]
            #[layout(MapLayout)]
                #[route("/")]
                Map,
                #[route("/minimap_settings")]
                MinimapSettingsPage,
            #[end_layout]
        #[end_nest]
        #[route("/team")]
        Team,
        #[route("/shops")]
        Shops,
        // #[route("/settings")]
        // Settings,
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    use_init_radio_station::<Data, DataChannel>(Data::default);
    let mut radio = use_radio::<Data, DataChannel>(DataChannel::NoUpdate);

    use_hook(|| {
        spawn(async move {
            let servers = load_servers().unwrap();
            radio.write_channel(DataChannel::ServersUpdate).servers = servers;
        })
    });

    // use_side_effect(move || {
    //     let radio = use_radio::<Data, DataChannel>(DataChannel::ServersUpdate);

    //     // TODO: Add error handling. Toast maybe?
    //     let _ = save_servers(radio.read().servers.clone());
    // });

    use_hook(|| {
        spawn(async move {
            println!("Creating RustPlus client...");
            radio
                .write_channel(DataChannel::ConnectionStateUpdate)
                .connection_state = "Setting up...".to_string();
            // Create a new RustPlus client
            let rustplus = Arc::new(
                match RustPlus::new(
                    "64.40.9.133",     // Your server IP
                    28082,             // Your app.port
                    76561198157374883, // Your Steam ID
                    1405224200,        // Your player token
                    false,             // Use Facepunch proxy
                )
                .await
                {
                    Ok(client) => client,
                    Err(e) => {
                        let err_msg = format!("Failed to create RustPlus client: {}", e);
                        println!("Error: {}", err_msg);
                        radio
                            .write_channel(DataChannel::ErrorStateUpdate)
                            .error_state = err_msg;
                        return;
                    }
                },
            );

            // Connect to the server
            println!("Connecting to Rust server...");

            radio
                .write_channel(DataChannel::ConnectionStateUpdate)
                .connection_state = "Connecting...".to_string();
            if let Err(e) = rustplus.connect().await {
                let err_msg = format!("Failed to connect to server: {}", e);
                println!("Error: {}", err_msg);
                radio
                    .write_channel(DataChannel::ErrorStateUpdate)
                    .error_state = err_msg;
                return;
            }

            println!("✅ Connected to Rust server!");
            radio
                .write_channel(DataChannel::ConnectionStateUpdate)
                .connection_state = "Connected, loading data".to_string();

            println!("Requesting server data...");
            match rustplus.get_info().await {
                Ok(info) => {
                    radio.write_channel(DataChannel::InfoStateUpdate).info_state = Some(info);
                }
                Err(e) => {
                    let err_msg = format!("Failed to get map data: {}", e);
                    println!("Error: {}", err_msg);
                    radio
                        .write_channel(DataChannel::ErrorStateUpdate)
                        .error_state = err_msg;
                }
            }
            radio
                .write_channel(DataChannel::ConnectionStateUpdate)
                .connection_state = "Connected, loading data.".to_string();

            // Get map data
            println!("Requesting map data...");
            match rustplus.get_map().await {
                Ok(map) => {
                    println!("✅ Map data received: {} bytes", map.jpg_image.len());
                    radio.write_channel(DataChannel::MapStateUpdate).map_state = Some(map);
                }
                Err(e) => {
                    let err_msg = format!("Failed to get map data: {}", e);
                    println!("Error: {}", err_msg);
                    radio
                        .write_channel(DataChannel::ErrorStateUpdate)
                        .error_state = err_msg;
                }
            }

            println!("Requesting markers data...");
            radio
                .write_channel(DataChannel::ConnectionStateUpdate)
                .connection_state = "Connected, loading data..".to_string();

            match rustplus.get_map_markers().await {
                Ok(markers) => {
                    radio
                        .write_channel(DataChannel::MapMarkersUpdate)
                        .map_markers = Some(markers);
                }
                Err(e) => {
                    let err_msg = format!("Failed to get map data: {}", e);
                    println!("Error: {}", err_msg);
                    radio
                        .write_channel(DataChannel::ErrorStateUpdate)
                        .error_state = err_msg;
                }
            }

            println!("Requesting team data...");
            radio
                .write_channel(DataChannel::ConnectionStateUpdate)
                .connection_state = "Connected, loading data...".to_string();

            match rustplus.get_team_info().await {
                Ok(team_info) => {
                    radio.write_channel(DataChannel::TeamInfoUpdate).team_info = Some(team_info);
                }
                Err(e) => {
                    let err_msg = format!("Failed to get map data: {}", e);
                    println!("Error: {}", err_msg);
                    radio
                        .write_channel(DataChannel::ErrorStateUpdate)
                        .error_state = err_msg;
                }
            }

            println!("Done...");
            radio
                .write_channel(DataChannel::ConnectionStateUpdate)
                .connection_state = "Done...".to_string();

            loop {
                match rustplus.get_map_markers().await {
                    Ok(markers) => {
                        radio
                            .write_channel(DataChannel::MapMarkersUpdate)
                            .map_markers = Some(markers);
                    }
                    Err(e) => {
                        let err_msg = format!("Failed to get map data: {}", e);
                        println!("Error: {}", err_msg);
                        radio
                            .write_channel(DataChannel::ErrorStateUpdate)
                            .error_state = err_msg;
                    }
                }

                async_std::task::sleep(std::time::Duration::from_secs(5)).await;
            }
        })
    });

    router::<Route>(|| RouterConfig::default().with_initial_path(Route::Loading))
}
