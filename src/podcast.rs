use warp::Filter;

use crate::{with_pool, SqlitePool};

mod handler;
pub(crate) mod model;

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
        .and_then(handler::list)
}

fn get(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("podcasts" / i32)
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(handler::get)
}

fn add(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());
    warp::post()
        .and(warp::path!("podcasts"))
        .and(json_body)
        .and(with_pool(pool))
        .and_then(handler::add)
}
