use rss::Channel;
use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::SqliteQueryAs;

use super::model::Podcast;

pub(crate) async fn list(pool: SqlitePool) -> anyhow::Result<Vec<Podcast>> {
    Ok(sqlx::query_as!(
        Podcast,
        r#"
SELECT id, title, url
FROM podcast
ORDER BY id
        "#
    )
    .fetch_all(&pool)
    .await?)
}

pub(super) async fn get(pool: SqlitePool, id: i32) -> anyhow::Result<Podcast> {
    Ok(sqlx::query_as!(
        Podcast,
        r#"
SELECT id, title, url
FROM podcast
WHERE id = ?
        "#,
        id
    )
    .fetch_one(&pool)
    .await?)
}

pub(super) async fn add(pool: SqlitePool, url: &str) -> anyhow::Result<i32> {
    let channel = Channel::from_url(url)?;

    // TODO: Insert episodes here

    sqlx::query!(
        r#"
INSERT INTO podcast ( title, url )
VALUES ( $1, $2 )
        "#,
        &channel.title(),
        url
    )
    .execute(&pool)
    .await?;

    let rec: (i32,) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(&pool)
        .await?;

    Ok(rec.0)
}
