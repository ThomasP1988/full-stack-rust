use actix_web::{get, Result, HttpResponse, Error};
use super::LoggedUser;

#[get("/me")]
pub async fn handler(user: LoggedUser) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(user.0))
}
