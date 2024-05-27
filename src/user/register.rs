use std::ops::FromResidual;

use crate::{
    database::DbConnection,
    user::authentication::{self, JwtKey},
};
use rocket::{
    http::Status,
    post,
    response::Responder,
    serde::{json::Json, Deserialize, Serialize},
    Response, State,
};
use rocket_errors::eyre::{self, EyreReport};
use sqlx::query;

use super::validate::{is_valid_email, is_valid_password, is_valid_username};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterMeta<'a> {
    username: &'a str,
    password: &'a str,
    email: &'a str,
    display_name: Option<&'a str>,
}

pub enum RegisterResponder {
    UsernameExists,
    EmailExists,
    PasswordInvalid,
    EmailInvalid,
    UsernameInvalid,
    Ok(String),
    Error(EyreReport),
}

impl<'a> Responder<'a, 'a> for RegisterResponder {
    fn respond_to(self, request: &'a rocket::Request<'_>) -> rocket::response::Result<'a> {
        use RegisterResponder::*;
        match self {
            UsernameExists => {
                Response::build_from("This username already exists".respond_to(request)?)
                    .status(Status::BadRequest)
                    .ok()
            }
            EmailExists => Response::build_from("This email already exists".respond_to(request)?)
                .status(Status::BadRequest)
                .ok(),
            PasswordInvalid => {
                Response::build_from("This password is invalid (at least six chars, at least one letter, number, and symbol".respond_to(request)?) // TODO
                    .status(Status::BadRequest)
                    .ok()
            }
            UsernameInvalid => {
                Response::build_from(
                    "This username is invalid (can't be a number, can't be someone else's email)"
                        .respond_to(request)?,
                ) // TODO
                .status(Status::BadRequest)
                .ok()
            }
            EmailInvalid => {
                Response::build_from("This email is invalid".respond_to(request)?) // TODO
                    .status(Status::BadRequest)
                    .ok()
            }
            Error(e) => Response::build_from(e.respond_to(request)?)
                .status(Status::InternalServerError)
                .ok(),
            Ok(t) => Response::build_from(t.respond_to(request)?)
                .status(Status::Created)
                .ok(),
        }
    }
}

impl<T> FromResidual<eyre::Result<T>> for RegisterResponder {
    fn from_residual(residual: eyre::Result<T>) -> Self {
        if let Err(e) = residual {
            RegisterResponder::Error(e)
        } else {
            unreachable!()
        }
    }
}

#[post("/register", data = "<data>", format = "application/json")]
pub async fn register(
    data: Json<RegisterMeta<'_>>,
    conn: &State<DbConnection>,
    key: &State<JwtKey>,
) -> RegisterResponder {
    if !is_valid_password(data.password) {
        println!("{}: {}", is_valid_password(data.password), data.password); // TODO
        return RegisterResponder::PasswordInvalid;
    }

    if !is_valid_email(data.email) {
        return RegisterResponder::EmailInvalid;
    }

    if !is_valid_username(data.username) {
        return RegisterResponder::UsernameInvalid;
    }

    let pw_hash = authentication::hash_password(data.password);
    match query!(
        "insert into User (username, pw_hash, email, display_name) values (?, ?, ?, ?) returning id",
        data.username,
        pw_hash,
        data.email,
        data.display_name
    )
    .map(|r| r.id)
    .fetch_one(&mut *conn.lock().await)
    .await
    {
        Ok(id) => RegisterResponder::Ok(authentication::generate_token(id, key)?),
        Err(e) => match e {
            sqlx::Error::Database(f) => {
                println!("{}", f.message());
                match f.message() {
                    "username is a number" | "username is a different user's email or id" => {
                        RegisterResponder::UsernameInvalid
                    }
                    "UNIQUE constraint failed: User.username" => RegisterResponder::UsernameExists,
                    "UNIQUE constraint failed: User.email" => RegisterResponder::EmailExists,
                    _ => panic!(),
                }
            }
            _ => RegisterResponder::Error(e.into()),
        },
    }
}
