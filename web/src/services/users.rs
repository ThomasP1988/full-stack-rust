use shared::user::{User, CreateUser};
use super::request;
use crate::error::{Error};

pub async fn me() -> Result<User, Error>  {
   request::get::<User>(format!("/me")).await
}

pub async fn create(payload: CreateUser) -> Result<User, Error>  {
   request::post::<CreateUser, User>(format!("/signin"), payload).await
}
