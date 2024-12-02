use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rand::{thread_rng, Rng};
use rocket::{http::{CookieJar, Status}, options, post, request::{FromRequest, Outcome}, serde::json::Json, Request};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use surrealdb::RecordId;

use crate::{database::DB, user::{
    User, AuthResponse
}};

#[derive(Deserialize)]
pub struct UserLoginForm {
    username: String,
    password: String
}

#[options("/login")]
pub fn login_options() -> Status {
    Status::Ok
}


#[post("/login", data = "<user>")]
pub async fn login(user: Json<UserLoginForm>, jar: &CookieJar<'_>) -> Status {

    println!("Logging in: {}", user.username);

    if let Ok(response) = User::authenticate(user.username.clone(), user.password.clone()).await {
        if response.matches {

            let token = SessionToken::new(response.user_id).await;
            jar.add_private(("auth", token.token));
            println!("success!");

            return Status::Ok
        } else {
            println!("does not match");
        }
    } else {
        println!("response err");
    }

    println!("failed!");
    return Status::Unauthorized
}

#[options("/signup")]
pub fn signup_options() -> Status {
    Status::Ok
}

#[post("/signup", data = "<user>")]
pub async fn signup(user: Json<UserLoginForm>, jar: &CookieJar<'_>) -> Status {

    println!("signing up: {}", user.username);

    User::create(user.username.clone(), user.password.clone()).await;

    login(user, jar).await
}

pub type SessionTokenGuard = User;

#[derive(Debug, Clone)]
pub struct SessionToken {
    token: String,
    created_at: i64,
    expires_at: i64,
    user_id: RecordId,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionTokenGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(cookie) = req.cookies().get_private("auth") {
            let token = cookie.value().to_string();
            return match DB
                .query("
                    SELECT * FROM User WHERE user_id = (SELECT user_id FROM SessionToken WHERE token = $1)[0]
                ") 
                .bind(("token", token))
                .await
                .unwrap()
                .take::<Option<User>>(0)
            {
                Ok(Some(user)) => Outcome::Success(user),
                _ => Outcome::Error((rocket::http::Status::Unauthorized, ())),
            }
        }
        Outcome::Error((rocket::http::Status::Unauthorized, ()))
    }
}

impl SessionToken {
    pub async fn new(user_id: RecordId) -> SessionToken {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let expiry = Duration::from_secs(7 * 24 * 60 * 60);
        let random_value: u32 = thread_rng().gen();
        let token = format!("{}-{}", current_time.as_secs(), random_value);
        let hashed = format!("{:x}", Sha256::digest(token.as_bytes()));

        println!("{}", hashed);

        let token = SessionToken {
            token: hashed,
            created_at: current_time.as_secs() as i64,
            expires_at: (current_time + expiry).as_secs() as i64,
            user_id,
        };

        let tok = token.clone();
        DB
            .query("
                CREATE SessionToken SET 
                    token = $token, 
                    created_at = $created_at, 
                    expires_at = $expires_at, 
                    user_id = $user_id
            ")
            .bind(("token", tok.token))
            .bind(("created_at", tok.created_at))
            .bind(("expires_at", tok.expires_at))
            .bind(("user_id", tok.user_id))
            .await
            .unwrap();

        token
    }
}   