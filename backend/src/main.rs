use auth::{login, login_options, signup, signup_options};
use config::Config;
use rocket::http::Method;
use rocket::tokio::sync::Mutex;
use rocket::{fs::FileServer, launch, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::sync::Arc;

mod auth;
mod database;
mod error;
mod features;
mod messenger;
mod servers;
mod user;
mod chatsync;
mod config;

pub(crate) use error::BackendError;
use messenger::{connect, MessengerServer};
use chatsync::ChatSyncFairing;

#[launch]
async fn rocket() -> _ {
    let messenger = Arc::new(Mutex::new(MessengerServer::new()));
    let chat_sync = ChatSyncFairing {
        messenger: messenger.clone(),
    };

    let config = Config::load();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::some_exact(&[
            &config.dev_url,
            &config.prod_url,
        ]))
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Options]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allowed_headers(AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
        ]))
        .allow_credentials(true);

    database::init().await.unwrap();

    rocket::build()
        .mount("/static", FileServer::from("static"))
        .mount(
            "/api",
            routes![
                connect,
                login,
                login_options,
                signup,
                signup_options,
            ],
        )
        .manage(messenger)
        .manage(config)
        .attach(cors.to_cors().unwrap())
        .attach(chat_sync)
}
