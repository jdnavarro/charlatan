use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize, sqlx::FromRow)]
pub struct Episode {
    pub id: i32, // TODO: uuid
    pub title: String,
    pub url: String,
    pub podcast_id: i32,
}
