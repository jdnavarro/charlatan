pub mod app;
pub mod auth;
pub mod crawl;
mod db;
pub mod episode;
pub mod podcast;
mod rejection;
pub mod response;

use serde::de::DeserializeOwned;
use warp::Filter;

pub use app::App;
use rejection::handle_rejection;

pub fn api(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    podcast::api(app.clone())
        .or(episode::api(app.clone()))
        .or(crawl::api(app.clone()))
        .or(auth::api(app.clone()))
        .recover(move |x| handle_rejection(app.clone(), x))
}

pub(crate) fn json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Copy
where
    T: DeserializeOwned + Send,
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
