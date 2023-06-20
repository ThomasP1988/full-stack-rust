use actix::Addr;
use actix_web::web::Data;
use actix_web::{get, web::Payload, Error, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use super::connection::actor::WsConnection;
use super::broker::actor::Broker;

#[get("/ws")]
pub async fn handler(req: HttpRequest, stream: Payload, ws_broker: Data<Addr<Broker>>) -> Result<HttpResponse, Error> {
    let resp: std::result::Result<HttpResponse, Error> = ws::start(WsConnection::new(ws_broker.get_ref().clone()), &req, stream);
    println!("{:?}", resp);
    resp
}
