use warp::Filter;

use super::handler;
use crate::{app::with_app, json_body, App};

pub fn api(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    register(app.clone()).or(login(app))
}

fn register(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("register"))
        .and(json_body())
        .and(with_app(app))
        .and_then(handler::register)
}

fn login(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(json_body())
        .and(with_app(app))
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
