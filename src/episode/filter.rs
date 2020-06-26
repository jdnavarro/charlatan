use sqlx::sqlite::SqlitePool;
use warp::Filter;

use super::handler;
use crate::{json_body, with_jwt_secret, with_pool};

pub fn api(
    pool: SqlitePool,
    jwt_secret: String,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(pool.clone(), jwt_secret)
        .or(get_progress(pool.clone()))
        .or(episode(pool))
}

fn list(
    pool: SqlitePool,
    jwt_secret: String,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(with_jwt_secret(jwt_secret))
        .and(warp::header("Authorization"))
        .and(warp::path!("episodes"))
        .and(warp::path::end())
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

fn episode(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("episodes" / i32))
        .and(warp::patch())
        .and(json_body())
        .and_then(handler::episode)
}
