use thiserror::Error;
use warp::http::StatusCode;
use warp::Reply;

use crate::auth;
use crate::response;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No episodes")]
    NotFound,
    #[error(transparent)]
    DB(sqlx::Error),
    #[error("Authentication error")]
    Auth(auth::Error),
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        log::error!("sqlx returned err -- {:#?}", &e);
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
            Error::DB(_) => Self(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
            Error::Auth(_) => Self(StatusCode::UNAUTHORIZED.into_response()),
        }
    }
}
