use crate::BASE_URL;
use crate::models::api::{Status, WarInfo};


/// Get the current status of a war
///
/// Arguments:
///    war_id: i64 - The ID of the war to get the status of
pub fn get_status(war_id: i64) -> Result<Status, reqwest::Error> {
    let url = format!("{}/WarSeason/{}/Status", BASE_URL, war_id);

    let response = reqwest::blocking::get(url).unwrap();

    let mut status: Status = response.json()?;

    for campaign in &mut status.campaigns {
        campaign.planet_name = crate::get_planet_name(campaign.planet_index);
    }

    for planet_attack in &mut status.planet_attacks {
        planet_attack.source_name = crate::get_planet_name(planet_attack.source);
        planet_attack.target_name = crate::get_planet_name(planet_attack.target);
    }

    for planet_status in &mut status.planet_status {
        planet_status.planet_name = crate::get_planet_name(planet_status.index);
    }

    Ok(status)
}

/// Get the information for a war
///
/// Arguments:
///   war_id: i64 - The ID of the war to get the information for
pub fn get_war_info(war_id: i64) -> Result<WarInfo, reqwest::Error> {
    let url = format!("{}/WarSeason/{}/WarInfo", BASE_URL, war_id);

    let response = reqwest::blocking::get(url).unwrap();

    let mut war_info: WarInfo = response.json()?;

    for planet_info in &mut war_info.planet_infos {
        planet_info.planet_name = crate::get_planet_name(planet_info.index);
    }

    Ok(war_info)
}