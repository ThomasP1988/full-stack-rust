use actix_web::{get, Result, HttpResponse, Error};
use crate::store::messages::{list};

#[get("/messages")]
pub async fn handler() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(list(100)))
}
