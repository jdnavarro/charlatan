use sqlx::sqlite::SqlitePool;
use warp::Filter;

use super::handler;
use crate::{json_body, with_pool};

pub fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("register"))
        .and(warp::post())
        .and(json_body())
        .and_then(handler::register)
}
