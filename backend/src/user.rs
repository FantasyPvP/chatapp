use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

use crate::{database::DB, BackendError};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: RecordId,
    pub username: String,
    pub email: String,
    pub passhash: String,
    pub displayname: String,
    pub joined: Datetime,
}

#[derive(Debug, Deserialize, Serialize)]
struct Entity {
    id: RecordId,
    deleted: bool,
    displayname: String,
    joined: Datetime,
    username: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Human {
    id: RecordId,
    email: String,
    passhash: String
}

impl User {
    pub async fn signup(
        username: String, 
        email: String, 
        password: String, 
        displayname: String
    ) -> Result<String, BackendError> {

        // let entity = DB
        //     .create("Entity")
        //     .content(Entity {
        //         id: RecordId::from(("Entity", Id::rand())),
        //         deleted: false,
        //         displayname: displayname,
        //         joined: Datetime::now(),
        //         username: username,   
        //     })
        //     .await;



        return Ok(String::new());

        // match DB
        //     .query("
        //         LET $entity_id = CREATE Entity CONTENT {
        //             deleted: false,
        //             displayname: $displayname,
        //             joined: time::now(),
        //             username: $username,
        //         } RETURN AFTER.id;

        //         LET $human_id = CREATE Human CONTENT {
        //             email: $email,
        //             passhash: crypto::argon2::generate($password),
        //         } RETURN AFTER.id;

        //         LET $node1 = type::thing('Entity', $entity_id);
        //         LET $node2 = type::thing('Human', $human_id);
        //         RELATE $node2 -> HasEntity -> $node1;

        //         LET $session_token = CREATE SessionToken CONTENT {
        //             id: rand::uuid::v4(),
        //             created: time::now(),
        //             expires: time::now() + 7d,
        //             token: rand::string(64),
        //             user: type::thing('Human', $human_id),
        //         } RETURN AFTER.token;

        //         RETURN $session_token;
        //     ")
        //     .bind(("username", username))
        //     .bind(("email", email))
        //     .bind(("password", password))
        //     .bind(("displayname", displayname))
        //     .await
        //     .unwrap()
        //     .take::<Option<String>>(0)
        //     .map_err(|e| BackendError::from(e)) 
        // {
        //     Ok(Some(user)) => Ok(user),
        //     Ok(None) => panic!("NO TOKEN RETURNED"),
        //     _ => Err(BackendError::DbError("Failed to create user".to_string())),
        // }
    }

    pub async fn login(username: String, password: String) -> Result<String, BackendError> {

        match DB
            .query("
                LET $human = (SELECT id, passhash FROM Human WHERE email = $email)[0];

                IF !crypto::argon2::compare($human.passhash, $password) {
                  RETURN NONE;
                };

                LET $session_token = rand::string(64);

                CREATE SessionToken CONTENT {
                    id: rand::uuid::v4(),
                    created: time::now(),
                    expires: time::now() + 7d,
                    token: $session_token,
                    user: type::thing('Human', $human.id),
                };

                RETURN $session_token;
            ")
            .bind(("email", username))
            .bind(("password", password))
            .await
            .unwrap()
            .take::<Option<String>>(0)
            .map_err(|e| BackendError::from(e)) 
        {
            Ok(Some(user)) => Ok(user),
            _ => Err(BackendError::DbError("Failed to create user".to_string())),
        }
    }
}