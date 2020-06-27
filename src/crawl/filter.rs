use warp::Filter;

use super::handler;
use crate::app::{with_app, App};

pub fn api(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    with_app(app)
        .and(warp::path!("crawl"))
        .and(warp::post())
        .and_then(handler::crawl)
}
