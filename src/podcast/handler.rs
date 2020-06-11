use std::collections::HashMap;

use sqlx::sqlite::SqlitePool;

use super::db;
use crate::json_reply;

pub(crate) async fn list(p: SqlitePool) -> Result<impl warp::Reply, warp::Rejection> {
    json_reply(db::list(p).await)
}

pub(super) async fn get(p: SqlitePool, id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let podcast = db::get(p, id)
        .await
        .map_err(|_| warp::reject::not_found())?;
    Ok(podcast)
}

pub(super) async fn add(
    p: SqlitePool,
    m: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let src = m.get("url").ok_or_else(warp::reject::not_found)?;

    // TODO: Report and skip errors
    let channel = rss::Channel::from_url(&src).unwrap();
    // TODO: Insert episodes here
    json_reply(db::add(p, &src, channel.title(), channel.image().unwrap().url()).await)
}
