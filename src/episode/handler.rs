use std::collections::HashMap;

use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;

use crate::queue;

pub(super) async fn position(
    p: SqlitePool,
    id: i32,
    hm: HashMap<String, i32>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: Custom error for expected progress key
    let position = hm.get("position").ok_or(warp::reject::not_found())?;
    match queue::db::position(&p, id, *position).await {
        Ok(r) => Ok(warp::reply::with_status(
            warp::reply::json(&r),
            StatusCode::OK,
        )),
        Err(e) => Ok(warp::reply::with_status(
            warp::reply::json(&format!("{}", e)),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
