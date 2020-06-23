use std::convert::Infallible;

use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;
use warp::Filter;

pub mod auth;
pub mod crawl;
pub mod episode;
pub mod podcast;

#[cfg(not(feature = "web"))]
pub fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    podcast::api(pool.clone())
        .or(episode::api(pool.clone()))
        .or(crawl::api(pool.clone()))
        .or(auth::api(pool))
}

#[cfg(feature = "web")]
pub fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let web_dir = std::env::var("WEB_DIR").expect("WEB_DIR is not set");
    warp::path("api")
        .and(podcast::api(pool.clone()).or(episode::api(pool)))
        .or(warp::fs::dir(web_dir))
}

fn with_pool(pool: SqlitePool) -> impl Filter<Extract = (SqlitePool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn json_reply(
    r: Result<impl Serialize, impl std::error::Error>,
) -> Result<warp::reply::WithStatus<warp::reply::Json>, warp::Rejection> {
    match r {
        Ok(o) => Ok(warp::reply::with_status(
            warp::reply::json(&o),
            StatusCode::OK,
        )),
        Err(e) => Ok(warp::reply::with_status(
            warp::reply::json(&e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub(crate) fn json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Copy
where
    T: DeserializeOwned + Send,
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
