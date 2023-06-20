use shared::message::{Message, AddMessage};
use gloo_console::log;
use super::request;
use crate::error::{Error};

pub async fn list() -> Result<Vec<Message>, Error>  {
  log!("getting list");
  return request::get::<Vec<Message>>(format!("/messages")).await
}

pub async fn add(payload: AddMessage) -> Result<Message, Error>  {
  log!("add message");
  return request::post::<AddMessage, Message>(format!("/message"), payload).await
}
