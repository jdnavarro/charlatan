use std::convert::Infallible;

use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;
use warp::Filter;

pub use app::App;

pub mod app;
pub mod auth;
pub mod crawl;
pub mod episode;
pub mod podcast;
pub mod response;

#[cfg(not(feature = "web"))]
pub fn api(
    pool: SqlitePool,
    jwt_secret: String,
    app: App,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    podcast::api(pool.clone())
        .or(episode::api(app))
        .or(crawl::api(pool.clone()))
        .or(auth::api(pool, jwt_secret))
}

#[cfg(feature = "web")]
pub fn api(
    pool: SqlitePool,
    jwt_secret: String,
    app: App,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let web_dir = std::env::var("WEB_DIR").expect("WEB_DIR is not set");
    warp::path("api")
        .and(
            podcast::api(pool.clone())
                .or(episode::api(app))
                .or(auth::api(pool, jwt_secret)),
        )
        .or(warp::fs::dir(web_dir))
}

fn with_pool(pool: SqlitePool) -> impl Filter<Extract = (SqlitePool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn with_jwt_secret(
    jwt_token: String,
) -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::any().map(move || jwt_token.clone())
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
