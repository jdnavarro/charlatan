use sqlx::sqlite::SqlitePool;

use super::entity::User;
use crate::auth;

type Result<T> = std::result::Result<T, auth::Error>;

#[allow(dead_code)]
pub(crate) async fn list(pool: SqlitePool) -> Result<Vec<User>> {
    Ok(sqlx::query_as!(
        User,
        r#"
SELECT name, password
FROM user
        "#
    )
    .fetch_all(&pool)
    .await?)
}

pub(crate) async fn get(pool: SqlitePool, name: &str) -> Result<User> {
    Ok(sqlx::query_as!(
        User,
        r#"
SELECT name, password
FROM user
WHERE name = ?
        "#,
        name
    )
    .fetch_one(&pool)
    .await?)
}

pub(crate) async fn add(pool: SqlitePool, name: &str, hash: &str) -> Result<()> {
    sqlx::query!(
        r#"
INSERT INTO user (name, password)
VALUES ($1, $2)
        "#,
        name,
        hash
    )
    .execute(&pool)
    .await?;
    Ok(())
}
