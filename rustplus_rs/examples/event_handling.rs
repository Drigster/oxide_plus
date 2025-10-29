use rustplus_rs::{RustPlus, RustPlusEvent, RustPlusError};
use tokio;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), RustPlusError> {
    // Initialize logging
    env_logger::init();
    
    // Create a new RustPlus client
    // Replace these values with your actual server details
    let rustplus = RustPlus::new(
        "127.0.0.1",        // Server IP
        28082,              // App port
        76561198000000000,  // Player ID (Steam ID)
        12345,              // Player token
        false,              // Use Facepunch proxy
    ).await?;
    
    // Get the event broadcaster
    let event_broadcaster = rustplus.event_broadcaster();
    
    // Spawn a task to handle events
    let mut event_receiver = event_broadcaster.subscribe();
    tokio::spawn(async move {
        while let Ok(event) = event_receiver.recv().await {
            match event {
                RustPlusEvent::Connecting => {
                    println!("🔄 Connecting to Rust server...");
                }
                RustPlusEvent::Connected => {
                    println!("✅ Connected to Rust server!");
                }
                RustPlusEvent::Disconnected => {
                    println!("❌ Disconnected from Rust server");
                }
                RustPlusEvent::Error(error) => {
                    println!("⚠️  Error: {}", error);
                }
                RustPlusEvent::Message(message) => {
                    println!("📨 Received message: {:?}", message);
                }
                RustPlusEvent::Request(request) => {
                    println!("📤 Sent request: seq={}", request.seq);
                }
            }
        }
    });
    
    // Connect to the server
    rustplus.connect().await?;
    
    // Send a team message
    rustplus.send_team_message("Hello from event handling example!").await?;
    
    // Get some info to generate events
    let _info = rustplus.get_info().await?;
    let _time = rustplus.get_time().await?;
    
    // Wait a bit to see events
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Disconnect
    rustplus.disconnect().await?;
    
    // Wait a bit more to see disconnect event
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    Ok(())
}
