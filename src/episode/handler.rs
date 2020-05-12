use std::convert::Infallible;

use diesel::prelude::{QueryDsl, RunQueryDsl};
use rss::Channel;
use warp::http::StatusCode;

use super::model::{Episode, NewEpisode};
use crate::podcast::model::Podcast;
use crate::schema;
use crate::PooledSqliteConnection;

pub async fn list(conn: PooledSqliteConnection) -> Result<impl warp::Reply, Infallible> {
    let results = schema::episode::table
        .order(schema::episode::id)
        .limit(20)
        .load::<Episode>(&conn)
        .expect("Error loading posts");
    Ok(warp::reply::json(&results))
}

pub async fn crawl(conn: PooledSqliteConnection) -> Result<impl warp::Reply, Infallible> {
    let podcasts = schema::podcast::table
        .load::<Podcast>(&conn)
        .expect("Error loading posts");

    // TODO async fetch
    // TODO insert in one SQL statement
    for podcast in podcasts {
        let channel = Channel::from_url(&podcast.url).unwrap();

        for episode_item in channel.items() {
            let new_episode = NewEpisode {
                title: &episode_item.title().unwrap(),
                url: &episode_item.enclosure().unwrap().url(),
                podcast_id: &podcast.id,
            };
            diesel::insert_into(schema::episode::table)
                .values(&new_episode)
                .execute(&conn)
                .expect("Error saving new episode");
        }
    }
    Ok(StatusCode::CREATED)
}
