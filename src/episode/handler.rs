use rss::Channel;
use sqlx::sqlite::SqlitePool;

use super::model::Episode;
use crate::podcast;

pub(super) async fn list(pool: SqlitePool) -> Result<Vec<Episode>, sqlx::Error> {
    sqlx::query_as!(
        Episode,
        r#"
SELECT id, title, uri, podcast
FROM episode
        "#
    )
    .fetch_all(&pool)
    .await
}

pub(super) async fn get_progress(pool: SqlitePool, episode: i32) -> Result<i32, sqlx::Error> {
    let episode = sqlx::query!(
        r#"
SELECT progress
FROM episode
WHERE id = ?
        "#,
        episode,
    )
    .fetch_one(&pool)
    .await?;
    Ok(episode.progress)
}

pub(super) async fn set_progress(
    pool: SqlitePool,
    episode: i32,
    progress: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
UPDATE episode
SET progress = $1
WHERE id = $2
        "#,
        progress,
        episode,
    )
    .execute(&pool)
    .await?;
    Ok(())
}

pub(super) async fn crawl(pool: SqlitePool) -> Result<(), sqlx::Error> {
    let podcasts = podcast::handler::list(pool.clone()).await?;
    for podcast in podcasts {
        // TODO: Bubble up error
        let channel = Channel::from_url(&podcast.uri.to_string()).unwrap();
        for episode in channel.items() {
            sqlx::query!(
                r#"
INSERT INTO episode ( title, uri, podcast )
VALUES ( $1, $2, $3 )
                "#,
                // &episode.guid().unwrap().value(),
                &episode.title(),
                &episode.enclosure().unwrap().url(),
                &podcast.uri,
            )
            .execute(&pool)
            .await?;
        }
    }
    Ok(())
}
