use std::{sync::Arc, vec};

use freya::prelude::*;
use rustplus_rs::{AppInfo, AppMap, RustPlus, RustPlusEvent};

use crate::components::Map;

static PROFILE_ICON: &[u8] = include_bytes!("./Drigster.png");
static CHEVRON_DOWN: &[u8] = include_bytes!("./assets/lucide/chevron-down.png");
static INFO: &[u8] = include_bytes!("./assets/lucide/info_dark.png");
static MAP_DARK: &[u8] = include_bytes!("./assets/lucide/map_dark.png");
static MAP_LIGHT: &[u8] = include_bytes!("./assets/lucide/map_light.png");
static STORE: &[u8] = include_bytes!("./assets/lucide/store_dark.png");
static USERS_ROUND: &[u8] = include_bytes!("./assets/lucide/users-round_dark.png");

#[allow(non_snake_case)]
pub fn App() -> Element {
    let profile_icon = static_bytes(PROFILE_ICON);
    let chevron_down = static_bytes(CHEVRON_DOWN);
    let info = static_bytes(INFO);
    let map_dark = static_bytes(MAP_DARK);
    let map_light = static_bytes(MAP_LIGHT);
    let store = static_bytes(STORE);
    let users_round = static_bytes(USERS_ROUND);

    let mut map_state = use_signal(|| None::<AppMap>);
    let mut info_state = use_signal(|| None::<AppInfo>);
    let mut connection_state = use_signal(|| false);
    let mut error_state = use_signal(|| None::<String>);
    
    use_future(move || async move {
        println!("Creating RustPlus client...");
        // Create a new RustPlus client
        let rustplus = Arc::new(match RustPlus::new(
            "64.40.9.133",     // Your server IP
            28082,             // Your app.port
            76561198157374883, // Your Steam ID
            1405224200,        // Your player token
            false,             // Use Facepunch proxy
        ).await {
            Ok(client) => client,
            Err(e) => {
                let err_msg = format!("Failed to create RustPlus client: {}", e);
                println!("Error: {}", err_msg);
                error_state.set(Some(err_msg));
                return;
            }
        });

        // Subscribe to events before connecting
        println!("Subscribing to events...");
        let mut event_receiver = rustplus.event_broadcaster().subscribe();

        // Connect to the server
        println!("Connecting to Rust server...");
        if let Err(e) = rustplus.connect().await {
            let err_msg = format!("Failed to connect to server: {}", e);
            println!("Error: {}", err_msg);
            error_state.set(Some(err_msg));
            return;
        }
        
        // Wait for the Connected event
        println!("Waiting for connection confirmation...");
        while let Ok(event) = event_receiver.recv().await {
            match event {
                RustPlusEvent::Connected => {
                    println!("✅ Connected to Rust server!");
                    connection_state.set(true);
                    break;
                }
                RustPlusEvent::Error(error) => {
                    let err_msg = format!("Connection error: {}", error);
                    println!("⚠️ {}", err_msg);
                    error_state.set(Some(err_msg));
                    return;
                }
                _ => continue,
            }
        }

        // Get map data
        println!("Requesting map data...");
        match rustplus.get_map().await {
            Ok(map) => {
                println!("✅ Map data received: {} bytes", map.jpg_image.len());
                map_state.set(Some(map));
            }
            Err(e) => {
                let err_msg = format!("Failed to get map data: {}", e);
                println!("Error: {}", err_msg);
                error_state.set(Some(err_msg));
            }
        }

        println!("Requesting map data...");
        match rustplus.get_info().await {
            Ok(info) => {
                info_state.set(Some(info));
            }
            Err(e) => {
                let err_msg = format!("Failed to get map data: {}", e);
                println!("Error: {}", err_msg);
                error_state.set(Some(err_msg));
            }
        }
    });

    rsx!(
        rect { 
            width: "100%",
            height: "100%",
            direction: "column",
            background: "#1e1e1e",

            rect { 
                width: "100%",
                height: "48",
                background: "linear-gradient(0deg, #171715 0%, #11110F 100%)",
                direction: "horizontal",
                main_align: "space-between",
                cross_align: "center",
                border: "0 0 1 0 solid #393834",
                
                rect { 
                    width: "242",
                    height: "fill",
                    padding: "8",
                    margin: "4",
                    background: "#222222",
                    direction: "horizontal",
                    main_align: "space-between",
                    cross_align: "center",

                    label {
                        font_size: "12",
                        font_weight: "bold",
                        color: "#E4DAD1",
                        "Rusty Moose | EU Hapis"
                    }

                    image {
                        image_data: chevron_down,
                        sampling: "trilinear",
                        width: "32",
                        height: "32",
                    }
                }

                rect {
                    direction: "horizontal",
                    padding: "8",
                    spacing: "8",
                    main_align: "center",
                    cross_align: "center",
                    rect {
                        main_align: "center",
                        label {
                            font_size: "12",
                            color: "#E4DAD1",
                            "Drigster"
                        }
                        label { 
                            font_size: "10",
                            color: "#8D8D8D",
                            "76561198157374883"
                        }
                    }
                    rect {
                        corner_radius: "1000",
                        height: "32",
                        width: "32",
                        overflow: "clip",
                        
                        image { 
                            image_data:profile_icon.clone(),
                            sampling: "trilinear",
                        }
                    }
                }
            }

            rect { 
                width: "100%",
                height: "fill",
                background: "linear-gradient(0deg, #1D1D1B 0%, #0E0E0D 100%)",
                direction: "horizontal",
                
                rect { 
                    height: "100%",
                    width: "250",
                    padding: "8",
                    spacing: "4",
                    background: "linear-gradient(0deg, #51241C00 5%, #51241C 100%)",

                    rect {
                        width: "fill",
                        height: "40",
                        background: "#FFFFFF0D",
                        direction: "horizontal",
                        padding: "8",
                        spacing: "8",
                        cross_align: "center",

                        image {
                            image_data: info,
                            sampling: "trilinear",
                        }
                        label {
                            font_size: "15",
                            font_weight: "bold",
                            color: "#E4DAD1",
                            "SERVER"
                        }
                    }

                    rect {
                        width: "fill",
                        height: "40",
                        background: "#5D7238",
                        direction: "horizontal",
                        padding: "8",
                        spacing: "8",
                        cross_align: "center",

                        image {
                            image_data: map_light,
                            sampling: "trilinear",
                        }
                        label {
                            font_size: "15",
                            font_weight: "bold",
                            color: "#E4DAD1",
                            "MAP"
                        }
                    }

                    rect {
                        width: "fill",
                        height: "40",
                        background: "#FFFFFF0D",
                        direction: "horizontal",
                        padding: "8",
                        spacing: "8",
                        cross_align: "center",

                        image {
                            image_data: store,
                            sampling: "trilinear",
                        }
                        label {
                            font_size: "15",
                            font_weight: "bold",
                            color: "#E4DAD1",
                            "SHOPS"
                        }
                    }

                    rect {
                        width: "fill",
                        height: "40",
                        background: "#FFFFFF0D",
                        direction: "horizontal",
                        padding: "8",
                        spacing: "8",
                        cross_align: "center",

                        image {
                            image_data: users_round,
                            sampling: "trilinear",
                        }
                        label {
                            font_size: "15",
                            font_weight: "bold",
                            color: "#E4DAD1",
                            "TEAM"
                        }
                    }
                }

                rect { 
                    height: "100%",
                    width: "fill",
                    padding: "8",
                    
                    if let Some(map) = map_state.read().as_ref() {
                        if let Some(info) = info_state.read().as_ref() {
                            Map{
                                map_state: map.clone(),
                                map_size: info.map_size,
                            }
                        }
                    }
                }
            }
        }
    )
}