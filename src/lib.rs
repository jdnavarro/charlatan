#[macro_use]
extern crate diesel;

use std::env;

use diesel::prelude::{Connection, RunQueryDsl};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use rss::Channel;

use models::{NewEpisode, NewPodcast, Podcast};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_podcast(conn: &SqliteConnection, title: &str, url: &str) -> usize {
    let new_podcast = NewPodcast { title, url };

    diesel::insert_into(schema::podcast::table)
        .values(&new_podcast)
        .execute(conn)
        .expect("Error saving new podcast")
}

pub fn fetch_all_episodes(conn: &SqliteConnection) {
    let podcasts = schema::podcast::table
        .load::<Podcast>(conn)
        .expect("Error loading posts");

    for p in podcasts {
        let channel = Channel::from_url(&p.url).unwrap();

        for e in channel.items() {
            let new_episode = NewEpisode {
                title: &e.title().unwrap(),
                url: &e.enclosure().unwrap().url(),
                podcast_id: &p.id,
            };
            diesel::insert_into(schema::episode::table)
                .values(&new_episode)
                .execute(conn)
                .expect("Error saving new episode");
        }
    }
}
