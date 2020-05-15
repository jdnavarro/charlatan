use sqlx::sqlite::SqlitePool;
use warp::Filter;

use super::handler;
use crate::{with_handler, with_pool};

pub fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(pool.clone()).or(get(pool.clone())).or(add(pool))
}

fn list(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("podcasts")
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(|p| with_handler(handler::list(p)))
}

fn get(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("podcasts" / String)
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(|uri, p| with_handler(handler::get(p, uri)))
}

fn add(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::path!("podcasts")
        .and(warp::post())
        .and(with_pool(pool))
        .and(json_body)
        .and_then(|p, uri| with_handler(handler::add(p, uri)))
}
