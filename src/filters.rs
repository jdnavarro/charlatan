use std::convert::Infallible;

use warp::Filter;

use super::handlers;
use super::models::SqlitePool;
use crate::models::PooledSqliteConnection;

pub fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list_podcasts(pool.clone())
        .or(get_podcasts(pool.clone()))
        .or(add_podcast(pool.clone()))
        .or(list_episodes(pool.clone()))
        .or(fetch_episodes(pool))
}

fn list_podcasts(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("podcasts")
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(handlers::list_podcasts)
}

fn get_podcasts(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("podcasts" / i32)
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(handlers::get_podcasts)
}

fn add_podcast(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());
    warp::post()
        .and(warp::path!("podcasts"))
        .and(json_body)
        .and(with_pool(pool))
        .and_then(handlers::add_podcast)
}

fn list_episodes(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("episodes")
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(handlers::list_episodes)
}

fn fetch_episodes(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("fetch"))
        .and(with_pool(pool))
        .and_then(handlers::fetch_episodes)
}

fn with_pool(
    pool: SqlitePool,
) -> impl Filter<Extract = (PooledSqliteConnection,), Error = Infallible> + Clone {
    // TODO: Return 503 or something
    warp::any().map(move || pool.clone().get().expect("Unable to connect to the db"))
}
