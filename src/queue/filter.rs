use sqlx::sqlite::SqlitePool;
use warp::Filter;

use super::handler;
use crate::with_pool;

pub(crate) fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(pool.clone()).or(add(pool.clone())).or(delete(pool))
}

fn list(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("queue")
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(handler::list)
}

fn add(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("queue" / i32))
        .and(warp::put())
        .and_then(handler::add)
}

fn delete(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("queue" / i32))
        .and(warp::delete())
        .and_then(handler::delete)
}
