use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::json::json_date::{json_date};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Message {
    pub username: String,
    pub message: String,
    #[serde(with = "json_date")]
    pub date: DateTime<Utc>,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AddMessage {
    pub message: String,
}
