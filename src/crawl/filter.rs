use warp::Filter;

use super::handler;
use crate::{
    app::{with_app, App},
    auth::filter::with_identity,
};

pub fn api(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("crawl"))
        .and(with_identity(app.clone()))
        .and(with_app(app))
        .and_then(handler::crawl)
}
