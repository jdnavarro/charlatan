use std::collections::HashMap;

use super::entity::Episode;
use crate::app::App;
use crate::response;

pub(super) async fn list(_identity: String, app: App) -> Result<impl warp::Reply, warp::Rejection> {
    let response = || async {
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
    id: i32,
    _identity: String,
    m: HashMap<String, Option<i32>>,
    app: App,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = || async {
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
                None => Err(response::bad()),
            },
        }
    };
    response::unify(response().await)
}

pub(super) async fn progress(
    e: i32,
    _identity: String,
    app: App,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = || async {
        let progress = app.episode.get_progress(e).await?;
        Ok(warp::reply::json(&progress))
    };
    response::unify(response().await)
}
