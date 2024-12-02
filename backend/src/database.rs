use std::sync::LazyLock;

use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, Surreal};

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(|| Surreal::init());

pub async fn init() -> Result<(), surrealdb::Error> {
    DB.connect::<Ws>("localhost:8001").await?;
    
    DB.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    DB.use_ns("database").use_db("database").await?;
    Ok(())
}