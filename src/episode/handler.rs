use rss::Channel;
use sqlx::sqlite::SqlitePool;

use super::model::Episode;
use crate::podcast;

pub(super) async fn list(pool: SqlitePool) -> anyhow::Result<Vec<Episode>> {
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

pub async fn crawl(pool: SqlitePool) -> anyhow::Result<()> {
    let podcasts = podcast::handler::list(pool.clone()).await?;
    for podcast in podcasts {
        let channel = Channel::from_url(&podcast.url)?;
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
