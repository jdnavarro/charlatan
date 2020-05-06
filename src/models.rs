use std::env;

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    sqlite::SqliteConnection,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use super::schema::{episode, podcast};

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
pub type PooledSqliteConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_pool() -> SqlitePool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);

    Pool::new(manager).expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable, Deserialize, Serialize, Clone)]
pub struct Podcast {
    pub id: i32, // TODO: uuid
    pub title: String,
    pub url: String,
}

#[derive(Insertable)]
#[table_name = "podcast"]
pub struct NewPodcast<'a> {
    pub title: &'a str,
    pub url: &'a str,
}

#[derive(Queryable, Deserialize, Clone, Serialize)]
pub struct Episode {
    pub id: i32, // TODO: uuid
    pub title: String,
    pub url: String,
    pub podcast_id: i32,
}

#[derive(Insertable)]
#[table_name = "episode"]
pub struct NewEpisode<'a> {
    pub title: &'a str,
    pub url: &'a str,
    pub podcast_id: &'a i32,
}
