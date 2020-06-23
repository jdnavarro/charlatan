use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub name: String,
    pub password: String,
}
