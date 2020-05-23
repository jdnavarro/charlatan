use sqlx::sqlite::SqlitePool;
use warp::Filter;

use super::db;
use super::handler;
use crate::{json_body, with_handler, with_pool};

pub fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(pool.clone())
        .or(crawl(pool.clone()))
        .or(get_progress(pool.clone()))
        .or(set_progress(pool.clone()))
        .or(position(pool))
}

fn list(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("episodes")
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(|p| with_handler(db::list(p)))
}

fn get_progress(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("episodes" / i32 / "progress")
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(|e, p| with_handler(db::get_progress(p, e)))
}

fn set_progress(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("episodes" / i32 / "progress")
        .and(warp::put())
        .and(json_body())
        .and(with_pool(pool))
        .and_then(|e, prog, p| with_handler(db::set_progress(p, e, prog)))
}

fn position(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("episode" / i32))
        .and(warp::patch())
        .and(json_body())
        .and_then(handler::position)
}

fn crawl(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("crawl")
        .and(warp::post())
        .and(with_pool(pool))
        .and_then(|p| with_handler(db::crawl(p)))
}
