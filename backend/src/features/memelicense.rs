
#[get("/license/<username>")]
pub async fn get_meme_license(username: String) -> String {
    format!("Meme license for {}", username)
}   