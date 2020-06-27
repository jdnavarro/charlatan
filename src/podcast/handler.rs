use std::collections::HashMap;

use super::{entity::NewPodcast, Podcast};
use crate::app::App;
use crate::response;

pub(crate) async fn list(token: String, app: App) -> Result<impl warp::Reply, warp::Rejection> {
    let response = || async {
        let _ = app.identify(&token)?;
        let podcasts = app.podcast.list().await.map(|v| {
            v.into_iter()
                .map(|p| (p.id, p))
                .collect::<HashMap<i32, Podcast>>()
        })?;

        Ok(warp::reply::json(&podcasts))
    };
    response::unify(response().await)
}

pub(super) async fn get(
    token: String,
    id: i32,
    app: App,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = || async {
        let _ = app.identify(&token)?;

        let podcasts = app.podcast.get(id).await?;
        Ok(warp::reply::json(&podcasts))
    };
    response::unify(response().await)
}

pub(super) async fn delete(
    token: String,
    id: i32,
    app: App,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = || async {
        let _ = app.identify(&token)?;

        let podcasts = app.podcast.delete(id).await?;
        Ok(warp::reply::json(&podcasts))
    };
    response::unify(response().await)
}

pub(super) async fn add(
    token: String,
    m: HashMap<String, String>,
    app: App,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = || async {
        let _ = app.identify(&token)?;
        // TODO: Sanitize or parse
        let src = m.get("url").ok_or(response::bad())?;
        let channel = rss::Channel::from_url(&src).map_err(|_| response::bad())?;
        let new_podcast = parse(&src, &channel);
        let podcasts = app.podcast.add(&new_podcast).await?;
        Ok(warp::reply::json(&podcasts))
    };
    response::unify(response().await)
}

fn parse<'a>(src: &'a str, channel: &'a rss::Channel) -> NewPodcast<'a> {
    let url = &channel.link();
    let title = &channel.title();
    let image = &channel.image().map_or_else(
        || {
            log::warn!("Missing image for podcast {}", &src);
            ""
        },
        |i| i.url(),
    );
    let description = &channel.description();

    NewPodcast {
        src,
        url,
        title,
        image,
        description,
    }
}
