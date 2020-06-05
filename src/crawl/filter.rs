use sqlx::sqlite::SqlitePool;
use warp::Filter;

use super::handler;
use crate::with_pool;

pub fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("crawl"))
        .and(warp::post())
        .and_then(handler::crawl)
}
