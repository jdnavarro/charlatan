use std::collections::HashMap;

use super::{db, entity::Episode};
use crate::json_reply;
use sqlx::sqlite::SqlitePool;

pub(super) async fn get_progress(
    p: SqlitePool,
    e: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    json_reply(db::get_progress(p, e).await)
}

pub(super) async fn set_progress(
    p: SqlitePool,
    e: i32,
    prog: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    json_reply(db::set_progress(p, e, prog).await)
}

pub(super) async fn position(
    p: SqlitePool,
    id: i32,
    hm: HashMap<String, Option<i32>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let position = hm.get("position").ok_or(warp::reject::not_found())?;
    match position {
        None => json_reply(db::delete(&p, id).await),
        Some(pos) => json_reply(db::position(&p, id, *pos).await),
    }
}

pub(super) async fn list(p: SqlitePool) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: Obtain HashMap directly from sqlx stream
    json_reply(db::list(p).await.map(|v| {
        v.into_iter()
            .map(|e| (e.id, e))
            .collect::<HashMap<i32, Episode>>()
    }))
}

#[allow(dead_code)]
pub(super) async fn queue(p: SqlitePool) -> Result<impl warp::Reply, warp::Rejection> {
    json_reply(db::queue(p).await)
}
