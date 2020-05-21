use rss::Channel;
use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

use super::entity::Episode;
use crate::episode;
use crate::podcast;

pub(super) async fn list(pool: SqlitePool) -> Result<Vec<Episode>, sqlx::Error> {
    sqlx::query_as!(
        Episode,
        r#"
SELECT id, title, src, progress, position, podcast
FROM episode
ORDER BY id ASC
LIMIT 50
        "#,
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
        let channel = Channel::from_url(&podcast.src.to_string()).unwrap();
        for episode in channel.items() {
            sqlx::query!(
                r#"
INSERT INTO episode ( title, src, progress, podcast )
VALUES ( $1, $2, 0, $3 )
                "#,
                &episode.title(),
                &episode.enclosure().unwrap().url(),
                &podcast.src,
            )
            .execute(&pool)
            .await?;
        }
    }
    Ok(())
}

pub(super) async fn position(
    pool: SqlitePool,
    id: i32,
    new_position: i32,
) -> Result<(), episode::Error> {
    // pool.acquire()
    //     .await?
    //     .execute("SAVEPOINT update_queue")
    //     .await?;

    let (old_position,): (i32,) = sqlx::query_as(
        r#"
SELECT position
FROM episode
WHERE id = ?;
        "#,
    )
    .bind(id)
    .fetch_one(&pool)
    .await?;

    // TODO: Handle when not enqueued
    if new_position > old_position {
        sqlx::query!(
            r#"
UPDATE episode
SET position = position - 1
WHERE position >= $1
AND position <= $2
        "#,
            old_position,
            new_position,
        )
        .execute(&pool)
        .await?;
    } else {
        sqlx::query!(
            r#"
UPDATE episode
SET position = position + 1
WHERE position >= $1
AND position <= $2
        "#,
            new_position,
            old_position,
        )
        .execute(&pool)
        .await?;
    }

    sqlx::query!(
        r#"
UPDATE episode
SET position = $1
WHERE id = $2;
        "#,
        new_position,
        id,
    )
    .execute(&pool)
    .await?;

    // pool.acquire()
    //     .await?
    //     .execute("RELEASE update_queue")
    //     .await?;

    Ok(())
}
