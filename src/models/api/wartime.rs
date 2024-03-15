use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct WarTime {
    pub time: i64, // note, seems to only update every 10 seconds
}