use actix::{prelude::{Message}, Handler, Context};
use uuid::Uuid;

use super::actor::Broker;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

impl Handler<Disconnect> for Broker {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
         self.sessions.remove(&msg.id);
    }
}
