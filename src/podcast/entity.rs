use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone)]
pub struct Podcast {
    pub id: i32,
    pub src: String,
    pub title: String,
}

impl warp::Reply for Podcast {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self).into_response()
    }
}
