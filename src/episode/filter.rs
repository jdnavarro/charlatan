use sqlx::sqlite::SqlitePool;
use warp::Filter;

use super::handler;
use crate::{json_body, with_pool};

pub fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(pool.clone())
        // .or(crawl(pool.clone()))
        .or(get_progress(pool.clone()))
        .or(set_progress(pool.clone()))
        .or(position(pool))
}

fn list(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("episodes"))
        .and(warp::get())
        .and_then(handler::list)
}

fn get_progress(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("episodes" / i32 / "progress"))
        .and(warp::get())
        .and_then(handler::get_progress)
}

fn set_progress(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("episodes" / i32 / "progress"))
        .and(warp::put())
        .and(json_body())
        .and_then(handler::set_progress)
}

fn position(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("episodes" / i32))
        .and(warp::patch())
        .and(json_body())
        .and_then(handler::position)
}
