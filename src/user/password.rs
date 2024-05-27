use std::ops::FromResidual;

use rocket::{
    http::Status,
    put,
    response::Responder,
    serde::{json::Json, Deserialize, Serialize},
    Response, State,
};
use rocket_errors::eyre;
use sqlx::query;

use crate::{
    database::DbConnection,
    user::authentication::{hash_password, Authentication},
};

use super::validate::is_valid_password;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PasswordChange<'a> {
    old_password: &'a str,
    new_password: &'a str,
}

pub enum PasswordChangeResponder {
    Ok,
    WrongOldPassword,
    InvalidNewPassword,
    Error(eyre::EyreReport),
}

impl<'a> Responder<'a, 'a> for PasswordChangeResponder {
    fn respond_to(self, request: &'a rocket::Request<'_>) -> rocket::response::Result<'a> {
        use PasswordChangeResponder::*;
        match self {
            Ok => Response::build_from(().respond_to(request)?)
                .status(Status::NoContent)
                .ok(),
            WrongOldPassword => Response::build_from("wrong password".respond_to(request)?)
                .status(Status::Forbidden)
                .ok(),
            InvalidNewPassword => Response::build_from("invalid password".respond_to(request)?)
                .status(Status::BadRequest)
                .ok(),
            Error(e) => Response::build_from(e.respond_to(request)?)
                .status(Status::InternalServerError)
                .ok(),
        }
    }
}

impl<T> FromResidual<eyre::Result<T>> for PasswordChangeResponder {
    fn from_residual(residual: eyre::Result<T>) -> Self {
        if let Err(e) = residual {
            PasswordChangeResponder::Error(e)
        } else {
            unreachable!()
        }
    }
}

#[put("/password", data = "<data>", format = "application/json")]
pub async fn change_password(
    auth: Authentication,
    data: Json<PasswordChange<'_>>,
    conn: &State<DbConnection>,
) -> PasswordChangeResponder {
    if !is_valid_password(data.new_password) {
        return PasswordChangeResponder::InvalidNewPassword;
    }

    let old_pw_hash = hash_password(data.old_password);
    if query!("select pw_hash from User where id = ?", auth.0)
        .map(|r| r.pw_hash != old_pw_hash)
        .fetch_one(&mut *conn.lock().await)
        .await
        .map_err(|e| e.into())?
    {
        return PasswordChangeResponder::WrongOldPassword;
    }

    let new_pw_hash = hash_password(data.new_password);
    query!(
        "update User set pw_hash = ? where id = ?",
        new_pw_hash,
        auth.0
    )
    .execute(&mut *conn.lock().await)
    .await
    .map_err(|e| e.into())?;

    PasswordChangeResponder::Ok
}
