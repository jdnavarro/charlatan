use serde::{Deserialize, Serialize};

use crate::schema::podcast;

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
