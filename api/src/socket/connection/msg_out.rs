
use actix::prelude::{Message};
use actix::{Handler};
use super::actor::{WsConnection};


#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

impl Handler<WsMessage> for WsConnection {
    type Result = ();
    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        let WsMessage(msg_str) = msg;
        ctx.text(msg_str);
    }
}
