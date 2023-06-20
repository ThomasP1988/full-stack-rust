use std::time::{Duration, Instant};
use actix::{Actor, Addr, Running, ActorFutureExt, AsyncContext, ContextFutureSpawner, WrapFuture, fut, ActorContext};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::socket::broker::actor::Broker;
use crate::socket::broker::connect::Connect;
use crate::socket::broker::disconnect::Disconnect;

#[derive( Clone, Debug)]
pub struct WsConnection {
    pub id: Uuid,
    pub broker_addr: Addr<Broker>,
    pub hb: Instant,
}

impl WsConnection {
    pub fn new(broker_addr: Addr<Broker>) -> WsConnection {
        WsConnection {
            id: Uuid::new_v4(),
            broker_addr: broker_addr,
            hb: Instant::now(),
        }
    }
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        self.broker_addr
            .send(Connect {
                addr: addr.recipient(),
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.broker_addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

impl WsConnection {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                ctx.stop();
                return;
            }

            ctx.ping(b"ping");
        });
    }
}
