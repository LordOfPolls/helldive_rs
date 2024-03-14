use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct WarInfo {
    #[serde(rename = "warId")]
    pub war_id: i64,
    #[serde(rename = "startDate")]
    pub start_date: i64,
    #[serde(rename = "endDate")]
    pub end_date: i64,
    #[serde(rename = "minimumClientVersion")]
    pub minimum_client_version: String,
    #[serde(rename = "planetInfos")]
    pub planet_infos: Vec<PlanetInfo>,
    #[serde(rename = "homeWorlds")]
    pub home_worlds: Vec<HomeWorld>,
    // #[serde(rename = "capitalInfos")]
    // pub capital_infos: Vec<String>,  // ToDo: Don't know the structure of capital_infos yet
    // #[serde(rename = "planetPermanentEffects")]
    // pub planet_permanent_effects: Vec<String>, // ToDo: Don't know the structure of planet_permanent_effects yet
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HomeWorld {
    pub race: i64,
    #[serde(rename = "planetIndices")]
    pub planet_indices: Vec<i64>,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlanetInfo {
    pub index: i64,
    #[serde(rename = "settingsHash")]
    pub settings_hash: i64,
    pub position: Position,
    pub waypoints: Vec<i64>,
    pub sector: i64,
    #[serde(rename = "maxHealth")]
    pub max_health: i64,
    pub disabled: bool,
    #[serde(rename = "initialOwner")]
    pub initial_owner: i64,
    #[serde(skip)]
    pub planet_name: String,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

