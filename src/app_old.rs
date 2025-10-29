use freya::prelude::*;
use rustplus_rs::{AppInfo, AppMap, RustPlus, RustPlusEvent};
use std::sync::Arc;

#[allow(non_snake_case)]
pub fn App() -> Element {
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

    let message = if let Some(error) = error_state.read().as_ref() {
        rsx!(
            label {
                color: "red",
                "{error}"
            }
        )
    } else if !*connection_state.read() {
        rsx!(
            label {
                color: "yellow",
                "Connecting to Rust server..."
            }
        )
    } else if map_state.read().is_none() {
        rsx!(
            label {
                color: "yellow",
                "Loading map data..."
            }
        )
    } else {
        rsx!(
            label {
                color: "green",
                "Map data loaded successfully!"
            }
        )
    };

    rsx!(
        rect {
            background: "rgb(51, 51, 51)",
            padding: "10",
            width: "100%",
            height: "100%",
            direction: "vertical",
            
            rect {
                direction: "horizontal",
                label {
                    color: "white",
                    "Status: "
                }
                {message}
            }

            // Show map image if available
            if let Some(map) = map_state.read().as_ref() {
                
                rect {
                    direction: "vertical",
                    width: "100%",
                    height: "100%",
                    
                    image {
                        image_data: dynamic_bytes(map.jpg_image.clone()),
                        width: "100%"
                    }
                    
                    rect {
                        padding: "10",
                        direction: "vertical",
                        background: "rgb(40, 40, 40)",
                        
                        if let Some(info) = info_state.read().as_ref() {
                            label { "Info: " }
                            label { {info.map.clone()} }
                            label { {info.map_size.to_string()} }
                        }

                        label { color: "white", "Map Information:" }
                        label { color: "white", {format!("Width: {} Height: {}", map.width, map.height)} }
                        label { color: "white", {format!("Ocean Margin: {}", map.ocean_margin)} }
                        if let Some(bg) = &map.background {
                            label { color: "white", {format!("Background: {}", bg)} }
                        }
                        
                        label { color: "white", "Monuments:" }
                        for monument in &map.monuments {
                            label {
                                color: "white",
                                {format!("• {} (x: {:.1}, y: {:.1})",
                                        monument.token,
                                        monument.x,
                                        monument.y)}
                            }
                        }
                    }
                }
            }
        }
    )
}