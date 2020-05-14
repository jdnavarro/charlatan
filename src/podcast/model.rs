use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone)]
pub struct Podcast {
    pub uri: String,
    pub title: String,
}
