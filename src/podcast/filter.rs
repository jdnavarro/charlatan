use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;
use warp::Filter;

use super::handler;
use crate::error::Error;
use crate::with_pool;

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
        .and_then(|p| async {
            match handler::list(p).await {
                Ok(podcasts) => Ok(warp::reply::json(&podcasts)),
                Err(e) => Err(warp::reject::custom(Error::Database(e))),
            }
        })
}

fn get(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("podcasts" / String)
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(|uri, p| async {
            match handler::get(p, uri).await {
                Ok(episode) => Ok(warp::reply::json(&episode)),
                Err(e) => Err(warp::reject::custom(Error::Database(e))),
            }
        })
}

fn add(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::path!("podcasts")
        .and(warp::post())
        .and(with_pool(pool))
        .and(json_body)
        .and_then(|p, uri| async {
            match handler::add(p, uri).await {
                Ok(_) => Ok(StatusCode::CREATED),
                Err(e) => Err(warp::reject::custom(Error::Database(e))),
            }
        })
}
