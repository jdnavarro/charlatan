use std::collections::HashMap;

use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;

use crate::queue;

pub(super) async fn position(
    p: SqlitePool,
    id: i32,
    hm: HashMap<String, Option<i32>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let position = hm.get("position").ok_or(warp::reject::not_found())?;
    match position {
        None => match queue::db::delete(&p, id).await {
            Ok(r) => Ok(warp::reply::with_status(
                warp::reply::json(&r),
                StatusCode::OK,
            )),
            Err(e) => Ok(warp::reply::with_status(
                warp::reply::json(&e.to_string()),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        },
        Some(pos) => match queue::db::position(&p, id, *pos).await {
            Ok(r) => Ok(warp::reply::with_status(
                warp::reply::json(&r),
                StatusCode::OK,
            )),
            Err(e) => Ok(warp::reply::with_status(
                warp::reply::json(&e.to_string()),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        },
    }
}
