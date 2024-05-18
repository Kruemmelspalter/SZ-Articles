use base64::prelude::*;
use jwt::{SignWithKey, VerifyWithKey};
use regex::Regex;
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Request,
};
use rocket_errors::eyre;
use sha2::{Digest, Sha512};

use hmac::Hmac;

pub struct JwtKey(pub Hmac<Sha512>);

pub fn hash_password(pw: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(pw);
    let pw_hash = hasher.finalize();
    BASE64_STANDARD.encode(pw_hash)
}

pub fn generate_token(id: i64, key: &JwtKey) -> eyre::Result<String> {
    Ok(id.sign_with_key(&key.0)?)
}

pub fn validate_token(token: &str, key: &JwtKey) -> eyre::Result<i64> {
    Ok(token.verify_with_key(&key.0)?)
}

pub struct Authentication(pub i64);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authentication {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let jwt_key = req.rocket().state::<JwtKey>().unwrap();
        let auth = match req.headers().get_one("Authorization") {
            None => {
                return request::Outcome::Error((Status::Unauthorized, "".to_owned()));
            }
            Some(a) => a,
        };

        let re = Regex::new(r"^(?:Bearer )?([\w\-]*\.[\w\-]*\.[\w\-]*)$").unwrap();
        let captures = match re.captures(auth) {
            None => return request::Outcome::Error((Status::BadRequest, "".to_owned())),
            Some(c) => c,
        };

        let token = &captures[1];

        match validate_token(token, jwt_key) {
            Ok(i) => request::Outcome::Success(Authentication(i)),
            Err(e) => request::Outcome::Error((Status::BadRequest, e.0.to_string())),
        }
    }
}
