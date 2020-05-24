// use rss::Channel;
use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

use super::entity::Episode;
use crate::episode;

type Result<T> = std::result::Result<T, episode::Error>;

pub(super) async fn list(pool: SqlitePool) -> Result<Vec<Episode>> {
    Ok(sqlx::query_as!(
        Episode,
        r#"
SELECT id, title, src, progress, position, podcast
FROM episode
ORDER BY id ASC
LIMIT 50
        "#,
    )
    .fetch_all(&pool)
    .await?)
}

pub(super) async fn get_progress(pool: SqlitePool, episode: i32) -> Result<i32> {
    let episode = sqlx::query!(
        r#"
SELECT progress
FROM episode
WHERE id = ?
        "#,
        episode,
    )
    .fetch_one(&pool)
    .await?;
    Ok(episode.progress)
}

pub(super) async fn set_progress(pool: SqlitePool, episode: i32, progress: i32) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE episode
SET progress = $1
WHERE id = $2
        "#,
        progress,
        episode,
    )
    .execute(&pool)
    .await?;
    Ok(())
}

pub(crate) async fn queue(pool: SqlitePool) -> Result<Vec<Episode>> {
    Ok(sqlx::query_as!(
        Episode,
        r#"
SELECT id, title, src, progress, position, podcast
FROM episode
WHERE position IS NOT NULL
ORDER BY position;
        "#,
    )
    .fetch_all(&pool)
    .await?)
}

pub async fn position(pool: &SqlitePool, id: i32, new_position: i32) -> Result<i32> {
    sqlx::query("SAVEPOINT update_queue").execute(pool).await?;

    let max_position = get_max_position(&pool).await?;

    let new_position = if new_position > max_position {
        max_position + 1
    } else {
        new_position
    };

    let old_position = get_position(&pool, id).await?.unwrap_or(max_position);

    if new_position > old_position {
        sqlx::query!(
            r#"
UPDATE episode
SET position = position - 1
WHERE position >= $1
AND position <= $2
                    "#,
            old_position,
            new_position,
        )
        .execute(pool)
        .await?;
    } else {
        sqlx::query!(
            r#"
UPDATE episode
SET position = position + 1
WHERE position >= $1
AND position <= $2
        "#,
            new_position,
            old_position,
        )
        .execute(pool)
        .await?;
    }

    sqlx::query!(
        r#"
UPDATE episode
SET position = $1
WHERE id = $2;
        "#,
        new_position,
        id,
    )
    .execute(pool)
    .await?;

    sqlx::query("RELEASE update_queue").execute(pool).await?;

    Ok(new_position)
}

pub(crate) async fn delete(pool: &SqlitePool, id: i32) -> Result<()> {
    match get_position(&pool, id).await? {
        None => Ok(()),
        Some(position) => {
            sqlx::query!(
                r#"
        UPDATE episode
        SET position = NULL
        WHERE id = ?;
                "#,
                id
            )
            .execute(pool)
            .await?;

            sqlx::query!(
                r#"
        UPDATE episode
        SET position = position - 1
        WHERE position >= ?
                "#,
                position
            )
            .execute(pool)
            .await?;

            Ok(())
        }
    }
}

async fn get_max_position(pool: &SqlitePool) -> Result<i32> {
    let (p,): (i32,) = sqlx::query_as(
        r#"
SELECT MAX(position) FROM episode
        "#,
    )
    .fetch_one(pool)
    .await?;
    Ok(p)
}

async fn get_position(pool: &SqlitePool, id: i32) -> Result<Option<i32>> {
    let (p,): (Option<i32>,) = sqlx::query_as(
        r#"
SELECT position
FROM episode
WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(p)
}

// pub(super) async fn crawl(pool: SqlitePool) -> Result<()> {
//     let podcasts = podcast::handler::list(pool.clone()).await?;
//     for podcast in podcasts {
//         // TODO: Bubble up error
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

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePool;
    use std::env;

    use super::*;

    async fn set_up(q: &str) -> anyhow::Result<SqlitePool> {
        let pool = SqlitePool::builder()
            .build(&env::var("DATABASE_URL")?)
            .await?;
        sqlx::query(q).execute(&pool).await?;
        Ok(pool)
    }

    #[tokio::test]
    async fn get_position_null() {
        let pool = set_up(
            r#"
CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER);
INSERT INTO episode (id, position) VALUES (1, NULL);
            "#,
        )
        .await
        .unwrap();

        assert_eq!(get_position(&pool, 1).await.unwrap(), None);
    }

    #[tokio::test]
    async fn get_position_empty() -> anyhow::Result<()> {
        let pool = set_up(
            r#"
CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER)
            "#,
        )
        .await?;

        match position(&pool, 2, 3).await {
            Err(episode::Error::NotFound) => Ok(()),
            e => panic!(e),
        }
    }

    #[tokio::test]
    async fn get_max_position_empty() {
        let pool = set_up(
            r#"
CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER)
            "#,
        )
        .await
        .unwrap();

        assert_eq!(get_max_position(&pool).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn position_above_maximum() {
        let pool = set_up(
            r#"
CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER);
INSERT INTO episode (id, position) VALUES (1, 3);
INSERT INTO episode (id, position) VALUES (2, 2);
INSERT INTO episode (id, position) VALUES (3, 1);
INSERT INTO episode (id, position) VALUES (4, 0);
INSERT INTO episode (id, position) VALUES (5, NULL);
            "#,
        )
        .await
        .unwrap();

        assert_eq!(position(&pool, 5, 6).await.unwrap(), 4);
    }

    #[tokio::test]
    async fn position_decrease() {
        let pool = set_up(
            r#"
CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER);
INSERT INTO episode (id, position) VALUES (1, 3);
INSERT INTO episode (id, position) VALUES (2, 2);
INSERT INTO episode (id, position) VALUES (3, 1);
INSERT INTO episode (id, position) VALUES (4, 0);
INSERT INTO episode (id, position) VALUES (5, 4);
            "#,
        )
        .await
        .unwrap();

        assert_eq!(position(&pool, 5, 2).await.unwrap(), 2);
        assert_eq!(get_position(&pool, 1).await.unwrap().unwrap(), 4);
        assert_eq!(get_position(&pool, 2).await.unwrap().unwrap(), 3);
        assert_eq!(get_position(&pool, 5).await.unwrap().unwrap(), 2);
        assert_eq!(get_position(&pool, 3).await.unwrap().unwrap(), 1);
        assert_eq!(get_position(&pool, 4).await.unwrap().unwrap(), 0);
    }

    #[tokio::test]
    async fn position_raise() {
        let pool = set_up(
            r#"
CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER);
INSERT INTO episode (id, position) VALUES (1, 4);
INSERT INTO episode (id, position) VALUES (2, 3);
INSERT INTO episode (id, position) VALUES (3, 2);
INSERT INTO episode (id, position) VALUES (4, 1);
INSERT INTO episode (id, position) VALUES (5, 0);
            "#,
        )
        .await
        .unwrap();

        assert_eq!(position(&pool, 5, 2).await.unwrap(), 2);
        assert_eq!(get_position(&pool, 1).await.unwrap().unwrap(), 4);
        assert_eq!(get_position(&pool, 2).await.unwrap().unwrap(), 3);
        assert_eq!(get_position(&pool, 5).await.unwrap().unwrap(), 2);
        assert_eq!(get_position(&pool, 3).await.unwrap().unwrap(), 1);
        assert_eq!(get_position(&pool, 4).await.unwrap().unwrap(), 0);
    }

    #[tokio::test]
    async fn position_null() {
        let pool = set_up(
            r#"
CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER);
INSERT INTO episode (id, position) VALUES (1, 3);
INSERT INTO episode (id, position) VALUES (2, 2);
INSERT INTO episode (id, position) VALUES (3, 1);
INSERT INTO episode (id, position) VALUES (4, 0);
INSERT INTO episode (id, position) VALUES (5, NULL);
            "#,
        )
        .await
        .unwrap();

        assert_eq!(position(&pool, 5, 2).await.unwrap(), 2);
        assert_eq!(get_position(&pool, 1).await.unwrap().unwrap(), 4);
        assert_eq!(get_position(&pool, 2).await.unwrap().unwrap(), 3);
        assert_eq!(get_position(&pool, 5).await.unwrap().unwrap(), 2);
        assert_eq!(get_position(&pool, 3).await.unwrap().unwrap(), 1);
        assert_eq!(get_position(&pool, 4).await.unwrap().unwrap(), 0);
    }

    #[tokio::test]
    async fn delete_position_null() {
        let pool = set_up(
            r#"
CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER);
INSERT INTO episode (id, position) VALUES (1, NULL);
            "#,
        )
        .await
        .unwrap();

        assert_eq!(delete(&pool, 1).await.unwrap(), ());
        assert_eq!(get_position(&pool, 1).await.unwrap(), None);
    }

    #[tokio::test]
    async fn delete_position_in_queue() {
        let pool = set_up(
            r#"
CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER);
INSERT INTO episode (id, position) VALUES (1, 4);
INSERT INTO episode (id, position) VALUES (2, 3);
INSERT INTO episode (id, position) VALUES (3, 2);
INSERT INTO episode (id, position) VALUES (4, 1);
INSERT INTO episode (id, position) VALUES (5, 0);
            "#,
        )
        .await
        .unwrap();

        assert_eq!(delete(&pool, 3).await.unwrap(), ());
        assert_eq!(get_position(&pool, 1).await.unwrap().unwrap(), 3);
        assert_eq!(get_position(&pool, 2).await.unwrap().unwrap(), 2);
        assert_eq!(get_position(&pool, 4).await.unwrap().unwrap(), 1);
        assert_eq!(get_position(&pool, 5).await.unwrap().unwrap(), 0);
        assert_eq!(get_position(&pool, 3).await.unwrap(), None);
    }
}
