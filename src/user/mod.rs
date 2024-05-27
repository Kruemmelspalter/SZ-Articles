use rocket::{get, routes, Route};

pub mod authentication;
mod login;
mod password;
mod register;
mod validate;

#[get("/<id>")]
async fn get_user_meta(id: i64) {
    todo!()
}

pub fn get_routes() -> Vec<Route> {
    routes![password::change_password, login::login, register::register,]
}
