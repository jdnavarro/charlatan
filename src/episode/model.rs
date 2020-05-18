use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize, sqlx::FromRow)]
pub struct Episode {
    pub id: i32, // Replace with UUID, u128 for sqlite not implemented yet in sqlx
    pub title: String,
    pub src: String,
    pub podcast: String,
}

#[derive(Deserialize, Clone, Serialize, sqlx::FromRow)]
pub struct UserEpisode {
    pub user: i32,
    pub episode: i32,
    pub progress: i32,
}
