use sqlx::sqlite::SqliteQueryAs;

use super::entity::{NewPodcast, Podcast};
use crate::podcast;

type Result<T> = std::result::Result<T, podcast::Error>;

#[derive(Debug, Clone)]
pub struct Store {
    pub pool: sqlx::SqlitePool,
}

impl Store {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

impl Store {
    pub(crate) async fn list(&self) -> Result<Vec<Podcast>> {
        Ok(sqlx::query_as!(
            Podcast,
            r#"
SELECT id, src, url, title, image, description
FROM podcast
        "#
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub(super) async fn get(&self, id: i32) -> Result<Podcast> {
        Ok(sqlx::query_as!(
            Podcast,
            r#"
SELECT id, src, url, title, image, description
FROM podcast
WHERE id = ?
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub(crate) async fn add(&self, podcast: &NewPodcast<'_>) -> Result<i32> {
        sqlx::query!(
            r#"
INSERT OR IGNORE INTO podcast (src, url, title, image, description)
VALUES ($1, $2, $3, $4, $5)
        "#,
            podcast.src,
            podcast.url,
            podcast.title,
            podcast.image,
            podcast.description
        )
        .execute(&self.pool)
        .await?;

        let (id,): (i32,) = sqlx::query_as("SELECT last_insert_rowid()")
            .fetch_one(&self.pool)
            .await?;

        Ok(id)
    }

    pub(super) async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!(
            r#"
DELETE FROM podcast
WHERE id = ?;
        "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
