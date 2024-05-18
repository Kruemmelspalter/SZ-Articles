use std::str::FromStr;

use rocket::futures::lock::{Mutex, MutexGuard};
use rocket_errors::eyre::Result;
use sqlx::{prelude::*, sqlite::SqliteConnectOptions, SqliteConnection};

pub struct DbConnection(Mutex<SqliteConnection>);

impl DbConnection {
    pub async fn new(url: &str) -> Result<Self> {
        let url: String = if !url.starts_with("sqlite:") {
            url.to_owned()
        } else {
            format!("sqlite://{url}")
        };

        Ok(Self(Mutex::new(
            SqliteConnectOptions::from_str(&url)? //
                .connect()
                .await?,
        )))
    }
    pub async fn lock(&self) -> MutexGuard<SqliteConnection> {
        self.0.lock().await
    }
}
