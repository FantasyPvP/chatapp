use std::sync::LazyLock;
use serde::{Deserialize, Serialize};
use toml::Table;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    socket_path: String,
    channel_id: u64,
    pub dev_url: String,
    pub prod_url: String
}

impl Config {
    pub fn load() -> Config {

        let content = match std::fs::read_to_string("Deploy.toml") {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read Deploy.toml: {}", e);
                return Config::default();
            },
        };

        let table: Table = match content.parse::<Table>() {
            Ok(table) => table,
            Err(e) => {
                eprintln!("Failed to parse Deploy.toml: {}", e);
                return Config::default();
            },
        };
        
        let mut config = Config::default();

        if let Some(socket_path) = table.get("chatsync") {
            if let Some(socket_path) = socket_path.as_table() {
                if let Some(path) = socket_path.get("socket_path") {
                    config.socket_path = path.to_string();
                }
            }
        }

        // get urls

        if let Some(server) = table.get("server") {
            if let Some(server) = server.as_table() {
                if let Some(url) = server.get("dev") {
                    if let Some(url) = url.as_table() {
                        if let Some(url) = url.get("url") {
                            config.dev_url = url.to_string();
                            println!("OK!")
                        }
                    }
                }
                if let Some(url) = server.get("prod") {
                    if let Some(url) = url.as_table() {
                        if let Some(url) = url.get("url") {
                            config.prod_url = url.to_string();
                        }
                    }
                }
            }
        }

        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            socket_path: "/tmp/minecraft_chat/chat.sock".to_string(),
            channel_id: 1315044374127841374,
            dev_url: "http://localhost:8080".to_string(),
            prod_url: "https://zxq5.dev".to_string()
        }
    }
}