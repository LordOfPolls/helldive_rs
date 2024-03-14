use std::collections::HashMap;
use serde::{Deserialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Status {
    #[serde(rename = "warId")]
    pub war_id: i64,
    pub time: i64,
    #[serde(rename = "impactMultiplier")]
    pub impact_multiplier: f64,
    #[serde(rename = "storyBeatId32")]
    pub story_beat_id32: i64,
    #[serde(rename = "planetStatus")]
    pub planet_status: Vec<PlanetStatus>,
    #[serde(rename = "planetAttacks")]
    pub planet_attacks: Vec<PlanetAttack>,
    pub campaigns: Vec<Campaign>,
    // pub community_targets: Vec<String>, // ToDo: Don't know the structure of community_targets yet
    // pub joint_operations: Vec<String>,  // ToDo: Don't know the structure of joint_operations yet
    // pub planet_events: Vec<String>,  // ToDo: Don't know the structure of planet_events yet
    // pub planet_active_effects: Vec<String>,  // ToDo: Don't know the structure of planet_active_effects yet
    // pub active_election_policy_effects: Vec<String>,  // ToDo: Don't know the structure of active_election_policy_effects yet
    #[serde(rename = "globalEvents")]
    pub global_events: Vec<GlobalEvent>,
    // pub super_earth_war_results: Vec<String>,  // ToDo: Don't know the structure of super_earth_war_results yet
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
pub struct GlobalEvent {
    #[serde(rename = "eventId")]
    pub event_id: i64,
    pub id32: i64,
    #[serde(rename = "portraitId32")]
    pub portrait_id32: i64,
    pub title: String,
    #[serde(rename = "titleId32")]
    pub title_id32: i64,
    pub message: String,
    #[serde(rename = "messageId32")]
    pub message_id32: i64,
    pub race: i64,
    pub flag: i64,
    #[serde(rename = "assignmentId32")]
    pub assignment_id32: i64,
    // pub effect_ids: Vec<String>, // ToDo: Don't know the structure of effect_ids yet
    // pub planet_indices: Vec<String>, // ToDo: Don't know the structure of planet_indices yet
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
pub struct Campaign {
    pub id: i64,
    #[serde(rename = "planetIndex")]
    pub planet_index: i64,
    pub r#type: i64,
    pub count: i64,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
    #[serde(skip)]
    pub planet_name: String,
}

#[derive(Deserialize, Debug)]
pub struct PlanetAttack {
    pub source: i64,
    pub target: i64,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,

    #[serde(skip)]
    pub source_name: String,
    #[serde(skip)]
    pub target_name: String,
}

#[derive(Deserialize, Debug)]
pub struct PlanetStatus {
    pub index: i64,
    pub owner: i64,
    pub health: i64,
    #[serde(rename = "regenPerSecond")]
    pub regen_per_second: f64,
    pub players: i64,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
    #[serde(skip)]
    pub planet_name: String,
}

