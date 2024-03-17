use std::collections::HashMap;
use crate::{PlanetStatus, Status, WarInfo, get_sector_name, Faction, get_faction_name};
use crate::models::Sector;

/// Get the total player count for a status
///
/// Arguments:
///     status: Status - The status to get the player count for
pub fn get_total_player_count(status: &Status) -> i64 {
    status.planet_status.iter().map(|ps| ps.players).sum()
}

/// Get the top planets by player count
///
/// Arguments:
///     status: &Status - The status to get the top planets from
///     count: usize - The number of top planets to get
pub fn get_top_planets_by_player_count(status: &Status, count: usize) -> Vec<(&PlanetStatus, i64)> {
    let mut planet_players: Vec<(&PlanetStatus, i64)> = status.planet_status.iter().map(|ps| (ps, ps.players)).collect();
    planet_players.sort_by(|a, b| b.1.cmp(&a.1));
    planet_players.into_iter().take(count).collect()
}

/// Get the faction distribution
///
/// Arguments:
///     status: &Status - The status to get the faction distribution from
pub fn get_faction_distribution(status: &Status) -> HashMap<i64, i64> {
    let mut distribution = HashMap::new();
    for ps in &status.planet_status {
        *distribution.entry(ps.owner).or_insert(0) += 1;
    }
    distribution
}

/// Get the sectors from a WarInfo
///
/// Arguments:
///     status: &WarInfo - The WarInfo to get the sectors from
pub fn get_sectors(status: &WarInfo) -> Vec<Sector> {
    // intentionally does not use the cached sectors in case there are new sectors in the WarInfo
    let mut sectors = status.planet_infos.iter().map(|pi| pi.sector).collect::<Vec<_>>();
    sectors.sort();
    sectors.dedup();
    sectors.into_iter().map(|s| Sector {
        id: s, name: get_sector_name(s).unwrap_or_default(), planets: status.planet_infos.iter().filter(|pi| pi.sector == s).map(|pi| pi.index).collect()
    }).collect()
}

/// Get the factions from a Status
///
/// Arguments:
///    status: &Status - The Status to get the factions from
pub fn get_factions(status: &Status) -> Vec<Faction> {
    let mut factions = status.planet_status.iter().map(|ps| ps.owner).collect::<Vec<_>>();
    factions.sort();
    factions.dedup();
    factions.into_iter().map(|f| Faction { id: f, name: get_faction_name(f).unwrap_or_default() }).collect()
}

