#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABSE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::esablish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}