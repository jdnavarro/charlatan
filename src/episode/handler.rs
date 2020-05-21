use std::collections::HashMap;
use std::convert::Infallible;

use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;

use super::db;

pub(super) async fn position(
    p: SqlitePool,
    id: i32,
    hm: HashMap<String, i32>,
) -> Result<impl warp::Reply, Infallible> {
    match hm.get("position") {
        Some(position) => match db::position(p, id, *position).await {
            Ok(r) => Ok(warp::reply::with_status(
                warp::reply::json(&r),
                StatusCode::OK,
            )),
            Err(e) => Ok(warp::reply::with_status(
                warp::reply::json(&format!("{}", e)),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        },
        None => Ok(warp::reply::with_status(
            warp::reply::json(&"Expecting position"),
            StatusCode::BAD_REQUEST,
        )),
    }
}
