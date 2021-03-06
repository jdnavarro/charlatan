use warp::Filter;

use super::handler;
use crate::{
    app::{with_app, App},
    auth::filter::with_identity,
    json_body,
};

pub fn api(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(app.clone())
        .or(get(app.clone()))
        .or(add(app.clone()))
        .or(delete(app))
}

fn list(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("podcasts"))
        .and(warp::path::end())
        .and(with_identity(app.clone()))
        .and(with_app(app))
        .and_then(handler::list)
}

fn get(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("podcasts" / i32))
        .and(warp::path::end())
        .and(with_identity(app.clone()))
        .and(with_app(app))
        .and_then(handler::get)
}

fn add(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("podcasts"))
        .and(warp::path::end())
        .and(with_identity(app.clone()))
        .and(json_body())
        .and(with_app(app))
        .and_then(handler::add)
}

fn delete(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::delete()
        .and(warp::path!("podcasts" / i32))
        .and(warp::path::end())
        .and(with_identity(app.clone()))
        .and(with_app(app))
        .and_then(handler::delete)
}
