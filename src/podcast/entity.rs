use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone)]
pub struct Podcast {
    pub id: i32,
    pub src: String,
    pub title: String,
}
