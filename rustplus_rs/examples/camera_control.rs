use rustplus_rs::{RustPlus, Camera, RustPlusError};
use rustplus_rs::camera::buttons;
use std::sync::Arc;
use tokio;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), RustPlusError> {
    // Initialize logging
    env_logger::init();
    
    // Create a new RustPlus client
    // Replace these values with your actual server details
    let rustplus = Arc::new(RustPlus::new(
        "127.0.0.1",        // Server IP
        28082,              // App port
        76561198000000000,  // Player ID (Steam ID)
        12345,              // Player token
        false,              // Use Facepunch proxy
    ).await?);
    
    // Connect to the server
    println!("Connecting to Rust server...");
    rustplus.connect().await?;
    println!("Connected successfully!");
    
    // Create a camera instance
    // Replace with actual camera identifier (e.g., "OILRIG1", "DOME1", etc.)
    let camera = Camera::new(rustplus.clone(), "OILRIG1");
    
    // Subscribe to the camera
    println!("Subscribing to camera...");
    camera.subscribe().await?;
    println!("Subscribed to camera successfully!");
    
    // Check if it's an auto turret
    if camera.is_auto_turret().await {
        println!("This camera is an auto turret!");
        
        // Shoot the turret
        println!("Shooting turret...");
        camera.shoot().await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        // Reload the turret
        println!("Reloading turret...");
        camera.reload().await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    } else {
        println!("This camera is a regular camera (CCTV or PTZ)");
        
        // Move the camera
        println!("Moving camera...");
        camera.move_camera(buttons::LEFT, 10.0, 0.0).await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        camera.move_camera(buttons::RIGHT, -10.0, 0.0).await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        // Try to zoom (for PTZ cameras)
        println!("Attempting to zoom...");
        camera.zoom().await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    
    // Unsubscribe from the camera
    println!("Unsubscribing from camera...");
    camera.unsubscribe().await?;
    println!("Unsubscribed from camera!");
    
    // Disconnect
    println!("Disconnecting...");
    rustplus.disconnect().await?;
    println!("Disconnected successfully!");
    
    Ok(())
}
