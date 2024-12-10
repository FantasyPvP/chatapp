pub mod listener;

use listener::init_socket;
use rocket::fairing::Fairing;
use std::sync::Arc;
use rocket::tokio::sync::Mutex;
use crate::messenger::MessengerServer;

pub struct ChatSyncFairing {
    pub messenger: Arc<Mutex<MessengerServer>>,
}

#[rocket::async_trait]
impl Fairing for ChatSyncFairing {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Chat Sync",
            kind: rocket::fairing::Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, _rocket: &rocket::Rocket<rocket::Orbit>) {
        let messenger = self.messenger.clone();

        init_socket();

        tokio::spawn(async move {
            listener::start_unix_socket(messenger).await;
        });
    }
}
