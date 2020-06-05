use sqlx::sqlite::SqlitePool;
use warp::Filter;

use super::handler;
use crate::{json_body, with_pool};

pub fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(pool.clone()).or(get(pool.clone())).or(add(pool))
}

fn list(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("podcasts"))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(handler::list)
}

fn get(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("podcasts" / i32))
        .and(warp::get())
        .and_then(handler::get)
}

fn add(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("podcasts"))
        .and(warp::path::end())
        .and(warp::post())
        .and(json_body())
        .and_then(handler::add)
}
