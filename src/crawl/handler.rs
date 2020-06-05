use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;

use crate::crawl;
use crate::episode::{self, NewEpisode};
use crate::podcast::{self, Podcast};

pub(super) async fn crawl(p: SqlitePool) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    match podcast::db::list(p.clone()).await {
        Ok(podcasts) => {
            // TODO: Stream directly with sqlx cursor?
            for podcast in podcasts {
                match crawl_podcast(p.clone(), &podcast).await {
                    Ok(()) => log::info!("Podcast crawled: {}", &podcast.id),
                    Err(e) => {
                        log::warn!("Skipping podcast {} because err -- {:#?}", &podcast.id, &e)
                    }
                };
            }
            Ok(StatusCode::CREATED)
        }

        Err(e) => {
            log::error!("Error while listing podcasts -- {:#?}", &e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

type Result<T> = std::result::Result<T, crawl::Error>;

async fn crawl_podcast(p: SqlitePool, podcast: &Podcast) -> Result<()> {
    let channel = rss::Channel::from_url(&podcast.src)?;

    for item in channel.items() {
        let new_episode = parse(podcast.id, &item)?;
        episode::db::add(p.clone(), &new_episode).await?;
    }
    Ok(())
}

fn parse(id: i32, item: &rss::Item) -> Result<NewEpisode> {
    let src = &item
        .enclosure()
        .ok_or(crawl::Error::MissingSource(id))?
        .url();

    let title = &item.title().unwrap_or_else(|| {
        log::warn!(
            "Missing title for episode id: {}. Using source URL: {}",
            id,
            &src
        );
        src.clone()
    });

    let guid = &item.guid().map_or_else(
        || {
            log::warn!(
                "Missing guid for episode id: {}. Using source URL: {}",
                id,
                &src
            );
            src.clone()
        },
        |i| i.value(),
    );

    // TODO: Parse duration format
    let duration = item
        .itunes_ext()
        .and_then(|it| it.duration())
        .unwrap_or_else(|| {
            log::warn!("Missing duration for episode id: {}", id);
            ""
        });

    // TODO: Use podcast image
    let image = item
        .itunes_ext()
        .and_then(|it| it.image())
        .unwrap_or_else(|| {
            log::info!("Missing image for episode id: {}. Using podcast image", id);
            ""
        });

    // TODO: Parse date;
    let publication = item.pub_date().unwrap_or_else(|| {
        log::warn!("Missing duration for episode id: {}", id);
        ""
    });

    Ok(NewEpisode {
        title,
        guid,
        duration,
        image,
        publication,
        src,
        podcast: id,
    })
}
