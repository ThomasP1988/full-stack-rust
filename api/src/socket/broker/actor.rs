use std::collections::HashMap;
use actix::{prelude::{Recipient}, Actor, Context};
use uuid::Uuid;
use crate::socket::connection::msg_out::WsMessage;

type Socket = Recipient<WsMessage>;

#[derive(Debug, Clone)]
pub struct Broker {
    pub sessions: HashMap<Uuid, Socket>,    
}

impl Default for Broker {
    fn default() -> Broker {
        Broker {
            sessions: HashMap::new(),
        }
    }
}

impl Actor for Broker {
    type Context = Context<Self>;
}

impl Broker {
    pub fn broadcast(&self, message: &str) {
        for (_, recipient) in self.sessions.clone() {
           recipient.do_send(WsMessage(message.to_owned()))
        }
    }

    pub fn _send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient
                .do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}


