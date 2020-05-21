use crate::entity::Episode;
use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

pub(super) async fn list(pool: SqlitePool) -> Result<Vec<Episode>, sqlx::Error> {
    sqlx::query_as!(
        Episode,
        r#"
SELECT id, title, src, progress, position, podcast
FROM episode
WHERE position IS NOT NULL
ORDER BY position;
        "#,
    )
    .fetch_all(&pool)
    .await
}

pub(super) async fn add(pool: SqlitePool, id: i32) -> Result<i32, sqlx::Error> {
    let (position,): (i32,) = sqlx::query_as("SELECT MAX(position) + 1 FROM episode")
        .fetch_one(&pool)
        .await?;

    sqlx::query!(
        r#"
UPDATE episode
SET position = $1
WHERE id = $2
AND position IS NULL;
        "#,
        position,
        id
    )
    .execute(&pool)
    .await?;
    Ok(position)
}

pub(super) async fn delete(pool: SqlitePool, id: i32) -> Result<(), sqlx::Error> {
    let (position,): (i32,) = sqlx::query_as(
        r#"
SELECT position
FROM episode
WHERE id = ?;
        "#,
    )
    .bind(id)
    .fetch_one(&pool)
    .await?;

    sqlx::query!(
        r#"
UPDATE episode
SET position = NULL
WHERE id = ?;
        "#,
        id
    )
    .execute(&pool)
    .await?;

    // TODO: Handle null
    sqlx::query!(
        r#"
UPDATE episode
SET position = position - 1
WHERE position >= ?
AND position IS NOT NULL;
        "#,
        position
    )
    .execute(&pool)
    .await?;

    Ok(())
}

// mod tests {
//     use std::env;

//     use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

//     use super::*;

//     #[tokio::test]
//     async fn it_fails_with_not_found_episode_when_no_episodes() -> anyhow::Result<()> {
//         let pool = SqlitePool::builder()
//             .build(&env::var("DATABASE_URL")?)
//             .await?;

//         let mut conn = pool.acquire().await?;

//         let _ = conn
//             .execute(
//                 r#"
// CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER)
//                 "#,
//             )
//             .await?;

//         match position(pool, 2, 3).await {
//             Err(EpisodeError::NotFound) => Ok(()),
//             Err(e) => panic!(e),
//             Ok(msg) => panic!(msg),
//         }

//         //         let _ = conn
//         //             .execute(
//         //                 r#"
//         // SELECT count(*) from episode;
//         //                 "#,
//         //             )
//         //             .await?;

//         // assert_eq!(affected, 0);

//         // Ok(())
//     }
// }
