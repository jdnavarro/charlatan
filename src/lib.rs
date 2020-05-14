use sqlx::sqlite::SqlitePool;
use warp::Filter;

pub mod episode;
pub(crate) mod error;
pub mod podcast;

pub(crate) fn with_pool(
    pool: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
