use std::collections::HashMap;
use once_cell::sync::Lazy;

mod load_res;
mod models;
mod requests;

pub use models::api::{Status, WarInfo, PlanetStatus, PlanetAttack, Campaign, GlobalEvent, HomeWorld, Position, PlanetInfo};
pub use models::{Planet, Faction, Sector, Language};
pub use requests::{get_status, get_war_info};


/// The base URL for the Helldivers API
pub const BASE_URL: &str = "https://api.live.prod.thehelldiversgame.com/api";

/// The planets in the game
pub static PLANETS: Lazy<HashMap<i64, Planet>> = Lazy::new(load_res::load_planets);

/// The active factions in the game
pub static FACTIONS: Lazy<HashMap<i64, models::Faction>> = Lazy::new(load_res::load_factions);

/// The sectors in the game
pub static SECTORS: Lazy<HashMap<i64, models::Sector>> = Lazy::new(load_res::load_sectors);


/// Get the name of a planet
///
/// Arguments:
///   id: i64 - The ID of the planet
pub fn get_planet_name(id: i64) -> Option<String> {
    PLANETS.get(&id).map(|planet| planet.name.clone())
}

/// Get the name of a faction
///
/// Arguments:
///    id: i64 - The ID of the faction
pub fn get_faction_name(id: i64) -> Option<String> {
    FACTIONS.get(&id).map(|faction| faction.name.clone())
}

/// Get the name of a sector
///
/// Arguments:
///   id: i64 - The ID of the sector
pub fn get_sector_name(id: i64) -> Option<String> {
    SECTORS.get(&id).map(|sector| sector.name.clone())
}

/// Get the total player count for a status
/// 
/// Arguments:
///  status: Status - The status to get the player count for
pub fn get_total_player_count(status: &Status) -> i64 {
    status.planet_status.iter().map(|ps| ps.players).sum()
}

/// Get the top planets by player count
/// 
/// Arguments:
///  status: &Status - The status to get the top planets from
///  count: usize - The number of top planets to get
pub fn get_top_planets_by_player_count(status: &Status, count: usize) -> Vec<(&PlanetStatus, i64)> {
    let mut planet_players: Vec<(&PlanetStatus, i64)> = status.planet_status.iter().map(|ps| (ps, ps.players)).collect();
    planet_players.sort_by(|a, b| b.1.cmp(&a.1));
    planet_players.into_iter().take(count).collect()
}

/// Get the faction distribution 
/// 
/// Arguments:
///  status: &Status - The status to get the faction distribution from
pub fn get_faction_distribution(status: &Status) -> HashMap<i64, i64> {
    let mut distribution = HashMap::new();
    for ps in &status.planet_status {
        *distribution.entry(ps.owner).or_insert(0) += 1;
    }
    distribution
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_status() {
        let _status = match get_status(801, Language::English).await{
            Ok(status) => status,
            Err(e) => panic!("Error: {}", e),
        };
    }

    #[tokio::test]
    async fn test_get_war_info() {
        let _war_info = match get_war_info(801).await {
            Ok(war_info) => war_info,
            Err(e) => panic!("Error: {}", e),
        };
    }

    #[test]
    fn test_load_planets() {
        assert!(PLANETS.len() > 0);
    }

    #[test]
    fn test_load_factions() {
        assert!(FACTIONS.len() > 0);
    }

    #[test]
    fn test_load_sectors() {
        assert!(SECTORS.len() > 0);
    }

    #[test]
    fn test_get_planet_name() {
        assert_eq!(get_planet_name(0).unwrap_or_default(), "Super Earth");
    }

    #[test]
    fn test_get_faction_name() {
        assert_eq!(get_faction_name(1).unwrap_or_default(), "Humans");
    }

    #[test]
    fn test_get_sector_name() {
        assert_eq!(get_sector_name(0).unwrap_or_default(), "Sol");
    }

    #[tokio::test]
    async fn test_war_info_planet_name() {
        let war_info = match get_war_info(801).await {
            Ok(war_info) => war_info,
            Err(e) => panic!("Error: {}", e),
        };
        for planet_info in war_info.planet_infos {
            assert!(!planet_info.planet_name.is_empty())
        }
    }

    #[tokio::test]
    async fn test_status_planet_name() {
        let status = match get_status(801, Language::English).await {
            Ok(status) => status,
            Err(e) => panic!("Error: {}", e),
        };
        for planet_status in status.planet_status {
            assert!(!planet_status.planet_name.is_empty())
        }
    }

    #[tokio::test]
    async fn test_alternate_language() {
        let de_status = match get_status(801, Language::German).await {
            Ok(status) => status,
            Err(e) => panic!("Error: {}", e),
        };

        let en_status = match get_status(801, Language::English).await {
            Ok(status) => status,
            Err(e) => panic!("Error: {}", e),
        };

        assert_ne!(de_status.global_events[0].message, en_status.global_events[0].message);
    }
    
    #[tokio::test]
    async fn test_total_player_count() {
        let status = match get_status(801, Language::English).await {
            Ok(status) => status,
            Err(e) => panic!("Error: {}", e),
        };
        assert!(get_total_player_count(&status) > 0);
    }
    
    #[tokio::test]
    async fn test_top_planets_by_player_count() {
        let status = match get_status(801, Language::English).await {
            Ok(status) => status,
            Err(e) => panic!("Error: {}", e),
        };
        let top_planets = get_top_planets_by_player_count(&status, 5);
        assert!(!top_planets.is_empty());
    }
    
    #[tokio::test]
    async fn test_faction_distribution() {
        let status = match get_status(801, Language::English).await {
            Ok(status) => status,
            Err(e) => panic!("Error: {}", e),
        };
        let distribution = get_faction_distribution(&status);
        assert!(!distribution.is_empty());
    }
}