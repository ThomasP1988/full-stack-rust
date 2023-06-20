
use std::time::Instant;
use actix::{ ActorContext};
use actix::{StreamHandler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;


use super::actor::{WsConnection};

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => {
                println!("websocket received text {}", s)
            },
            Err(e) => std::panic::panic_any(e),
        }
    }
}