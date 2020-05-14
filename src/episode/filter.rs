use sqlx::sqlite::SqlitePool;
use warp::Filter;

use super::handler;
use crate::error::Error;
use crate::with_pool;

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
        .and_then(|p| async {
            match handler::list(p).await {
                Ok(episodes) => Ok(warp::reply::json(&episodes)),
                Err(e) => Err(warp::reject::custom(Error::Database(e))),
            }
        })
}

fn crawl(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("crawl"))
        .and(warp::any().map(move || pool.clone()))
        .and_then(|p| async {
            match handler::crawl(p).await {
                Ok(episodes) => Ok(warp::reply::json(&episodes)),
                Err(e) => Err(warp::reject::custom(Error::Database(e))),
            }
        })
}
