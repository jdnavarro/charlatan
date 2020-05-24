use super::db;
use crate::json_reply;
use sqlx::sqlite::SqlitePool;

pub(super) async fn list(p: SqlitePool) -> Result<impl warp::Reply, warp::Rejection> {
    json_reply(db::list(p).await)
}

pub(super) async fn get(p: SqlitePool, src: String) -> Result<impl warp::Reply, warp::Rejection> {
    json_reply(db::get(p, src).await)
}

pub(super) async fn add(p: SqlitePool, src: String) -> Result<impl warp::Reply, warp::Rejection> {
    json_reply(db::add(p, src).await)
}
