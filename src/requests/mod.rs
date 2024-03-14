use crate::BASE_URL;
use crate::models::api::{Status, WarInfo};


/// Get the current status of a war
///
/// Arguments:
///    war_id: i64 - The ID of the war to get the status of
///    language: &str - The language to get the status in, in language-country format (e.g. en-US)
pub async fn get_status(war_id: i64, language: &str) -> Result<Status, reqwest::Error> {
    let url = format!("{}/WarSeason/{}/Status", BASE_URL, war_id);

    let language = if language.is_empty() { "en-US" } else { language };

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept-Language", language.parse().unwrap());

    let response = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .await?;

    let mut status: Status = response.json().await?;

    for campaign in &mut status.campaigns {
        campaign.planet_name = crate::get_planet_name(campaign.planet_index).unwrap_or_default();
    }

    for planet_attack in &mut status.planet_attacks {
        planet_attack.source_name = crate::get_planet_name(planet_attack.source).unwrap_or_default();
        planet_attack.target_name = crate::get_planet_name(planet_attack.target).unwrap_or_default();
    }

    for planet_status in &mut status.planet_status {
        planet_status.planet_name = crate::get_planet_name(planet_status.index).unwrap_or_default();
    }

    Ok(status)
}

/// Get the information for a war
///
/// Arguments:
///   war_id: i64 - The ID of the war to get the information for
pub async fn get_war_info(war_id: i64) -> Result<WarInfo, reqwest::Error> {
    let url = format!("{}/WarSeason/{}/WarInfo", BASE_URL, war_id);

    let response = reqwest::get(url).await?;

    let mut war_info: WarInfo = response.json().await?;

    for planet_info in &mut war_info.planet_infos {
        planet_info.planet_name = crate::get_planet_name(planet_info.index).unwrap_or_default();
    }

    Ok(war_info)
}