use actix::{prelude::{Message}, Handler, Context};
use super::actor::Broker;


#[derive(Message)]
#[rtype(result = "()")]
pub struct Broadcast {
    pub content: String,
}

impl Handler<Broadcast> for Broker {
    type Result = ();

    fn handle(&mut self, msg: Broadcast, _: &mut Context<Self>) -> Self::Result {
        self.broadcast(msg.content.as_str());
    }
}
