use std::convert::Infallible;

use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;

use super::db;

pub(super) async fn list(p: SqlitePool) -> Result<impl warp::Reply, Infallible> {
    match db::list(p).await {
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

pub(super) async fn add(p: SqlitePool, id: i32) -> Result<impl warp::Reply, Infallible> {
    match db::add(p, id).await {
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

pub(super) async fn delete(p: SqlitePool, id: i32) -> Result<impl warp::Reply, Infallible> {
    match db::delete(p, id).await {
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

pub(super) async fn position(
    p: SqlitePool,
    id: i32,
    position: i32,
) -> Result<impl warp::Reply, Infallible> {
    match db::position(p, id, position).await {
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
