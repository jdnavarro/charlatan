use serde::{Deserialize, Serialize};

use crate::schema::episode;

#[derive(Queryable, Deserialize, Clone, Serialize)]
pub struct Episode {
    pub id: i32, // TODO: uuid
    pub title: String,
    pub url: String,
    pub podcast_id: i32,
}

#[derive(Insertable)]
#[table_name = "episode"]
pub struct NewEpisode<'a> {
    pub title: &'a str,
    pub url: &'a str,
    pub podcast_id: &'a i32,
}
