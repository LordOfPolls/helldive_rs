use std::collections::HashMap;
use crate::{PlanetStatus, Status};

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