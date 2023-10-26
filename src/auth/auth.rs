use rocket::{ Request, Data };
use rocket::data::ByteUnit;
use rocket::data::FromData;
use rocket::data::ByteUnit;
use rocket::data::{ ByteUnit, FromData };
use rocket::outcome::Outcome;
use rocket::http::Status;
use rocket::request::{ self, FromRequest, Request };
use rocket_contrib::json::Json;
use jsonwebtoken::{ decode, encode, Header, Validation, EncodingKey, DecodingKey };
use std::collections::HashMap;
use std::str;
use std::io::Read;
use std::sync::atomic::{ AtomicUsize, Ordering };

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct AuthenticatedUser(User);

impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => {
                if let Ok(user) = verify_token(token) {
                    Outcome::Success(AuthenticatedUser(user))
                } else {
                    Outcome::Failure((Status::Unauthorized, ()))
                }
            }
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

fn verify_token(token: &str) -> Result<User, ()> {
    let secret = "your_secret_key";
    let validation = Validation {
        validate_exp: false,
        ..Default::default()
    };
    let token_data = decode::<User>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation
    );
    match token_data {
        Ok(data) => Ok(data.claims),
        Err(_) => Err(()),
    }
}
