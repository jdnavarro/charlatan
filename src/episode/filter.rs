use warp::Filter;

use super::handler;
use crate::{
    app::{with_app, App},
    auth::filter::with_identity,
    json_body,
};

pub fn api(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(app.clone()).or(episode(app.clone())).or(progress(app))
}

fn list(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("episodes"))
        .and(warp::path::end())
        .and(with_identity(app.clone()))
        .and(with_app(app))
        .and_then(handler::list)
}

fn episode(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::patch()
        .and(warp::path!("episodes" / i32))
        .and(warp::path::end())
        .and(with_identity(app.clone()))
        .and(json_body())
        .and(with_app(app))
        .and_then(handler::episode)
}

fn progress(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("episodes" / i32 / "progress"))
        .and(warp::path::end())
        .and(with_identity(app.clone()))
        .and(with_app(app))
        .and_then(handler::progress)
}
