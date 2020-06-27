use std::collections::HashMap;

use sqlx::sqlite::SqlitePool;

use super::{db, entity::Episode};
use crate::app::App;
use crate::json_reply;
use crate::response;

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

pub(super) async fn list(token: String, app: App) -> Result<impl warp::Reply, warp::Rejection> {
    let response = || async {
        let _ = app.identify(&token)?;

        let episodes = app.episode.list().await.map(|v| {
            v.into_iter()
                .map(|e| (e.id, e))
                .collect::<HashMap<i32, Episode>>()
        })?;

        Ok(warp::reply::json(&episodes))
    };
    response::unify(response().await)
}

#[allow(dead_code)]
pub(super) async fn queue(p: SqlitePool) -> Result<impl warp::Reply, warp::Rejection> {
    json_reply(db::queue(p).await)
}
