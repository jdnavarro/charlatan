use std::convert::Infallible;

use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;
use warp::Filter;

use super::handler;

pub fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(pool.clone()).or(crawl(pool))
}

fn list(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    async fn handle(p: SqlitePool) -> Result<impl warp::Reply, Infallible> {
        let episodes = handler::list(p).await.expect("Error loading episodes");
        Ok(warp::reply::json(&episodes))
    }
    warp::path!("episodes")
        .and(warp::get())
        .and(warp::any().map(move || pool.clone()))
        .and_then(handle)
}

fn crawl(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    async fn handle(p: SqlitePool) -> Result<impl warp::Reply, Infallible> {
        handler::crawl(p).await.expect("Error loading episodes");
        Ok(StatusCode::CREATED)
    }
    warp::post()
        .and(warp::path!("crawl"))
        .and(warp::any().map(move || pool.clone()))
        .and_then(handle)
}
