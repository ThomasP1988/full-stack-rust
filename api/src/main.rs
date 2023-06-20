use crate::socket::broker::actor::Broker;
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::{DefaultHeaders, Logger}, App, HttpServer, web::Data};
use env_logger;
use actix::Actor;

mod messages;
mod socket;
mod store;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key: Key = Key::generate();
    let ws_broker = Data::new(Broker::default().start());
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(
                "ip=%{r}a req_line=\"%r\" referer=\"%{Referer}i\" status=%s size=%bB time=%Ts",
            ))
           
            .wrap(Cors::permissive())
            .wrap(DefaultHeaders::new().add(("Access-Control-Allow-Credentials", "true")))
            .service(messages::list::handler)
            .service(user::signin::handler)
            .service(user::me::handler)
            .service(messages::add::handler)
            .service(socket::connect::handler)
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .cookie_same_site(actix_web::cookie::SameSite::Lax)
                    .build(),
            )
            .app_data(Data::clone(&ws_broker))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
