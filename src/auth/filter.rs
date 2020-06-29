use sqlx::sqlite::SqlitePool;
use warp::Filter;

use super::handler;
use crate::{app::with_app, json_body, with_jwt_secret, with_pool, App};

pub fn api(
    pool: SqlitePool,
    jwt_secret: String,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    register(pool.clone()).or(login(pool, jwt_secret))
}

fn register(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(warp::path!("register"))
        .and(warp::post())
        .and(json_body())
        .and_then(handler::register)
}

fn login(
    pool: SqlitePool,
    jwt_secret: String,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_pool(pool)
        .and(with_jwt_secret(jwt_secret))
        .and(warp::path!("login"))
        .and(warp::post())
        .and(json_body())
        .and_then(handler::login)
}

pub(crate) fn with_identity(
    app: App,
) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header("Authorization"))
        .and(with_app(app))
        .and_then(|token: String, app: App| async move {
            app.identify(&token).map_err(warp::reject::custom)
        })
}
