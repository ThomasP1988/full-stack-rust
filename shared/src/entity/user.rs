use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct User {
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CreateUser {
    pub username: String,
}
