use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone)]
pub struct Podcast {
    pub id: i32, // TODO: uuid
    pub title: String,
    pub url: String,
}
