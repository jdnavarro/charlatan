use sqlx::sqlite::SqlitePool;
use warp::Filter;

use super::handler;
use crate::app::{with_app, App};
use crate::{json_body, with_pool};

pub fn api(
    pool: SqlitePool,
    app: App,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(app).or(get_progress(pool.clone())).or(episode(pool))
}

fn list(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("episodes"))
        .and(warp::path::end())
        .and(warp::header("Authorization"))
        .and(with_app(app))
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
