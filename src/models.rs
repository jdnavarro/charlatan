use super::schema::podcasts;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Clone)]
pub struct Podcast {
    pub id: i32, // TODO: uuid
    pub title: String,
    pub url: String,
}

#[derive(Insertable)]
#[table_name = "podcasts"]
pub struct NewPodcast<'a> {
    pub title: &'a str,
    pub url: &'a str,
}
