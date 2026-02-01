use serde::{Deserialize, Serialize};

use crate::pages::UserData;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServerData {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub img: String,
    pub logo: String,
    pub url: String,
    pub ip: String,
    pub port: String,
    #[serde(rename = "playerId")]
    pub player_id: String,
    #[serde(rename = "playerToken")]
    pub player_token: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FcmData {
    pub android_id: u64,
    pub security_token: u64,
    pub private_key: String,
    pub auth_secret: String,
    pub fcm_token: String,
    pub last_persistent_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppData {
    steam_id: Option<String>,
    steam_token: Option<String>,
    expo_token: Option<String>,
    fcm_data: Option<FcmData>,
}

pub const APP_DIR_NAME: &str = "OxidePlus";
const SERVERS_FILENAME: &str = "servers.json";
const APP_DATA_FILENAME: &str = "user_data.json";

pub fn load_servers() -> Result<Vec<ServerData>, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join(APP_DIR_NAME).join(SERVERS_FILENAME);

    if !config_path.exists() {
        std::fs::create_dir_all(config_dir.join(APP_DIR_NAME))?;
    }

    let data = match std::fs::read_to_string(&config_path) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("[load_servers] No {SERVERS_FILENAME} found...");
            Ok(None)
        }
        Err(e) => Err(e),
    }?;

    let servers = match data {
        Some(content) => match serde_json::from_str::<Vec<ServerData>>(&content) {
            Ok(servers) => servers,
            Err(e) => {
                println!("[load_servers] Failed to parse {SERVERS_FILENAME}: {:?}", e);
                vec![]
            }
        },
        None => vec![],
    };

    return Ok(servers);
}

pub fn save_server(server: ServerData) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join(APP_DIR_NAME).join(SERVERS_FILENAME);

    if !config_path.exists() {
        std::fs::create_dir_all(config_dir.join(APP_DIR_NAME))?;
    }

    let data = match std::fs::read_to_string(&config_path) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("[save_server] No {SERVERS_FILENAME} found...");
            Ok(None)
        }
        Err(e) => Err(e),
    }?;

    let mut servers = match data {
        Some(content) => match serde_json::from_str::<Vec<ServerData>>(&content) {
            Ok(servers) => servers,
            Err(e) => {
                println!("[save_server] Failed to parse {SERVERS_FILENAME}: {:?}", e);
                vec![]
            }
        },
        None => vec![],
    };

    let mut existing_server = servers.iter().find(|s| s.id == server.id);

    if let Some(existing) = existing_server.as_mut() {
        *existing = &server;
    } else {
        servers.push(server);
    }

    std::fs::write(config_path, serde_json::to_string_pretty(&servers)?)?;

    println!("Saved {} servers", servers.len());

    Ok(())
}

pub fn load_user_data() -> Result<Option<UserData>, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join(APP_DIR_NAME).join(APP_DATA_FILENAME);

    if !config_path.exists() {
        std::fs::create_dir_all(config_dir.join(APP_DIR_NAME))?;
    }

    let data = match std::fs::read_to_string(&config_path) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("[load_user_data] No {APP_DATA_FILENAME} found...");
            Ok(None)
        }
        Err(e) => Err(e),
    }?;

    let user_data = match data {
        Some(content) => match serde_json::from_str::<AppData>(&content) {
            Ok(app_data) => {
                let (steam_id, steam_token) = match (app_data.steam_id, app_data.steam_token) {
                    (Some(id), Some(token)) => (id, token),
                    _ => {
                        println!("[load_user_data] Incomplete user data in {APP_DATA_FILENAME}");
                        return Ok(None);
                    }
                };
                Some(UserData {
                    steam_id: steam_id,
                    token: steam_token,
                })
            }
            Err(e) => {
                println!(
                    "[load_user_data] Failed to parse {APP_DATA_FILENAME}: {:?}",
                    e
                );
                None
            }
        },
        None => None,
    };

    return Ok(user_data);
}

pub fn save_user_data(user_data: UserData) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join(APP_DIR_NAME).join(APP_DATA_FILENAME);

    if !config_path.exists() {
        std::fs::create_dir_all(config_dir.join(APP_DIR_NAME))?;
    }

    let data = match std::fs::read_to_string(&config_path) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("[save_user_data] No {APP_DATA_FILENAME} found...");
            Ok(None)
        }
        Err(e) => Err(e),
    }?;

    let app_data = match data {
        Some(content) => match serde_json::from_str::<AppData>(&content) {
            Ok(app_data) => Some(app_data),
            Err(e) => {
                println!(
                    "[save_user_data] Failed to parse {APP_DATA_FILENAME}: {:?}",
                    e
                );
                None
            }
        },
        None => None,
    };

    let app_data = match app_data {
        Some(mut existing) => {
            existing.steam_id = Some(user_data.steam_id);
            existing.steam_token = Some(user_data.token);
            existing
        }
        None => AppData {
            steam_id: Some(user_data.steam_id),
            steam_token: Some(user_data.token),
            expo_token: None,
            fcm_data: None,
        },
    };

    std::fs::write(config_path, serde_json::to_string_pretty(&app_data)?)?;

    Ok(())
}

pub fn load_expo_push_token() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join(APP_DIR_NAME).join(APP_DATA_FILENAME);

    if !config_path.exists() {
        std::fs::create_dir_all(config_dir.join(APP_DIR_NAME))?;
    }

    let data = match std::fs::read_to_string(&config_path) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("[load_expo_push_token] No {APP_DATA_FILENAME} found...");
            Ok(None)
        }
        Err(e) => Err(e),
    }?;

    let expo_token = match data {
        Some(content) => match serde_json::from_str::<AppData>(&content) {
            Ok(app_data) => {
                let expo_token = match app_data.expo_token {
                    Some(token) => token,
                    None => {
                        println!(
                            "[load_expo_push_token] No expo token found in {APP_DATA_FILENAME}"
                        );
                        return Ok(None);
                    }
                };
                Some(expo_token)
            }
            Err(e) => {
                println!(
                    "[load_expo_push_token] Failed to parse {APP_DATA_FILENAME}: {:?}",
                    e
                );
                None
            }
        },
        None => None,
    };

    return Ok(expo_token);
}

pub fn save_expo_push_token(expo_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join(APP_DIR_NAME).join(APP_DATA_FILENAME);

    if !config_path.exists() {
        std::fs::create_dir_all(config_dir.join(APP_DIR_NAME))?;
    }

    let data = match std::fs::read_to_string(&config_path) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("[save_expo_push_token] No {APP_DATA_FILENAME} found...");
            Ok(None)
        }
        Err(e) => Err(e),
    }?;

    let app_data = match data {
        Some(content) => match serde_json::from_str::<AppData>(&content) {
            Ok(app_data) => Some(app_data),
            Err(e) => {
                println!(
                    "[save_expo_push_token] Failed to parse {APP_DATA_FILENAME}: {:?}",
                    e
                );
                None
            }
        },
        None => None,
    };

    let app_data = match app_data {
        Some(mut existing) => {
            existing.expo_token = Some(expo_token.to_owned());
            existing
        }
        None => AppData {
            steam_id: None,
            steam_token: None,
            expo_token: Some(expo_token.to_owned()),
            fcm_data: None,
        },
    };

    std::fs::write(config_path, serde_json::to_string_pretty(&app_data)?)?;

    Ok(())
}

pub fn load_fcm_data() -> Result<Option<FcmData>, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join(APP_DIR_NAME).join(APP_DATA_FILENAME);

    if !config_path.exists() {
        std::fs::create_dir_all(config_dir.join(APP_DIR_NAME))?;
    }

    let data = match std::fs::read_to_string(&config_path) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("[load_fcm_data] No {APP_DATA_FILENAME} found...");
            Ok(None)
        }
        Err(e) => Err(e),
    }?;

    let fcm_data = match data {
        Some(content) => match serde_json::from_str::<AppData>(&content) {
            Ok(app_data) => app_data.fcm_data,
            Err(e) => {
                println!(
                    "[load_fcm_data] Failed to parse {APP_DATA_FILENAME}: {:?}",
                    e
                );
                None
            }
        },
        None => None,
    };

    Ok(fcm_data)
}

pub fn save_fcm_data(fcm_data: FcmData) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join(APP_DIR_NAME).join(APP_DATA_FILENAME);

    if !config_path.exists() {
        std::fs::create_dir_all(config_dir.join(APP_DIR_NAME))?;
    }

    let data = match std::fs::read_to_string(&config_path) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("[save_fcm_data] No {APP_DATA_FILENAME} found...");
            Ok(None)
        }
        Err(e) => Err(e),
    }?;

    let app_data = match data {
        Some(content) => match serde_json::from_str::<AppData>(&content) {
            Ok(app_data) => Some(app_data),
            Err(e) => {
                println!(
                    "[save_fcm_data] Failed to parse {APP_DATA_FILENAME}: {:?}",
                    e
                );
                None
            }
        },
        None => None,
    };

    let app_data = match app_data {
        Some(mut existing) => {
            existing.fcm_data = Some(fcm_data);
            existing
        }
        None => AppData {
            steam_id: None,
            steam_token: None,
            expo_token: None,
            fcm_data: Some(fcm_data),
        },
    };

    std::fs::write(config_path, serde_json::to_string_pretty(&app_data)?)?;

    Ok(())
}

pub fn load_last_persistent_id() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join(APP_DIR_NAME).join(APP_DATA_FILENAME);

    if !config_path.exists() {
        std::fs::create_dir_all(config_dir.join(APP_DIR_NAME))?;
    }

    let data = match std::fs::read_to_string(&config_path) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("[load_fcm_data] No {APP_DATA_FILENAME} found...");
            Ok(None)
        }
        Err(e) => Err(e),
    }?;

    let fcm_data = match data {
        Some(content) => match serde_json::from_str::<AppData>(&content) {
            Ok(app_data) => app_data.fcm_data,
            Err(e) => {
                println!(
                    "[load_fcm_data] Failed to parse {APP_DATA_FILENAME}: {:?}",
                    e
                );
                None
            }
        },
        None => None,
    };

    match fcm_data {
        Some(fcm_data) => Ok(fcm_data.last_persistent_id),
        None => Ok(None),
    }
}

pub fn save_last_persistent_id(last_persistent_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join(APP_DIR_NAME).join(APP_DATA_FILENAME);

    if !config_path.exists() {
        std::fs::create_dir_all(config_dir.join(APP_DIR_NAME))?;
    }

    let data = match std::fs::read_to_string(&config_path) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("[save_fcm_data] No {APP_DATA_FILENAME} found...");
            Ok(None)
        }
        Err(e) => Err(e),
    }?;

    let app_data = match data {
        Some(content) => match serde_json::from_str::<AppData>(&content) {
            Ok(app_data) => Some(app_data),
            Err(e) => {
                println!(
                    "[save_fcm_data] Failed to parse {APP_DATA_FILENAME}: {:?}",
                    e
                );
                None
            }
        },
        None => None,
    };

    let app_data = match app_data {
        Some(mut existing) => {
            existing.fcm_data.as_mut().unwrap().last_persistent_id =
                Some(last_persistent_id.to_owned());
            existing
        }
        None => {
            panic!("FCM data is None");
        }
    };

    std::fs::write(config_path, serde_json::to_string_pretty(&app_data)?)?;

    Ok(())
}
