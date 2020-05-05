use super::schema::podcast;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Clone)]
pub struct Podcast {
    pub id: i32, // TODO: uuid
    pub title: String,
    pub url: String,
}

#[derive(Insertable)]
#[table_name = "podcast"]
pub struct NewPodcast<'a> {
    pub title: &'a str,
    pub url: &'a str,
}

#[derive(Queryable, Deserialize, Clone, Serialize)]
pub struct Episode {
    pub id: i32, // TODO: uuid
    pub title: String,
    pub url: String,
    pub podcast_id: i32,
}
