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

pub(super) async fn crawl(pool: SqlitePool) -> Result<(), sqlx::Error> {
    let podcasts = podcast::handler::list(pool.clone()).await?;
    for podcast in podcasts {
        // TODO: Bubble up error
        let channel = Channel::from_url(&podcast.uri.to_string()).unwrap();
        for episode in channel.items() {
            sqlx::query!(
                r#"
            INSERT INTO episode ( id, title, uri, podcast )
            VALUES ( $1, $2, $3, $4 )
                "#,
                &episode.guid().unwrap().value(),
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
