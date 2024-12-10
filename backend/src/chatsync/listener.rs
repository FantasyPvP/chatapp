use std::{fs, os::unix::fs::PermissionsExt, path::Path};
use rocket::{get, serde::json::Json};
use rocket::tokio::{
    self,
    net::{UnixListener, UnixStream},
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::messenger::MessengerServer;

const SOCKET_PATH: &str = "/tmp/minecraft_chat/chat.sock";
const SOCKET_DIR: &str = "/tmp/minecraft_chat";

#[derive(Deserialize)]
struct MinecraftMessage {
    player: String,
    content: String,
}

#[derive(Serialize)]
struct Response {
    status: String,
}

#[derive(Serialize)]
pub struct Status {
    status: String,
    timestamp: i64,
}

// This function ensures the socket directory exists with correct permissions
pub fn init_socket() {
    // Create socket directory if it doesn't exist
    let socket_dir = "/tmp/minecraft_chat";
    if !std::path::Path::new(socket_dir).exists() {
        fs::create_dir(socket_dir).expect("Failed to create socket directory");
        // Set directory permissions to 755 (rwxr-xr-x)
        fs::set_permissions(socket_dir, fs::Permissions::from_mode(0o755))
            .expect("Failed to set directory permissions");
    }
}

pub fn get_socket_path() -> String {
    "/tmp/minecraft_chat/chat.sock".to_string()
}

pub async fn start_unix_socket(messenger: Arc<Mutex<MessengerServer>>) {
    // Ensure the socket directory exists with correct permissions
    if !Path::new(SOCKET_DIR).exists() {
        fs::create_dir(SOCKET_DIR).expect("Failed to create socket directory");
        fs::set_permissions(SOCKET_DIR, fs::Permissions::from_mode(0o755))
            .expect("Failed to set directory permissions");
    }

    // Remove existing socket file if it exists
    if Path::new(SOCKET_PATH).exists() {
        fs::remove_file(SOCKET_PATH).expect("Failed to remove existing socket");
    }

    // Create and bind to the Unix socket
    let listener = UnixListener::bind(SOCKET_PATH).expect("Failed to bind to Unix socket");
    
    // Set socket file permissions to 777 for testing (adjust as needed)
    fs::set_permissions(SOCKET_PATH, fs::Permissions::from_mode(0o777))
        .expect("Failed to set socket permissions");

    println!("Unix socket listening at {}", SOCKET_PATH);

    while let Ok((stream, _)) = listener.accept().await {
        let messenger = messenger.clone();
        tokio::spawn(async move {
            handle_connection(stream, messenger).await;
        });
    }
}

async fn handle_connection(mut stream: UnixStream, messenger: Arc<Mutex<MessengerServer>>) {
    let mut buffer = [0; 1024];
    
    match stream.read(&mut buffer).await {
        Ok(n) => {
            if let Ok(message_str) = String::from_utf8(buffer[..n].to_vec()) {
                if let Ok(message) = serde_json::from_str::<MinecraftMessage>(&message_str) {
                    // Forward the message to the chat system
                    let response = handle_minecraft_message(message, messenger).await;
                    
                    // Send response
                    if let Ok(response_json) = serde_json::to_string(&response) {
                        let _ = stream.write_all(response_json.as_bytes()).await;
                    }
                }
            }
        }
        Err(e) => eprintln!("Error reading from Unix socket: {}", e),
    }
}

async fn handle_minecraft_message(msg: MinecraftMessage, messenger: Arc<Mutex<MessengerServer>>) -> Response {
    // Create a real-time message
    use crate::messenger::RealTimeMessage;
    use std::time::{SystemTime, UNIX_EPOCH};

    let message = RealTimeMessage {
        message_id: 0,
        user_id: "minecraft".to_string(),
        display_name: msg.player,
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64,
        content: msg.content,
    };

    // Send the message to channel 1 (or adjust as needed)
    messenger.lock().await.send(1, message).await;

    Response {
        status: "ok".to_string(),
    }
}

#[get("/minecraft/chat")]
pub fn chat() -> Json<Status> {
    Json(Status {
        status: "ok".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    })
}
