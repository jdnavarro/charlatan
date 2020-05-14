use rss::Channel;
use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::SqliteQueryAs;

use super::model::Podcast;

pub(crate) async fn list(pool: SqlitePool) -> Result<Vec<Podcast>, sqlx::Error> {
    Ok(sqlx::query_as!(
        Podcast,
        r#"
SELECT uri, title
FROM podcast
        "#
    )
    .fetch_all(&pool)
    .await?)
}

pub(super) async fn get(pool: SqlitePool, uri: String) -> Result<Podcast, sqlx::Error> {
    Ok(sqlx::query_as!(
        Podcast,
        r#"
SELECT uri, title
FROM podcast
WHERE uri = ?
        "#,
        uri
    )
    .fetch_one(&pool)
    .await?)
}

pub(super) async fn add(pool: SqlitePool, uri: String) -> Result<String, sqlx::Error> {
    // TODO: Report and skip errors.
    let channel = Channel::from_url(&uri).unwrap();

    // TODO: Insert episodes here

    sqlx::query!(
        r#"
INSERT INTO podcast ( uri, title )
VALUES ( $1, $2 )
        "#,
        uri,
        &channel.title()
    )
    .execute(&pool)
    .await?;

    let rec: (String,) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(&pool)
        .await?;

    Ok(rec.0)
}
