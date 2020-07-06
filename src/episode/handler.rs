use std::{collections::HashMap, convert::Infallible};

use super::entity::Episode;
use crate::{app::App, db, response};

pub(super) async fn list(_identity: String, app: App) -> Result<impl warp::Reply, Infallible> {
    db::respond(app.episode.list().await.map(|v| {
        v.into_iter()
            .map(|p| (p.id, p))
            .collect::<HashMap<i32, Episode>>()
    }))
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
    id: i32,
    _identity: String,
    app: App,
) -> Result<impl warp::Reply, Infallible> {
    db::respond(app.episode.get_progress(id).await)
}
