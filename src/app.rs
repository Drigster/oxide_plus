use std::sync::Arc;

use freya::prelude::*;
use freya_radio::prelude::*;
use freya_router::prelude::{Routable, RouterConfig, RouterContext, outlet, router};
use rustplus_rs::{AppInfo, AppMap, AppMapMarkers, RustPlus};

use crate::{components::Navbar, components::Sidebar, pages::Map, pages::Team};

#[derive(Default)]
pub struct Data {
    pub map_state: Option<AppMap>,
    pub info_state: Option<AppInfo>,
    pub map_markers: Option<AppMapMarkers>,
    pub connection_state: String,
    pub error_state: String,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum DataChannel {
    NoUpdate,
    MapStateUpdate,
    InfoStateUpdate,
    MapMarkersUpdate,
    ConnectionStateUpdate,
    ErrorStateUpdate,
}

impl RadioChannel<Data> for DataChannel {}

#[derive(PartialEq)]
struct Settings {}
impl Render for Settings {
    fn render(&self) -> Element {
        Button::new()
            .on_press(|_| {
                RouterContext::get().replace(Route::Map);
            })
            .child("Go Map")
            .into()
    }
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Map,
        #[route("/team")]
        Team,
        #[route("/settings")]
        Settings,
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    use_init_radio_station::<Data, DataChannel>(Data::default);

    router::<Route>(|| RouterConfig::default().with_initial_path(Route::Settings))
}

#[derive(PartialEq)]
struct Layout;
impl Render for Layout {
    fn render(&self) -> Element {
        let mut radio = use_radio::<Data, DataChannel>(DataChannel::NoUpdate);

        use_hook(|| {
            spawn(async move {
                println!("Creating RustPlus client...");
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
                    .connection_state = "Connected".to_string();

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
            })
        });

        rect()
            .width(Size::percent(100.0))
            .height(Size::percent(100.0))
            .background(Color::from_hex("#1e1e1e").unwrap())
            .children([
                Navbar::new().into(),
                rect()
                    .width(Size::percent(100.0))
                    .height(Size::Fill)
                    .background_linear_gradient(
                        LinearGradient::new()
                            .angle(0.0)
                            .stop((Color::from_hex("#1D1D1B").unwrap(), 0.0))
                            .stop((Color::from_hex("#0E0E0D").unwrap(), 100.0)),
                    )
                    .direction(Direction::Horizontal)
                    .children([
                        Sidebar::new().into(),
                        rect()
                            .height(Size::percent(100.0))
                            .width(Size::Fill)
                            .padding(8.0)
                            .child(outlet::<Route>())
                            .into(),
                    ])
                    .into(),
            ])
            .into()
    }
}
