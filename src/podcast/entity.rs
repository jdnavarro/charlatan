use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone)]
pub struct Podcast {
    pub id: i32,
    pub src: String,
    pub url: String,
    pub title: String,
    pub image: String,
    pub description: String,
}

pub struct NewPodcast<'a> {
    pub src: &'a str,
    pub url: &'a str,
    pub title: &'a str,
    pub image: &'a str,
    pub description: &'a str,
}

impl warp::Reply for Podcast {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self).into_response()
    }
}
