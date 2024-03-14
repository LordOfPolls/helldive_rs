use std::collections::hash_map::HashMap;
use toml::from_str;

use crate::models::{Planet, Faction, Sector};


const RAW_PLANETS: &str = include_str!("../res/planets.toml");

// const RAW_EFFECTS: &str = include_str!("../res/effects.toml");  
// ToDo: Implement effects once their structure is known
const RAW_FACTIONS: &str = include_str!("../res/factions.toml");
const RAW_SECTORS: &str = include_str!("../res/sectors.toml");

pub fn load_planets() -> HashMap<i64, Planet> {
    let planets: HashMap<String, String> = from_str(RAW_PLANETS).unwrap();
    let mut planet_map: HashMap<i64, Planet> = HashMap::new();
    for (id, name) in planets {
        planet_map.insert(id.parse().unwrap(), Planet{id: id.parse().unwrap(), name});
    }
    planet_map
}

pub fn load_factions() -> HashMap<i64, Faction> {
    let factions: HashMap<String, String> = from_str(RAW_FACTIONS).unwrap();
    let mut faction_map: HashMap<i64, Faction> = HashMap::new();
    for (id, name) in factions {
        faction_map.insert(id.parse().unwrap(), Faction{id: id.parse().unwrap(), name});
    }
    faction_map
}

pub fn load_sectors() -> HashMap<i64, Sector> {
    let sectors: HashMap<String, String> = from_str(RAW_SECTORS).unwrap();
    let mut sector_map: HashMap<i64, Sector> = HashMap::new();
    for (id, name) in sectors {
        sector_map.insert(id.parse().unwrap(), Sector{id: id.parse().unwrap(), name});
    }
    sector_map
}
