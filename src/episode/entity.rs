use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize, sqlx::FromRow)]
pub struct Episode {
    pub id: i32,
    pub guid: String,
    pub title: String,
    pub progress: i32,
    pub duration: i32,
    pub publication: i32,
    pub image: String,
    pub src: String,
    pub position: Option<i32>,
    pub podcast: i32,
}

pub struct NewEpisode<'a> {
    pub guid: &'a str,
    pub title: &'a str,
    pub duration: &'a str,    // Parse i32,
    pub publication: &'a str, // Parse i32,
    pub image: &'a str,
    pub src: &'a str,
    pub podcast: i32,
}
