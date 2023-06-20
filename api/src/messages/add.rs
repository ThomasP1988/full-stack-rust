use actix::Addr;
use actix_web::{post, web, HttpResponse, Error};
use actix_web::web::Data;
use shared::message::{AddMessage, Message};
use crate::socket::broker::actor::Broker;
use crate::socket::broker::broadcast::Broadcast;
use crate::store::messages::{add};
use crate::user::LoggedUser;


#[post("/message")]
pub async fn handler(add_message: web::Json<AddMessage>, user: LoggedUser, ws_broker: Data<Addr<Broker>>) -> Result<HttpResponse, Error> {
    let LoggedUser(user) = user;
    let msg: Message = add(&user.username, &add_message.message);
    
    match serde_json::to_string(&msg) {
        Ok(msg_json) => ws_broker.do_send(Broadcast{ content: msg_json }),
        Err(_) => return Ok(HttpResponse::InternalServerError().finish())
    };

    Ok(HttpResponse::Ok().json(msg))
}