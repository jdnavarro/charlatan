use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

use super::entity::Podcast;
use crate::podcast;

type Result<T> = std::result::Result<T, podcast::Error>;

pub(crate) async fn list(pool: SqlitePool) -> Result<Vec<Podcast>> {
    Ok(sqlx::query_as!(
        Podcast,
        r#"
SELECT id, src, title
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
SELECT id, src, title
FROM podcast
WHERE id = ?
        "#,
        id
    )
    .fetch_one(&pool)
    .await?)
}

pub(super) async fn add(pool: SqlitePool, src: &str, title: &str) -> Result<i32> {
    sqlx::query!(
        r#"
INSERT INTO podcast ( src, title )
VALUES ( $1, $2 )
        "#,
        src,
        title
    )
    .execute(&pool)
    .await?;

    let (id,): (i32,) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(&pool)
        .await?;

    Ok(id)
}

pub(super) async fn crawl(pool: SqlitePool, id: i32, items: &[rss::Item]) -> Result<()> {
    for item in items {
        sqlx::query!(
            r#"
INSERT INTO episode ( title, src, progress, podcast )
VALUES ( $1, $2, 0, $3 )
            "#,
            &item.title(),
            &item.enclosure().unwrap().url(),
            id,
        )
        .execute(&pool)
        .await?;
    }
    Ok(())
}
