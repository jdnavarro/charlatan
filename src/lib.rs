#[macro_use]
extern crate diesel;

use self::models::{NewEpisode, NewPodcast, Podcast};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use rss::Channel;
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
    use schema::podcast;

    let new_podcast = NewPodcast { title, url };

    diesel::insert_into(podcast::table)
        .values(&new_podcast)
        .execute(conn)
        .expect("Error saving new podcast")
}

pub fn fetch_all_episodes(conn: &SqliteConnection) {
    use schema::episode;
    use schema::podcast::dsl::podcast;

    let podcasts = podcast.load::<Podcast>(conn).expect("Error loading posts");

    for p in podcasts {
        let channel = Channel::from_url(&p.url).unwrap();

        for e in channel.items() {
            let new_episode = NewEpisode {
                title: &e.title().unwrap(),
                url: &e.enclosure().unwrap().url(),
                podcast_id: &p.id,
            };
            diesel::insert_into(episode::table)
                .values(&new_episode)
                .execute(conn)
                .expect("Error saving new episode");
        }
    }
}
