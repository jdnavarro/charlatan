use rss::Channel;
use sqlx::sqlite::SqlitePool;

use super::model::Episode;
use crate::podcast;

pub(super) async fn list(pool: SqlitePool) -> Result<Vec<Episode>, sqlx::Error> {
    Ok(sqlx::query_as!(
        Episode,
        r#"
SELECT id, title, url, podcast_id
FROM episode
ORDER BY id
        "#
    )
    .fetch_all(&pool)
    .await?)
}

pub(super) async fn crawl(pool: SqlitePool) -> Result<(), sqlx::Error> {
    let podcasts = podcast::handler::list(pool.clone()).await?;
    for podcast in podcasts {
        // TODO: Bubble up error
        let channel = Channel::from_url(&podcast.url).unwrap();
        for episode in channel.items() {
            sqlx::query!(
                r#"
            INSERT INTO episode ( title, url, podcast_id )
            VALUES ( $1, $2, $3 )
                    "#,
                &episode.title(),
                &episode.enclosure().unwrap().url(),
                &podcast.id,
            )
            .execute(&pool)
            .await?;
        }
    }
    Ok(())
}
