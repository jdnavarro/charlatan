use sqlx::sqlite::SqliteQueryAs;

use super::entity::{Episode, NewEpisode};
use crate::episode;

type Result<T> = std::result::Result<T, episode::Error>;

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
    pub(super) async fn list(&self) -> Result<Vec<Episode>> {
        Ok(sqlx::query_as!(
            Episode,
            r#"
SELECT id, guid, title, src, progress, duration, publication, image, position, notes, podcast
FROM episode
ORDER BY id ASC
        "#,
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub(super) async fn get_progress(&self, episode: i32) -> Result<i32> {
        let episode = sqlx::query!(
            r#"
SELECT progress
FROM episode
WHERE id = ?
        "#,
            episode,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(episode.progress)
    }
    pub(super) async fn set_progress(&self, episode: i32, progress: i32) -> Result<()> {
        sqlx::query!(
            r#"
UPDATE episode
SET progress = $1
WHERE id = $2
        "#,
            progress,
            episode,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub(super) async fn dequeue(&self, id: i32) -> Result<()> {
        match self.get_position(id).await? {
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
                .execute(&self.pool)
                .await?;
                sqlx::query!(
                    r#"
UPDATE episode
SET position = position - 1
WHERE position >= ?
                "#,
                    position
                )
                .execute(&self.pool)
                .await?;
                Ok(())
            }
        }
    }

    pub(super) async fn position(&self, id: i32, new_position: i32) -> Result<i32> {
        sqlx::query("SAVEPOINT update_queue")
            .execute(&self.pool)
            .await?;

        let max_position = self.get_max_position().await?;

        let new_position = if new_position > max_position {
            max_position + 1
        } else {
            new_position
        };

        let old_position = self.get_position(id).await?.unwrap_or(max_position);

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
            .execute(&self.pool)
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
            .execute(&self.pool)
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
        .execute(&self.pool)
        .await?;

        sqlx::query("RELEASE update_queue")
            .execute(&self.pool)
            .await?;

        Ok(new_position)
    }

    pub(crate) async fn add(&self, episode: &NewEpisode<'_>) -> Result<i32> {
        sqlx::query!(
        r#"
INSERT OR IGNORE INTO episode ( guid, title, progress, duration, publication, image, src, position, notes, podcast )
VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
        episode.guid,
        episode.title,
        0,
        episode.duration,
        episode.publication,
        episode.image,
        episode.src,
        None::<Option<i32>>,
        episode.notes,
        episode.podcast,
    )
    .execute(&self.pool)
    .await?;

        let (id,): (i32,) = sqlx::query_as("SELECT last_insert_rowid()")
            .fetch_one(&self.pool)
            .await?;

        Ok(id)
    }

    #[allow(dead_code)]
    pub(crate) async fn queue(&self) -> Result<Vec<Episode>> {
        Ok(sqlx::query_as!(
            Episode,
            r#"
SELECT id, guid, title, src, progress, duration, publication, image, position, notes, podcast
FROM episode
WHERE position IS NOT NULL
ORDER BY position;
        "#,
        )
        .fetch_all(&self.pool)
        .await?)
    }

    async fn get_max_position(&self) -> Result<i32> {
        let (p,): (i32,) = sqlx::query_as(
            r#"
SELECT MAX(position) FROM episode
        "#,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(p)
    }

    async fn get_position(&self, id: i32) -> Result<Option<i32>> {
        let (p,): (Option<i32>,) = sqlx::query_as(
            r#"
SELECT position
FROM episode
WHERE id = ?
        "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePool;
    use std::env;

    use super::*;

    async fn set_up(q: &str) -> anyhow::Result<Store> {
        let pool = SqlitePool::builder()
            .build(&env::var("DATABASE_URL")?)
            .await?;
        sqlx::query(q).execute(&pool).await?;
        Ok(Store::new(pool))
    }

    #[tokio::test]
    async fn get_position_null() {
        let store = set_up(
            r#"
CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER);
INSERT INTO episode (id, position) VALUES (1, NULL);
            "#,
        )
        .await
        .unwrap();

        assert_eq!(store.get_position(1).await.unwrap(), None);
    }

    #[tokio::test]
    async fn get_position_empty() -> anyhow::Result<()> {
        let store = set_up(
            r#"
    CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER)
                "#,
        )
        .await?;

        match store.position(2, 3).await {
            Err(episode::Error::NotFound) => Ok(()),
            e => panic!(e),
        }
    }

    #[tokio::test]
    async fn get_max_position_empty() {
        let store = set_up(
            r#"
    CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER)
                "#,
        )
        .await
        .unwrap();

        assert_eq!(store.get_max_position().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn position_above_maximum() {
        let store = set_up(
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

        assert_eq!(store.position(5, 6).await.unwrap(), 4);
    }

    #[tokio::test]
    async fn position_decrease() {
        let store = set_up(
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

        assert_eq!(store.position(5, 2).await.unwrap(), 2);
        assert_eq!(store.get_position(1).await.unwrap().unwrap(), 4);
        assert_eq!(store.get_position(2).await.unwrap().unwrap(), 3);
        assert_eq!(store.get_position(5).await.unwrap().unwrap(), 2);
        assert_eq!(store.get_position(3).await.unwrap().unwrap(), 1);
        assert_eq!(store.get_position(4).await.unwrap().unwrap(), 0);
    }

    #[tokio::test]
    async fn position_raise() {
        let store = set_up(
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

        assert_eq!(store.position(5, 2).await.unwrap(), 2);
        assert_eq!(store.get_position(1).await.unwrap().unwrap(), 4);
        assert_eq!(store.get_position(2).await.unwrap().unwrap(), 3);
        assert_eq!(store.get_position(5).await.unwrap().unwrap(), 2);
        assert_eq!(store.get_position(3).await.unwrap().unwrap(), 1);
        assert_eq!(store.get_position(4).await.unwrap().unwrap(), 0);
    }

    #[tokio::test]
    async fn position_null() {
        let store = set_up(
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

        assert_eq!(store.position(5, 2).await.unwrap(), 2);
        assert_eq!(store.get_position(1).await.unwrap().unwrap(), 4);
        assert_eq!(store.get_position(2).await.unwrap().unwrap(), 3);
        assert_eq!(store.get_position(5).await.unwrap().unwrap(), 2);
        assert_eq!(store.get_position(3).await.unwrap().unwrap(), 1);
        assert_eq!(store.get_position(4).await.unwrap().unwrap(), 0);
    }

    #[tokio::test]
    async fn dequeue_position_not_in_queue() {
        let store = set_up(
            r#"
    CREATE TEMPORARY TABLE episode (id INTEGER PRIMARY KEY, position INTEGER);
    INSERT INTO episode (id, position) VALUES (1, NULL);
                "#,
        )
        .await
        .unwrap();

        assert_eq!(store.dequeue(1).await.unwrap(), ());
        assert_eq!(store.get_position(1).await.unwrap(), None);
    }

    #[tokio::test]
    async fn dequeue_position_in_queue() {
        let store = set_up(
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

        assert_eq!(store.dequeue(3).await.unwrap(), ());
        assert_eq!(store.get_position(1).await.unwrap().unwrap(), 3);
        assert_eq!(store.get_position(2).await.unwrap().unwrap(), 2);
        assert_eq!(store.get_position(4).await.unwrap().unwrap(), 1);
        assert_eq!(store.get_position(5).await.unwrap().unwrap(), 0);
        assert_eq!(store.get_position(3).await.unwrap(), None);
    }
}
