use std::collections::HashMap;
use serde::{Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct NewsItem {
    pub id: i64,
    pub published: i64,
    #[serde(rename = "type")]
    pub news_type: i64,  // always 0 so far
    #[serde(default)]
    pub tag_ids: Vec<String>,
    pub message: String,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}