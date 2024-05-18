use rocket::{get, post, routes, Route};

pub mod auth;
mod login;

#[get("/<id>")]
async fn get_user_meta(id: i64) {
    todo!()
}

pub fn get_routes() -> Vec<Route> {
    routes![get_user_meta, login::login]
}
