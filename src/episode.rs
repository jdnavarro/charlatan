use warp::Filter;

use crate::{with_pool, SqlitePool};

mod handler;
mod model;

pub fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(pool.clone()).or(crawl(pool))
}

fn list(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("episodes")
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(handler::list)
}

fn crawl(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("fetch"))
        .and(with_pool(pool))
        .and_then(handler::crawl)
}
