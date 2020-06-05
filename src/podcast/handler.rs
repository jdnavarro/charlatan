use sqlx::sqlite::SqlitePool;

use super::db;
use crate::episode;
use crate::episode::NewEpisode;
use crate::json_reply;

pub(super) async fn list(p: SqlitePool) -> Result<impl warp::Reply, warp::Rejection> {
    json_reply(db::list(p).await)
}

pub(super) async fn get(p: SqlitePool, id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let podcast = db::get(p, id)
        .await
        .map_err(|_| warp::reject::not_found())?;
    Ok(podcast)
}

pub(super) async fn add(p: SqlitePool, src: String) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: Report and skip errors
    let channel = rss::Channel::from_url(&src).unwrap();
    // TODO: Insert episodes here

    json_reply(db::add(p, &src, channel.title()).await)
}

pub(super) async fn crawl(p: SqlitePool, id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let podcast = db::get(p.clone(), id)
        .await
        .map_err(|_| warp::reject::not_found())?;
    // TODO: Async, reuse connection, handle error
    let channel = rss::Channel::from_url(&podcast.src).unwrap();
    json_reply(db::crawl(p, id, &channel.items()).await)
}

pub(super) async fn crawl1(p: SqlitePool, id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let podcast = db::get(p.clone(), id)
        .await
        .map_err(|_| warp::reject::not_found())?;

    let channel = rss::Channel::from_url(&podcast.src).unwrap();

    for item in channel.items() {
        let new_episode = NewEpisode {
            title: &item.title().unwrap(),
            guid: &item.guid().unwrap().value(),
            duration: item
                .itunes_ext()
                .unwrap()
                .duration()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            image: &item.itunes_ext().unwrap().image().unwrap(),
            publication: item.pub_date().unwrap().parse::<i32>().unwrap(),
            src: &item.enclosure().unwrap().url(),
            podcast: id,
        };
    }
    Ok(warp::http::StatusCode::CREATED)
}
