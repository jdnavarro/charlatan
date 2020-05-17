use sqlx::sqlite::SqlitePool;
use warp::Filter;

use super::handler;
use crate::{with_handler, with_pool};

pub fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(pool.clone())
        .or(crawl(pool.clone()))
        .or(get_progress(pool.clone()))
        .or(set_progress(pool))
}

fn list(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("episodes")
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(|p| with_handler(handler::list(p)))
}

fn get_progress(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("episodes" / i32 / "progress")
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(|e, p| with_handler(handler::get_progress(p, e)))
}

fn set_progress(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::path!("episodes" / i32 / "progress")
        .and(warp::put())
        .and(json_body)
        .and(with_pool(pool))
        .and_then(|e, prog, p| with_handler(handler::set_progress(p, e, prog)))
}

fn crawl(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("crawl")
        .and(warp::post())
        .and(with_pool(pool))
        .and_then(|p| with_handler(handler::crawl(p)))
}
