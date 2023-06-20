use crate::store::users::{add, get};
use actix_identity::Identity;
use actix_web::{post, web, Error, HttpMessage, HttpRequest, HttpResponse};
use shared::user::{CreateUser, User};

#[post("/signin")]
pub async fn handler(
    request: HttpRequest,
    create_user: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    if let Some(_) = get(create_user.username.as_str()) {
        return Ok(HttpResponse::Forbidden().reason("user already exists".clone()).body("user already exists".clone()));
    }

    let user: User = add(create_user.username.as_str());
    match Identity::login(&request.extensions(), user.username.clone()) {
        Ok(_) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
