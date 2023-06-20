use actix::{prelude::{Message, Recipient}, Handler, Context};
use uuid::Uuid;

use crate::socket::connection::msg_out::WsMessage;
use super::actor::Broker;


#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub self_id: Uuid,
}

impl Handler<Connect> for Broker {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.sessions.insert(
            msg.self_id,
            msg.addr,
        );
    }
}
