use capitalize::Capitalize;
use freya::prelude::{Bytes, Color};

pub fn number_to_letters(n: u32) -> String {
    let mut result = String::new();
    let mut num = n + 1;

    while num > 0 {
        num -= 1;
        let remainder = (num % 26) as u8;
        result.insert(0, (b'A' + remainder) as char);
        num /= 26;
    }

    result
}

pub fn normalize_monument_name(name: String) -> String {
    let regex = regex::Regex::new(r"([A-Z])").unwrap();

    match name.as_str() {
        "power_plant_display_name" => "Power Plant".to_string(),
        "excavator" => "Excavator".to_string(),
        "junkyard_display_name" => "Junkyard".to_string(),
        "stables_a" => "Ranch".to_string(),
        "stables_b" => "Large Barn".to_string(),
        "mining_quarry_hqm_display_name" => "HQM Quarry".to_string(),
        "mining_quarry_stone_display_name" => "Stone Quarry".to_string(),
        "mining_quarry_sulfur_display_name" => "Sulfur Quarry".to_string(),
        "sewer_display_name" => "Sewer Branch".to_string(),
        "train_tunnel_link_display_name" => "Train Tunnel".to_string(),
        "jungle_ziggurat" => "Jungle Ziggurat".to_string(),
        "supermarket" => "Abandoned Supermarket".to_string(),
        "mining_outpost_display_name" => "Mining Outpost".to_string(),
        "gas_station" => "Oxum's Gas Station".to_string(),
        "radtown" => "Radtown".to_string(),
        "underwater_lab" => "Underwater Lab".to_string(),
        "oil_rig_small" => "Oil Rig".to_string(),
        "large_oil_rig" => "Large Oil Rig".to_string(),
        "lighthouse_display_name" => "Lighthouse".to_string(),
        "harbor_display_name" => "Harbor".to_string(),
        "harbor_2_display_name" => "Harbor".to_string(),
        "ferryterminal" => "Ferry Terminal".to_string(),
        "large_fishing_village_display_name" => "Large Fishing Village".to_string(),
        "fishing_village_display_name" => "Fishing Village".to_string(),
        "AbandonedMilitaryBase" => "Abandoned Military Base".to_string(),
        "arctic_base_a" => "Arctic Base".to_string(),
        "water_treatment_plant_display_name" => "Water Treatment Plant".to_string(),
        "outpost" => "Outpost".to_string(),
        "launchsite" => "Launch Site".to_string(),
        "dome_monument_name" => "The Dome".to_string(),
        "train_yard_display_name" => "Train Yard".to_string(),
        "military_tunnels_display_name" => "Military Tunnel".to_string(),
        "satellite_dish_display_name" => "Satellite Dish".to_string(),
        "airfield_display_name" => "Airfield".to_string(),
        "missile_silo_monument" => "Missile Silo".to_string(),
        unknown_name => {
            println!("Unknown name: {}", unknown_name);
            let name = unknown_name
                .replace("display_name", "")
                .replace("monument_name", "")
                .replace("monument", "");

            regex
                .replace_all(name.as_str(), "_$1")
                .replace("_", " ")
                .trim()
                .capitalize()
        }
    }
}

pub fn index_to_color(index: i32) -> (Color, Color) {
    match index {
        1 => (
            Color::from_hex("#3075ca").unwrap(),
            Color::from_hex("#12233e").unwrap(),
        ),
        2 => (
            Color::from_hex("#79ab3a").unwrap(),
            Color::from_hex("#243410").unwrap(),
        ),
        3 => (
            Color::from_hex("#c03939").unwrap(),
            Color::from_hex("#3a110f").unwrap(),
        ),
        4 => (
            Color::from_hex("#af59bc").unwrap(),
            Color::from_hex("#361d39").unwrap(),
        ),
        5 => (
            Color::from_hex("#05eec3").unwrap(),
            Color::from_hex("#044c3d").unwrap(),
        ),
        _ => (
            Color::from_hex("#d2d456").unwrap(),
            Color::from_hex("#444518").unwrap(),
        ),
    }
}

pub fn index_to_icon(index: i32) -> Bytes {
    match index {
        1 => Bytes::from_static(include_bytes!("../assets/MDI/currency-usd.svg")),
        2 => Bytes::from_static(include_bytes!("../assets/MDI/home.svg")),
        3 => Bytes::from_static(include_bytes!("../assets/MDI/parachute.svg")),
        4 => Bytes::from_static(include_bytes!("../assets/MDI/crosshairs-gps.svg")),
        5 => Bytes::from_static(include_bytes!("../assets/MDI/shield.svg")),
        6 => Bytes::from_static(include_bytes!("../assets/MDI/skull.svg")),
        7 => Bytes::from_static(include_bytes!("../assets/MDI/bed.svg")),
        8 => Bytes::from_static(include_bytes!("../assets/MDI/sleep.svg")),
        9 => Bytes::from_static(include_bytes!("../assets/MDI/pistol.svg")),
        // 10 => lucide::stone(),
        11 => Bytes::from_static(include_bytes!("../assets/MDI/treasure-chest.svg")),
        _ => Bytes::from_static(include_bytes!("../assets/MDI/map-marker.svg")),
    }
}
