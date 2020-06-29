use jsonwebtoken::errors::Error as JwtError;
use thiserror::Error;
use warp::hyper::StatusCode;
use warp::{reject::Reject, Reply};

use crate::response;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No user")]
    NotFound,
    #[error(transparent)]
    JWT(JwtError),
    #[error(transparent)]
    DB(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::DB(e),
        }
    }
}

impl From<Error> for response::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound => Self(StatusCode::NOT_FOUND.into_response()),
            Error::JWT(_) => Self(StatusCode::UNAUTHORIZED.into_response()),
            Error::DB(_) => Self(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
        }
    }
}

impl Reject for Error {}
