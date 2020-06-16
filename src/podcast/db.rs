use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

use super::entity::{NewPodcast, Podcast};
use crate::podcast;

type Result<T> = std::result::Result<T, podcast::Error>;

pub(crate) async fn list(pool: SqlitePool) -> Result<Vec<Podcast>> {
    Ok(sqlx::query_as!(
        Podcast,
        r#"
SELECT id, src, url, title, image, description
FROM podcast
        "#
    )
    .fetch_all(&pool)
    .await?)
}

pub(super) async fn get(pool: SqlitePool, id: i32) -> Result<Podcast> {
    Ok(sqlx::query_as!(
        Podcast,
        r#"
SELECT id, src, url, title, image, description
FROM podcast
WHERE id = ?
        "#,
        id
    )
    .fetch_one(&pool)
    .await?)
}

pub(super) async fn add(pool: SqlitePool, podcast: &NewPodcast<'_>) -> Result<i32> {
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
    .execute(&pool)
    .await?;

    let (id,): (i32,) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(&pool)
        .await?;

    Ok(id)
}

pub(super) async fn delete(pool: SqlitePool, id: i32) -> Result<()> {
    sqlx::query!(
        r#"
DELETE FROM podcast where id = ?;
        "#,
        id
    )
    .execute(&pool)
    .await?;

    Ok(())
}
