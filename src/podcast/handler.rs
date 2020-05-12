use std::collections::HashMap;
use std::convert::Infallible;

use diesel::prelude::{QueryDsl, RunQueryDsl};
use warp::http::StatusCode;

use super::model::{NewPodcast, Podcast};
use crate::schema;
use crate::PooledSqliteConnection;

pub async fn list(conn: PooledSqliteConnection) -> Result<impl warp::Reply, Infallible> {
    let results = schema::podcast::table
        .load::<Podcast>(&conn)
        .expect("Error loading posts");
    Ok(warp::reply::json(&results))
}

pub async fn get(id: i32, conn: PooledSqliteConnection) -> Result<impl warp::Reply, Infallible> {
    let results = schema::podcast::table
        .find(id)
        .load::<Podcast>(&conn)
        .expect("Error loading posts");
    Ok(warp::reply::json(&results))
}

pub async fn add(
    hm: HashMap<String, String>,
    conn: PooledSqliteConnection,
) -> Result<impl warp::Reply, Infallible> {
    match (hm.get("title"), hm.get("url")) {
        (Some(title), Some(url)) => {
            let new_podcast = NewPodcast { title, url };

            let rows_inserted = diesel::insert_into(schema::podcast::table)
                .values(&new_podcast)
                .execute(&conn)
                .expect("Error saving new podcast");

            Ok(warp::reply::with_status(
                warp::reply::json(&rows_inserted),
                StatusCode::CREATED,
            ))
        }
        p => Ok(warp::reply::with_status(
            warp::reply::json(&p),
            StatusCode::BAD_REQUEST,
        )),
    }
}
