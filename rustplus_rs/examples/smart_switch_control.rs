use rustplus_rs::{RustPlus, RustPlusError};
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
    
    // Connect to the server
    println!("Connecting to Rust server...");
    rustplus.connect().await?;
    println!("Connected successfully!");
    
    // Example entity ID - replace with actual smart switch entity ID
    let entity_id = 1234567;
    
    // Get initial entity info
    println!("Getting entity info for entity {}...", entity_id);
    match rustplus.get_entity_info(entity_id).await {
        Ok(entity_info) => {
            println!("Entity type: {:?}", entity_info.entity_type);
            if let Some(value) = entity_info.payload.value {
                println!("Current value: {}", value);
            }
        }
        Err(e) => println!("Failed to get entity info: {}", e),
    }
    
    // Turn the smart switch on
    println!("Turning smart switch on...");
    rustplus.turn_smart_switch_on(entity_id).await?;
    println!("Smart switch turned on!");
    
    // Wait a bit
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    // Turn the smart switch off
    println!("Turning smart switch off...");
    rustplus.turn_smart_switch_off(entity_id).await?;
    println!("Smart switch turned off!");
    
    // Wait a bit
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    // Turn it on again
    println!("Turning smart switch on again...");
    rustplus.turn_smart_switch_on(entity_id).await?;
    println!("Smart switch turned on!");
    
    // Disconnect
    println!("Disconnecting...");
    rustplus.disconnect().await?;
    println!("Disconnected successfully!");
    
    Ok(())
}
