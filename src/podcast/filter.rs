use std::convert::Infallible;

use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;
use warp::Filter;

use super::handler;

pub fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(pool.clone()).or(get(pool.clone())).or(add(pool))
}

fn list(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    async fn handle(p: SqlitePool) -> Result<impl warp::Reply, Infallible> {
        let podcasts = handler::list(p).await.expect("Error loading podcasts");
        Ok(warp::reply::json(&podcasts))
    }
    warp::path!("podcasts")
        .and(warp::get())
        .and(warp::any().map(move || pool.clone()))
        .and_then(handle)
}

fn get(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    async fn handle(id: i32, p: SqlitePool) -> Result<impl warp::Reply, Infallible> {
        let podcast = handler::get(p, id).await.expect("Error loading podcast");
        Ok(warp::reply::json(&podcast))
    }
    warp::path!("podcasts" / i32)
        .and(warp::get())
        .and(warp::any().map(move || pool.clone()))
        .and_then(handle)
}

fn add(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    async fn handle(url: String, p: SqlitePool) -> Result<impl warp::Reply, Infallible> {
        handler::add(p, &url).await.expect("Error adding podcast");
        Ok(StatusCode::CREATED)
    }

    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());
    warp::path!("podcasts")
        .and(warp::post())
        .and(json_body)
        .and(warp::any().map(move || pool.clone()))
        .and_then(handle)
}
