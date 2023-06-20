use actix_identity::{Identity, IdentityExt};
use actix_web::{dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpRequest, Result};
use shared::user::User;
use std::future::{ready, Ready};

use crate::store::users::get;

pub mod me;
pub mod signin;

pub struct LoggedUser(pub User);

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let identity: Identity = match req.get_identity() {
            Ok(identity) => identity,
            Err(e) => {
                println!("{}", e);
                return ready(Err(ErrorUnauthorized("can't retrieve user")));
            }
        };

        let username: String = match identity.id() {
            Ok(username) => username,
            Err(e) => {
                println!("{}", e);
                return ready(Err(ErrorUnauthorized("can't retrieve user id")));
            }
        };

        let user: User = match get(username.as_str()) {
            Some(user) => user,
            None => return ready(Err(ErrorUnauthorized("user not found"))),
        };

        return ready(Ok(LoggedUser(user)));
    }
}
