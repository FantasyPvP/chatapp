use rocket::{get, serde::json::Json};
use serde::Serialize;

#[get("/servers")]
pub fn servers() -> Json<Vec<Server>> {
    Json(vec![
        Server { id: "0".to_string(), name: "Test Server".to_string() },
        Server { id: "1".to_string(), name: "Test Server2".to_string() },
        Server { id: "2".to_string(), name: "Test Server3".to_string() },
        Server { id: "3".to_string(), name: "Test Server4".to_string() },
        Server { id: "4".to_string(), name: "Test Server5".to_string() },
        Server { id: "5".to_string(), name: "Test Server6".to_string() },
        Server { id: "6".to_string(), name: "Test Server7".to_string() },
    ])
}

#[derive(Serialize)]
struct Server {
    id: String,
    name: String,
}