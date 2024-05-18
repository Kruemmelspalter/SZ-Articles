use hmac::{Hmac, Mac};
use rocket_errors::eyre::Result;
use user::auth::JwtKey;

mod database;
mod image;
mod user;

#[rocket::main]
async fn main() -> Result<()> {
    let conn = database::DbConnection::new("database.sqlite").await?;

    let jwt_key = JwtKey(Hmac::new_from_slice(b"amogus")?);

    let _instance = rocket::build() //
        .mount("/user", user::get_routes())
        .mount("/image", image::get_routes())
        .manage(conn)
        .manage(jwt_key)
        .launch()
        .await?;

    Ok(())
}
