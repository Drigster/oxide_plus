use std::sync::Arc;

use freya::prelude::*;
use freya_radio::prelude::*;
use freya_router::prelude::{Routable, RouterConfig, RouterContext, router};
use rustplus_rs::{AppInfo, AppMap, AppMapMarkers, RustPlus};

use crate::{
    layouts::{LoginLayout, MainLayout},
    pages::{Info, Map, ServerSelect, Shops, Team},
    utils::settings::{ServerData, load_servers},
};

#[derive(Default)]
pub struct Data {
    pub map_state: Option<AppMap>,
    pub info_state: Option<AppInfo>,
    pub map_markers: Option<AppMapMarkers>,
    pub connection_state: String,
    pub error_state: String,
    pub servers: Vec<ServerData>,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum DataChannel {
    NoUpdate,
    MapStateUpdate,
    InfoStateUpdate,
    MapMarkersUpdate,
    ConnectionStateUpdate,
    ErrorStateUpdate,
    ServersUpdate,
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
            .background_linear_gradient(
                LinearGradient::new()
                    .angle(0.0)
                    .stop((Color::from_hex("#1D1D1B").unwrap(), 0.0))
                    .stop((Color::from_hex("#0E0E0D").unwrap(), 100.0)),
            )
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
        #[route("/map")]
        Map,
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
                .connection_state = "Connected, loading data.".to_string();

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
                .connection_state = "Connected, loading data..".to_string();

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
                .connection_state = "Connected, loading data...".to_string();

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

            println!("Done...");
            radio
                .write_channel(DataChannel::ConnectionStateUpdate)
                .connection_state = "Done...".to_string();
        })
    });

    router::<Route>(|| RouterConfig::default().with_initial_path(Route::Loading))
}
