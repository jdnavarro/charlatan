use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

use super::entity::Podcast;
use crate::podcast;

type Result<T> = std::result::Result<T, podcast::Error>;

pub(crate) async fn list(pool: SqlitePool) -> Result<Vec<Podcast>> {
    Ok(sqlx::query_as!(
        Podcast,
        r#"
SELECT id, src, title, image
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
SELECT id, src, title, image
FROM podcast
WHERE id = ?
        "#,
        id
    )
    .fetch_one(&pool)
    .await?)
}

pub(super) async fn add(pool: SqlitePool, src: &str, title: &str, image: &str) -> Result<i32> {
    sqlx::query!(
        r#"
INSERT OR IGNORE INTO podcast ( src, title, image)
VALUES ( $1, $2, $3)
        "#,
        src,
        title,
        image
    )
    .execute(&pool)
    .await?;

    let (id,): (i32,) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(&pool)
        .await?;

    Ok(id)
}
