use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Sector {
    pub id: i64,
    pub name: String,
    pub planets: Vec<i64>,
}