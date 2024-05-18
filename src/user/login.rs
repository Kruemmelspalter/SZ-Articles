use rocket::{
    http::Status,
    post,
    response::{self, Responder},
    serde::{json::Json, Deserialize},
    Response, State,
};
use rocket_errors::eyre::Result;
use sqlx::query;

use crate::{database::DbConnection, user::auth, JwtKey};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Credentials<'a> {
    identifier: &'a str,
    password: &'a str,
}

pub enum LoginResponder {
    InvalidPassword,
    UserNotFound,
    Ok(String),
}

impl<'a> Responder<'a, 'a> for LoginResponder {
    fn respond_to(self, request: &'a rocket::Request<'_>) -> response::Result<'a> {
        match self {
            LoginResponder::InvalidPassword => {
                Response::build_from("Invalid Credentials".respond_to(request)?)
                    .status(Status::new(403))
                    .ok()
            }
            LoginResponder::UserNotFound => {
                Response::build_from("User Not Found".respond_to(request)?)
                    .status(Status::new(403))
                    .ok()
            }
            LoginResponder::Ok(s) => s.respond_to(request),
        }
    }
}

#[post("/login", data = "<data>")]
pub async fn login(
    data: Json<Credentials<'_>>,
    conn: &State<DbConnection>,
    key: &State<JwtKey>,
) -> Result<LoginResponder> {
    let res = match query!(
        "select id, pw_hash from User where email = ? or username = ?",
        data.identifier,
        data.identifier
    )
    .map(|r| (r.id, r.pw_hash))
    .fetch_one(&mut *conn.inner().lock().await)
    .await
    {
        Ok(c) => Some(c),
        Err(e) => match e {
            sqlx::Error::RowNotFound => None,
            _ => return Err(e.into()),
        },
    };

    let pw_hash = auth::hash_password(data.password);
    match res {
        None => Ok(LoginResponder::UserNotFound),
        Some((_, check_hash)) if check_hash != pw_hash => Ok(LoginResponder::InvalidPassword),
        Some((id, _)) => Ok(LoginResponder::Ok(auth::generate_token(id, key)?)), // TODO generate JWT
    }
}
