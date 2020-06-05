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
        let new_episode = parse(&podcast, &item)?;
        episode::db::add(p.clone(), &new_episode).await?;
    }
    Ok(())
}

fn parse<'a>(podcast: &'a Podcast, item: &'a rss::Item) -> Result<NewEpisode<'a>> {
    let src = &item
        .enclosure()
        .ok_or(crawl::Error::MissingSource(podcast.id))?
        .url();

    let guid = &item.guid().map_or_else(
        || {
            log::warn!(
                "Missing guid in episode for podcast id: {}, using source instead: {}",
                podcast.id,
                &src
            );
            src.clone()
        },
        |i| i.value(),
    );

    let title = &item.title().unwrap_or_else(|| {
        log::warn!(
            "Missing title in episode guid: {}. Using source URL: {}",
            &guid,
            &src
        );
        src.clone()
    });

    // TODO: Parse duration format
    let duration = item
        .itunes_ext()
        .and_then(|it| it.duration())
        .unwrap_or_else(|| {
            log::warn!("Missing duration for episode guid: {}", &guid);
            ""
        });

    let image = item
        .itunes_ext()
        .and_then(|it| it.image())
        .unwrap_or_else(|| {
            log::info!(
                "Missing image for episode guid: {}. Using podcast image",
                &guid
            );
            &podcast.image
        });

    // TODO: Parse date;
    let publication = item.pub_date().unwrap_or_else(|| {
        log::warn!("Missing duration for episode guid: {}", &guid);
        ""
    });

    Ok(NewEpisode {
        title,
        guid,
        duration,
        image,
        publication,
        src,
        podcast: podcast.id,
    })
}
