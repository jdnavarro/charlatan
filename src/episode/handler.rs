use std::collections::HashMap;

use super::{db, entity::Episode};
use crate::auth::handler::identify;
use crate::json_reply;
use crate::response::{self, Response};
use sqlx::sqlite::SqlitePool;

pub(super) async fn get_progress(
    p: SqlitePool,
    e: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    json_reply(db::get_progress(p, e).await)
}

pub(super) async fn episode(
    p: SqlitePool,
    id: i32,
    m: HashMap<String, Option<i32>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match m.get("progress") {
        Some(progress) => match progress {
            None => json_reply(db::set_progress(p.clone(), id, 0).await),
            Some(prog) => json_reply(db::set_progress(p.clone(), id, *prog).await),
        },
        None => match m.get("position") {
            Some(position) => match position {
                None => json_reply(db::dequeue(p.clone(), id).await),
                Some(pos) => json_reply(db::position(p, id, *pos).await),
            },
            None => Err(warp::reject::not_found()),
        },
    }
}

pub(super) async fn list(
    p: SqlitePool,
    jwt_secret: String,
    token: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = || async {
        let _ = identify(&jwt_secret, &token)?;

        let episodes = db::list(p).await.map(|v| {
            v.into_iter()
                .map(|e| (e.id, e))
                .collect::<HashMap<i32, Episode>>()
        })?;

        Ok(warp::reply::json(&episodes))
    };
    response::reply(response().await)
}

#[allow(dead_code)]
pub(super) async fn queue(p: SqlitePool) -> Result<impl warp::Reply, warp::Rejection> {
    json_reply(db::queue(p).await)
}
