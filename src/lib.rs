use std::collections::HashMap;
use once_cell::sync::Lazy;

mod load_res;
mod models;
mod requests;
mod utils;
mod error;

pub use models::api::{Status, WarInfo, PlanetStatus, PlanetAttack, Campaign, GlobalEvent, HomeWorld, Position, PlanetInfo, WarTime, NewsItem};
pub use models::{Planet, Faction, Sector, Language};
pub use requests::{get_status, get_war_info, get_war_time, get_news_feed};
pub use utils::{get_total_player_count, get_top_planets_by_player_count, get_faction_distribution, get_sectors, get_factions};

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

        if de_status.global_events.is_empty() || en_status.global_events.is_empty(){
            return
            // cant test, there is no message
        }

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
    
    #[tokio::test]
    async fn test_war_time() {
        let _ = match get_war_time(801).await {
            Ok(war_time) => war_time,
            Err(e) => panic!("Error: {}", e),
        };
    }

    #[tokio::test]
    async fn test_news_feed() {
        let news_feed = match get_news_feed(801, Language::English).await {
            Ok(news_feed) => news_feed,
            Err(e) => panic!("Error: {}", e),
        };
        assert!(!news_feed.is_empty());
    }

    #[tokio::test]
    async fn test_sector_planet_mapping () {
        // Ensure that our sector to planet mapping is correct
        let war_info = match get_war_info(801).await {
            Ok(war_info) => war_info,
            Err(e) => panic!("Error: {}", e),
        };

        let planet_info = war_info.planet_infos;
        let planet_info_map: HashMap<i64, &PlanetInfo> = planet_info.iter().map(|info| (info.index, info)).collect();

        for (_, sector) in SECTORS.iter() {
            for planet_index in &sector.planets {
                if let Some(info) = planet_info_map.get(planet_index) {
                    assert_eq!(sector.id, info.sector);
                    break;
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_sectors() {
        let war_info = match get_war_info(801).await {
            Ok(war_info) => war_info,
            Err(e) => panic!("Error: {}", e),
        };
        let sectors = get_sectors(&war_info);
        assert!(!sectors.is_empty());
    }

    #[tokio::test]
    async fn test_get_factions() {
        let status = match get_status(801, Language::English).await {
            Ok(status) => status,
            Err(e) => panic!("Error: {}", e),
        };
        let factions = get_factions(&status);
        assert!(!factions.is_empty());
    }
}