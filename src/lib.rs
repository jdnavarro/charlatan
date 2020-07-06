use serde::de::DeserializeOwned;
use warp::Filter;

pub use app::App;

pub mod app;
pub mod auth;
pub mod crawl;
mod db;
pub mod episode;
pub mod podcast;
pub mod response;

#[cfg(not(feature = "web"))]
pub fn api(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    podcast::api(app.clone())
        .or(episode::api(app.clone()))
        .or(crawl::api(app.clone()))
        .or(auth::api(app))
}

#[cfg(feature = "web")]
pub fn api(app: App) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let web_dir = std::env::var("WEB_DIR").expect("WEB_DIR is not set");
    warp::path("api")
        .and(
            podcast::api(app.clone())
                .or(episode::api(app.clone()))
                .or(auth::api(app)),
        )
        .or(warp::fs::dir(web_dir))
}

pub(crate) fn json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Copy
where
    T: DeserializeOwned + Send,
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
