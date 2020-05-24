use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize, sqlx::FromRow)]
pub struct Episode {
    pub id: i32,
    pub title: String,
    pub src: String,
    pub progress: i32,
    pub position: Option<i32>,
    pub podcast: String,
}
