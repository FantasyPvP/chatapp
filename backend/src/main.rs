use auth::{login, login_options, signup, signup_options};
use rocket::{fs::FileServer, launch, routes};
use std::sync::Arc;
use rocket::tokio::sync::Mutex;

mod messenger;
mod auth;
mod user;
mod database;

use messenger::{MessengerServer, connect};

#[launch]
async fn rocket() -> _ {
    let messenger = Arc::new(Mutex::new(MessengerServer::new()));

    database::init().await.unwrap();

    rocket::build()
        .manage(messenger)
        .mount("/", routes![
            connect,
            login,
            signup,
            login_options,
            signup_options,
        ])
        .mount("/static", FileServer::from("static"))
        .attach(rocket::fairing::AdHoc::on_response("CORS", |_, res| Box::pin(async move {
            res.set_header(rocket::http::Header::new(
                "Access-Control-Allow-Origin",
                "*"
            ));
            res.set_header(rocket::http::Header::new(
                "Access-Control-Allow-Methods",
                "GET, POST, OPTIONS"
            ));
            res.set_header(rocket::http::Header::new(
                "Access-Control-Allow-Headers",
                "*"
            ));
        })))
}
