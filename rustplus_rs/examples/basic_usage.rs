use rustplus_rs::{RustPlus, RustPlusError};
use tokio;

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
    
    // Send a team message
    println!("Sending team message...");
    rustplus.send_team_message("Hello from RustPlus RS!").await?;
    
    // Get server info
    println!("Getting server info...");
    let info = rustplus.get_info().await?;
    println!("Server: {} ({} players/{})", info.name, info.players, info.max_players);
    
    // Get time info
    println!("Getting time info...");
    let time = rustplus.get_time().await?;
    println!("Current time: {:.2} (Day length: {:.1} minutes)", time.time, time.day_length_minutes);
    
    // Get team info
    println!("Getting team info...");
    match rustplus.get_team_info().await {
        Ok(team_info) => {
            println!("Team leader: {}", team_info.leader_steam_id);
            println!("Team members: {}", team_info.members.len());
            for member in &team_info.members {
                println!("  - {} ({}): ({:.1}, {:.1})", 
                    member.name, 
                    if member.is_online { "online" } else { "offline" },
                    member.x, member.y
                );
            }
        }
        Err(e) => println!("Failed to get team info: {}", e),
    }
    
    // Disconnect
    println!("Disconnecting...");
    rustplus.disconnect().await?;
    println!("Disconnected successfully!");
    
    Ok(())
}
