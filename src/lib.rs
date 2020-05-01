#[macro_use]
extern crate diesel;

use self::models::NewPodcast;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_podcast(conn: &SqliteConnection, title: &str, url: &str) -> usize {
    use schema::podcasts;

    let new_podcast = NewPodcast { title, url };

    diesel::insert_into(podcasts::table)
        .values(&new_podcast)
        .execute(conn)
        .expect("Error saving new podcast")
}
