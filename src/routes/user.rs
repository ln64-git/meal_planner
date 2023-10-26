use jsonwebtoken::{ encode, Header, EncodingKey };
use rocket_contrib::json::Json;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String,
    password: String,
}

#[post("/signin", data = "<user>")]
pub fn signin(user: Json<User>) -> Json<String> {
    let secret: &str = "your_secret_key";
    let user_claims = User {
        id: 1,
        username: user.username.to_string(),
        password: user.password.to_string(),
    };
    let token = encode(
        &Header::default(),
        &user_claims,
        &EncodingKey::from_secret(secret.as_ref())
    ).unwrap();
    Json(token)
}
