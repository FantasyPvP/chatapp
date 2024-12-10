use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rand::{thread_rng, Rng};
use rocket::{http::{Cookie, CookieJar, Status}, options, post, request::{FromRequest, Outcome}, serde::json::Json, Request};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use surrealdb::RecordId;

use crate::{database::DB, user::User};

#[derive(Deserialize)]
pub struct UserLoginForm {
    username: String,
    password: String
}

#[options("/login")]
pub fn login_options() -> Status {
    Status::Ok
}


#[post("/login", data = "<form>")]
pub async fn login(form: Json<UserLoginForm>, jar: &CookieJar<'_>) -> Status {
    match User::login(form.username.clone(), form.password.clone()).await {
        Ok(token) => {
            println!("logged in");
            jar.add_private(Cookie::new("auth-token", token));
            Status::Ok
        }
        Err(e) => {
            println!("login failed: {}", e);
            Status::Unauthorized
        }
    }
}

#[derive(Deserialize)]
pub struct UserSignupForm {
    username: String,
    password: String,
    token: String,
}


#[options("/signup")]
pub fn signup_options() -> Status {
    Status::Ok
}

#[post("/signup", data = "<user>")]
pub async fn signup(user: Json<UserSignupForm>, jar: &CookieJar<'_>) -> Status {    
    match User::signup(
        user.username.clone(), 
        user.username.clone(),
        user.password.clone(), 
        user.username.clone()
    ).await {
        Ok(token) => {
            println!("signed up");
            jar.add_private(Cookie::new("auth-token", token));
            Status::Ok
        }
        Err(e) => {
            println!("signup failed: {}", e);
            Status::Conflict
        }
    }
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
        if let Some(cookie) = req.cookies().get_private("auth-token") {
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