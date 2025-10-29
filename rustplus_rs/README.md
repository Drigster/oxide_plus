# RustPlus RS

[![Crates.io](https://img.shields.io/crates/v/rustplus_rs.svg)](https://crates.io/crates/rustplus_rs)
[![Documentation](https://docs.rs/rustplus_rs/badge.svg)](https://docs.rs/rustplus_rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An unofficial Rust library for interacting with Smart Switches, Smart Alarms, and various other devices in the PC game [Rust](https://store.steampowered.com/app/252490/Rust/).

This library communicates with the Rust Game Server via WebSocket using the same protocol as the official Rust+ app, providing a safe and efficient way to control Rust devices from Rust applications.

## Features

- 🎮 **Smart Device Control**: Turn switches/alarms on/off, get entity info
- 💬 **Team Chat**: Send messages to team chat
- 🗺️ **Map & Server Info**: Get map data, server info, time, team info, map markers
- 📹 **Camera Support**: Subscribe to cameras, render frames, control PTZ cameras and auto turrets
- 📡 **Real-time Monitoring**: Monitor entity state changes and team events
- ⚡ **Async/Await Support**: Built with Tokio for high-performance async operations
- 🔒 **Type Safety**: Full Rust type safety with comprehensive error handling
- 📊 **Event System**: Subscribe to connection status, messages, and errors

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustplus_rs = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use rustplus_rs::{RustPlus, RustPlusError};
use tokio;

#[tokio::main]
async fn main() -> Result<(), RustPlusError> {
    // Create a new RustPlus client
    let rustplus = RustPlus::new(
        "127.0.0.1",        // Server IP
        28082,              // App port
        76561198000000000,  // Player ID (Steam ID)
        12345,              // Player token
        false,              // Use Facepunch proxy
    ).await?;
    
    // Connect to the server
    rustplus.connect().await?;
    
    // Send a team message
    rustplus.send_team_message("Hello from RustPlus RS!").await?;
    
    // Turn on a smart switch
    rustplus.turn_smart_switch_on(1234567).await?;
    
    // Get server info
    let info = rustplus.get_info().await?;
    println!("Server: {} ({} players)", info.name, info.players);
    
    // Disconnect
    rustplus.disconnect().await?;
    
    Ok(())
}
```

## Examples

### Smart Switch Control

```rust
use rustplus_rs::{RustPlus, RustPlusError};

#[tokio::main]
async fn main() -> Result<(), RustPlusError> {
    let rustplus = RustPlus::new("127.0.0.1", 28082, 76561198000000000, 12345, false).await?;
    rustplus.connect().await?;
    
    // Turn switch on/off
    rustplus.turn_smart_switch_on(1234567).await?;
    rustplus.turn_smart_switch_off(1234567).await?;
    
    // Get entity info
    let entity_info = rustplus.get_entity_info(1234567).await?;
    println!("Entity type: {:?}", entity_info.entity_type);
    
    rustplus.disconnect().await?;
    Ok(())
}
```

### Camera Control

```rust
use rustplus_rs::{RustPlus, Camera, RustPlusError};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), RustPlusError> {
    let rustplus = Arc::new(RustPlus::new("127.0.0.1", 28082, 76561198000000000, 12345, false).await?);
    rustplus.connect().await?;
    
    // Create camera instance
    let camera = Camera::new(rustplus.clone(), "OILRIG1");
    
    // Subscribe to camera
    camera.subscribe().await?;
    
    // Move camera
    camera.move_camera(camera::buttons::LEFT, 10.0, 0.0).await?;
    
    // Zoom (for PTZ cameras)
    camera.zoom().await?;
    
    // Shoot (for auto turrets)
    if camera.is_auto_turret().await {
        camera.shoot().await?;
    }
    
    camera.unsubscribe().await?;
    rustplus.disconnect().await?;
    Ok(())
}
```

### Event Handling

```rust
use rustplus_rs::{RustPlus, RustPlusEvent, RustPlusError};

#[tokio::main]
async fn main() -> Result<(), RustPlusError> {
    let rustplus = RustPlus::new("127.0.0.1", 28082, 76561198000000000, 12345, false).await?;
    
    // Subscribe to events
    let mut event_receiver = rustplus.event_broadcaster().subscribe();
    tokio::spawn(async move {
        while let Ok(event) = event_receiver.recv().await {
            match event {
                RustPlusEvent::Connected => println!("Connected!"),
                RustPlusEvent::Disconnected => println!("Disconnected!"),
                RustPlusEvent::Message(msg) => println!("Message: {:?}", msg),
                _ => {}
            }
        }
    });
    
    rustplus.connect().await?;
    // ... use the client
    rustplus.disconnect().await?;
    Ok(())
}
```

## API Reference

### Core Client

- `RustPlus::new()` - Create a new client instance
- `connect()` - Connect to the Rust server
- `disconnect()` - Disconnect from the server
- `is_connected()` - Check connection status

### Smart Device Control

- `turn_smart_switch_on(entity_id)` - Turn a smart switch on
- `turn_smart_switch_off(entity_id)` - Turn a smart switch off
- `set_entity_value(entity_id, value)` - Set entity value
- `get_entity_info(entity_id)` - Get entity information

### Communication

- `send_team_message(message)` - Send a message to team chat

### Information Retrieval

- `get_info()` - Get server information
- `get_time()` - Get in-game time
- `get_map()` - Get map data
- `get_team_info()` - Get team information
- `get_map_markers()` - Get map markers

### Camera Control

- `Camera::new()` - Create a camera instance
- `subscribe()` - Subscribe to camera feed
- `unsubscribe()` - Unsubscribe from camera
- `move_camera(buttons, x, y)` - Move camera
- `zoom()` - Zoom camera (PTZ)
- `shoot()` - Shoot (auto turrets)
- `reload()` - Reload (auto turrets)

## Configuration

### Server Setup

To use this library, you need:

1. **Server IP**: The IP address or hostname of your Rust server
2. **App Port**: The port configured in `server.cfg` as `app.port` (default: 28082)
3. **Player ID**: Your Steam ID (64-bit)
4. **Player Token**: Token from server pairing

### Getting Your Credentials

#### As a Server Admin

If you're a server admin, you can find your credentials in the server files:

- **Player Token**: Check `player.tokens.db` with:
  ```bash
  sqlite3 player.tokens.db "select * from data;"
  ```
- **Entity IDs**: Use the `lookingat_debug` command in-game to see entity IDs

#### Using the Official Rust+ App

1. Install the official Rust+ app
2. Pair with your server
3. Use the pairing information in your Rust application

## Error Handling

The library uses Rust's `Result` type for comprehensive error handling:

```rust
use rustplus_rs::{RustPlusError, Result};

async fn handle_rust_operations() -> Result<(), RustPlusError> {
    let rustplus = RustPlus::new("127.0.0.1", 28082, 76561198000000000, 12345, false).await?;
    
    match rustplus.connect().await {
        Ok(_) => println!("Connected successfully!"),
        Err(RustPlusError::NotConnected) => println!("Failed to connect"),
        Err(RustPlusError::Timeout) => println!("Connection timeout"),
        Err(e) => println!("Other error: {}", e),
    }
    
    Ok(())
}
```

## Rate Limits

The Rust server enforces rate limits on requests:

- **Per IP**: 50 tokens limit, 15 tokens replenished per second
- **Per Player**: 25 tokens limit, 3 tokens replenished per second
- **Server Pairing**: 5 tokens limit, 0.1 tokens replenished per second

Request costs:
- Most requests: 1 token
- Map requests: 5 tokens
- Team chat: 2 tokens
- Camera movement: 0.01 tokens

## Connection Limits

- **Max Connections**: 500 (default, configurable with `app.maxconnections`)
- **Max Connections per IP**: 5 (default, configurable with `app.maxconnectionsperip`)

## Examples

Check the `examples/` directory for more detailed examples:

- `basic_usage.rs` - Basic client operations
- `smart_switch_control.rs` - Smart switch control
- `camera_control.rs` - Camera and turret control
- `event_handling.rs` - Event subscription and handling

Run examples with:

```bash
cargo run --example basic_usage
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This is an **unofficial** library and is not affiliated with Facepunch Studios or the official Rust+ app. Use at your own risk.

## Acknowledgments

- Based on the [rustplus.js](https://github.com/liamcottle/rustplus.js) library by Liam Cottle
- Uses the same protocol as the official Rust+ app
- Built with [Tokio](https://tokio.rs/) for async runtime
- Uses [Prost](https://github.com/tokio-rs/prost) for Protocol Buffers

## Changelog

### 0.1.0
- Initial release
- Basic client functionality
- Smart device control
- Camera support
- Event system
- Comprehensive examples
