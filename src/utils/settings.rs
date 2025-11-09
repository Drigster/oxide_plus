use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerData {
    pub name: String,
    pub address: String,
}

pub fn load_servers() -> Result<Vec<ServerData>, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join("oxideplus").join("servers.json");

    let file = std::fs::File::open(config_path);
    if file.is_err() {
        println!("No servers.json found.");
        return Ok(vec![]);
    }

    let file = file.unwrap();
    let reader = std::io::BufReader::new(file);

    let servers: Vec<ServerData> = serde_json::from_reader(reader).unwrap();
    return Ok(servers);
}

pub fn save_servers(servers: Vec<ServerData>) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join("oxideplus").join("servers.json");

    if !config_path.exists() {
        std::fs::create_dir_all(config_dir.join("oxideplus"))?;
    }

    let file = std::fs::File::create(config_path);
    if file.is_err() {
        println!("Failed to create servers.json");
        return Err("Failed to create servers.json".into());
    }

    let file = file.unwrap();
    let mut writer = std::io::BufWriter::new(file);

    let json = serde_json::to_string_pretty(&servers).unwrap();
    writer.write_all(json.as_bytes()).unwrap();
    println!("Saved {} servers", servers.len());

    Ok(())
}
