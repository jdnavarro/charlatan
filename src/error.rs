#[derive(Debug)]
pub(crate) enum Error {
    Database(sqlx::Error),
}

impl warp::reject::Reject for Error {}
