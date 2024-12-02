use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use crate::database::DB;

use chrono::Utc;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: RecordId,
    pub username: String,
    pub passhash: String,
    pub displayname: String,
    pub joined: i64,
}

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub matches: bool,
    pub user_id: RecordId,
}

impl User {
    pub async fn authenticate(username: String, password: String) -> Result<AuthResponse, ()> {

        match DB
            .query("
                LET $user = (SELECT id, passhash FROM User WHERE username = $username)[0];
                RETURN {
                    matches: crypto::argon2::compare($user.passhash, $password),
                    user_id: $user.id
                }"
            )
            .bind(("username", username))
            .bind(("password", password))
            .await
            .map_err(|_| ())?
            .take::<Option<AuthResponse>>(0)
        {
            Ok(Some(response)) => Ok(response),
            Ok(None) => {
                println!("User not found");
                Err(())
            }
            Err(e) => {
                println!("Error authenticating user: {}", e);
                Err(())
            }
            _ => Err(()),
        }
    }

    pub async fn create(username: String, password: String) -> Result<String, ()> {
        match DB
            .query("
                CREATE User:uuid() SET 
                    username = $username, 
                    displayname = $displayname, 
                    passhash = crypto::argon2::generate($passhash), 
                    joined = $joined;
                SELECT * FROM User WHERE username = $username
            ")
            .bind(("username", username.clone()))
            .bind(("passhash", password))
            .bind(("displayname", username))
            .bind(("joined", Utc::now().timestamp()))
            .await
            .unwrap()
            .take::<Option<User>>(0)
        {
            Ok(Some(user)) => {
                let k = user.id.key().to_string();
                println!("Created User: {}", k);
                Ok(k)
            },
            _ => Err(())
        }
    }
}