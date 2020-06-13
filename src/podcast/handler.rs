use std::collections::HashMap;

use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;

use super::db;
use super::entity::NewPodcast;
use crate::json_reply;

pub(crate) async fn list(p: SqlitePool) -> Result<impl warp::Reply, warp::Rejection> {
    json_reply(db::list(p).await)
}

pub(super) async fn get(p: SqlitePool, id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let podcast = db::get(p, id)
        .await
        .map_err(|_| warp::reject::not_found())?;
    Ok(podcast)
}

pub(super) async fn add(
    p: SqlitePool,
    m: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let src = m.get("url").ok_or_else(warp::reject::not_found)?;

    match rss::Channel::from_url(&src) {
        Ok(channel) => {
            log::info!("Adding podcast {}", &src);
            let new_podcast = parse(&src, &channel);
            // TODO: Crawl podcast here
            json_reply(db::add(p, &new_podcast).await)
        }
        Err(e) => {
            let msg = ("There was a problem with podcast url {}", &src);
            log::warn!(
                "There was a problem with podcast url {} -- err {:#?}",
                &src,
                e
            );

            Ok(warp::reply::with_status(
                warp::reply::json(&msg),
                // TODO: Should handle 3rd party error
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
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
