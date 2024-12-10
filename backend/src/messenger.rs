use std::{collections::HashMap, sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use chrono::Utc;
use rocket::{
    futures::{
        channel::mpsc, 
        stream::{SplitSink, SplitStream}, 
        SinkExt, 
        StreamExt
    }, 
    tokio::sync::Mutex, 
    serde::json::to_string,
    get, 
    http::Status, 
    Shutdown
};

use serde::Serialize;
use rocket_ws::{Channel, WebSocket, stream::DuplexStream};
use surrealdb::{RecordId, Uuid, Datetime};
use crate::{auth::SessionTokenGuard, user::User};

#[get("/messenger/connect/<channel_id>/<user_id>")]
pub async fn connect<'r> (
    // user: SessionTokenGuard,
    user_id: String,
    ws: WebSocket,
    messenger: &'r rocket::State<Arc<Mutex<MessengerServer>>>,
    channel_id: i32,
    mut shutdown: Shutdown,
) -> Result<Channel<'r>, Status> {

    let messenger = Arc::clone(messenger.inner());

    Ok(ws.channel(move | channel| {
        Box::pin(async move {

            let user = User {
                id: RecordId::from_table_key("User", Uuid::new_v4().to_string()),
                email: "".to_string(),
                username: user_id,
                passhash: "test".to_string(),
                displayname: "test".to_string(),
                joined: Datetime::from(Utc::now()),
            };

            let (sender, receiver) = mpsc::channel::<RealTimeMessage>(100);
            let (ws_sender, ws_receiver) = channel.split();

            messenger.lock().await.register(user.id.clone(), channel_id, sender);

            tokio::select! {
                _ = inbound_message(ws_receiver, messenger.clone(), channel_id, &user) => {},
                _ = outbound_message(ws_sender, receiver) => {},
                _ = &mut shutdown => {},
            }

            messenger.lock().await.deregister(user.id);
            Ok(())
        })
    }))
}





pub async fn inbound_message(
    mut ws_receiver: SplitStream<DuplexStream>, 
    messenger: Arc<Mutex<MessengerServer>>,
    channel_id: i32,
    user: &User,
) {
    while let Some(Ok(msg)) = ws_receiver.next().await {
        if let rocket_ws::Message::Text(text) = msg {
            
            let message = RealTimeMessage {
                message_id: 0,
                user_id: user.id.key().to_string().trim_start_matches('⟨').trim_end_matches('⟩').to_string(),
                display_name: user.username.clone(),
                created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64,
                content: text,
            };

            messenger.lock().await.send(channel_id, message).await;
        } else if let rocket_ws::Message::Binary(b) = msg {
            println!("recieved binary message: {}", String::from_utf8(b).unwrap());
        } else {
            println!("OTHER");
        }
    }
}

pub async fn outbound_message(
    mut ws_sender: SplitSink<DuplexStream, rocket_ws::Message>, 
    mut receiver: mpsc::Receiver<RealTimeMessage>
) {
    while let Some(msg) = receiver.next().await {
        if let Err(e) = ws_sender.send(to_string(&msg).unwrap().into()).await {
            println!("Failed to send message to client\nError: {}", e);
            return;
        }
    }
}


type UserId = RecordId;
type ChannelId = i32;

pub struct MessengerServer {
    pub channels: HashMap<i32, HashMap<UserId, mpsc::Sender<RealTimeMessage>>> // map of the channel id to the channel object
}

impl MessengerServer {
    pub fn new() -> MessengerServer {
        MessengerServer {
            channels: HashMap::new(),
        }
    }

    pub fn register(&mut self, user_id: UserId, channel_id: ChannelId, sender: mpsc::Sender<RealTimeMessage>) {
        if let Some(channel) = self.channels.get_mut(&channel_id) {
            channel.insert(user_id, sender);
        } else {
            self.channels.insert(channel_id, HashMap::from([(user_id, sender)]));
        }
    }

    pub fn deregister(&mut self, user_id: UserId) {
        for (_, channel) in self.channels.iter_mut() {
            channel.remove(&user_id);
        }
    }

    pub async fn send(&mut self, channel_id: ChannelId, msg: RealTimeMessage) {
        if let Some(channel) = self.channels.get_mut(&channel_id) {
            for (_, sender) in channel.iter_mut() {
                if let Err(e) = sender.send(msg.clone()).await {
                    println!("Failed to send message to channel {}\nError: {}", channel_id, e);
                }
            }
        }
    }
}

#[derive(Serialize, Clone)]
pub struct RealTimeMessage {
    pub message_id: i32,
    pub user_id: String,
    pub display_name: String,
    pub created_at: i64,
    pub content: String,
}



