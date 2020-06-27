use std::collections::HashMap;

use sqlx::sqlite::SqlitePool;

use super::{db, entity::Episode};
use crate::app::App;
use crate::json_reply;
use crate::response;

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

pub(super) async fn episode(
    token: String,
    id: i32,
    m: HashMap<String, Option<i32>>,
    app: App,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = || async {
        let _ = app.identify(&token)?;

        match m.get("progress") {
            Some(value) => {
                let r = match value {
                    None => app.episode.set_progress(id, 0).await?,
                    Some(progress) => app.episode.set_progress(id, *progress).await?,
                };
                Ok(warp::reply::json(&r))
            }
            None => match m.get("position") {
                Some(value) => {
                    let r = match value {
                        None => {
                            let _ = app.episode.dequeue(id).await?;
                            -1
                        }
                        Some(pos) => app.episode.position(id, *pos).await?,
                    };
                    Ok(warp::reply::json(&r))
                }
                None => response::bad(),
            },
        }
    };
    response::unify(response().await)
}

pub(super) async fn progress(
    token: String,
    e: i32,
    app: App,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = || async {
        let _ = app.identify(&token)?;
        let progress = app.episode.get_progress(e).await?;
        Ok(warp::reply::json(&progress))
    };
    response::unify(response().await)
}

#[allow(dead_code)]
pub(super) async fn queue(p: SqlitePool) -> Result<impl warp::Reply, warp::Rejection> {
    json_reply(db::queue(p).await)
}
