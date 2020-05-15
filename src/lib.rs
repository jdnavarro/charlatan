use std::convert::Infallible;

use serde::Serialize;
use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;
use warp::{Filter, Future};

pub mod episode;
pub mod podcast;

pub(crate) fn with_pool(
    pool: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub(crate) async fn with_handler<T, F>(f: F) -> Result<impl warp::Reply, Infallible>
where
    F: Future<Output = Result<T, sqlx::Error>>,
    T: Serialize,
{
    match f.await {
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
