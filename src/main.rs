#![feature(try_trait_v2)]

use hmac::{Hmac, Mac};
use rocket_errors::eyre::Result;
use user::authentication::JwtKey;

mod database;
mod user;

#[rocket::main]
async fn main() -> Result<()> {
    let conn = database::DbConnection::new("database.sqlite").await?;

    let jwt_key = JwtKey(Hmac::new_from_slice(b"amogus")?);

    let _instance = rocket::build() //
        .mount("/user", user::get_routes()
        .manage(conn)
        .manage(jwt_key)
        .launch()
        .await?;

    Ok(())
}
