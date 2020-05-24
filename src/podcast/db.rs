use rss::Channel;
use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

use super::entity::Podcast;
use crate::podcast;

type Result<T> = std::result::Result<T, podcast::Error>;

pub(crate) async fn list(pool: SqlitePool) -> Result<Vec<Podcast>> {
    Ok(sqlx::query_as!(
        Podcast,
        r#"
SELECT src, title
FROM podcast
        "#
    )
    .fetch_all(&pool)
    .await?)
}

pub(super) async fn get(pool: SqlitePool, src: String) -> Result<Podcast> {
    Ok(sqlx::query_as!(
        Podcast,
        r#"
SELECT src, title
FROM podcast
WHERE src = ?
        "#,
        src
    )
    .fetch_one(&pool)
    .await?)
}

pub(super) async fn add(pool: SqlitePool, src: String) -> Result<i32> {
    // TODO: Report and skip errors
    let channel = Channel::from_url(&src).unwrap();

    // TODO: Insert episodes here

    sqlx::query!(
        r#"
INSERT INTO podcast ( src, title )
VALUES ( $1, $2 )
        "#,
        src,
        &channel.title()
    )
    .execute(&pool)
    .await?;

    let rec: (i32,) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(&pool)
        .await?;

    Ok(rec.0)
}
