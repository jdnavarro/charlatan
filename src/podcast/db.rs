use rss::Channel;
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

// pub(super) async fn crawl(pool: SqlitePool) -> Result<()> {
//         let channel = Channel::from_url(&podcast.src.to_string()).unwrap();
//         for episode in channel.items() {
//             sqlx::query!(
//                 r#"
// INSERT INTO episode ( title, src, progress, podcast )
// VALUES ( $1, $2, 0, $3 )
//                 "#,
//                 &episode.title(),
//                 &episode.enclosure().unwrap().url(),
//                 &podcast.src,
//             )
//             .execute(&pool)
//             .await?;
//         }
//     }
//     Ok(())
// }
