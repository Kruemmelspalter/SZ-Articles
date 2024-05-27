use std::ops::FromResidual;

use rocket::{
    http::Status,
    post,
    response::{self, Responder},
    serde::{json::Json, Deserialize},
    Response, State,
};
use rocket_errors::eyre::{self, EyreReport};
use sqlx::query;

use crate::{database::DbConnection, user::authentication, JwtKey};

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
    Error(EyreReport),
}

impl<'a> Responder<'a, 'a> for LoginResponder {
    fn respond_to(self, request: &'a rocket::Request<'_>) -> response::Result<'a> {
        use LoginResponder::*;
        match self {
            InvalidPassword => Response::build_from("Invalid Credentials".respond_to(request)?)
                .status(Status::Forbidden)
                .ok(),
            UserNotFound => Response::build_from("User Not Found".respond_to(request)?)
                .status(Status::Forbidden)
                .ok(),
            Ok(t) => t.respond_to(request),
            Error(e) => Response::build_from(e.respond_to(request)?)
                .status(Status::InternalServerError)
                .ok(),
        }
    }
}

impl<T, E: Into<EyreReport>> FromResidual<Result<T, E>> for LoginResponder {
    fn from_residual(residual: Result<T, E>) -> Self {
        if let Err(e) = residual {
            LoginResponder::Error(e.into())
        } else {
            unreachable!()
        }
    }
}

#[post("/login", data = "<data>")]
pub async fn login(
    data: Json<Credentials<'_>>,
    conn: &State<DbConnection>,
    key: &State<JwtKey>,
) -> LoginResponder {
    let res = match query!(
        "select id, pw_hash from User where email = ? or username = ? or id = ?",
        data.identifier,
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
            _ => return LoginResponder::Error(e.into()),
        },
    };

    let pw_hash = authentication::hash_password(data.password);
    match res {
        None => LoginResponder::UserNotFound,
        Some((_, check_hash)) if check_hash != pw_hash => LoginResponder::InvalidPassword,
        Some((id, _)) => LoginResponder::Ok(authentication::generate_token(id, key)?),
    }
}
