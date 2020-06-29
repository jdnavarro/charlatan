use super::entity::User;
use crate::auth;

#[derive(Debug, Clone)]
pub struct Store {
    pub pool: sqlx::SqlitePool,
}

impl Store {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

type Result<T> = std::result::Result<T, auth::Error>;

impl Store {
    #[allow(dead_code)]
    pub(crate) async fn list(&self) -> Result<Vec<User>> {
        Ok(sqlx::query_as!(
            User,
            r#"
SELECT name, password
FROM user
        "#
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub(crate) async fn get(&self, name: &str) -> Result<User> {
        Ok(sqlx::query_as!(
            User,
            r#"
SELECT name, password
FROM user
WHERE name = ?
        "#,
            name
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub(crate) async fn add(self, name: &str, hash: &str) -> Result<()> {
        sqlx::query!(
            r#"
INSERT INTO user (name, password)
VALUES ($1, $2)
        "#,
            name,
            hash
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
