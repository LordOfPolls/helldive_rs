use std::collections::HashMap;
use serde::{Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct NewsItem {
    id: i64,
    published: i64,
    #[serde(rename = "type")]
    news_type: i64,  // always 0 so far
    #[serde(default)]
    tag_ids: Vec<String>,
    message: String,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}