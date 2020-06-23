mod db;
mod entity;
mod filter;
mod handler;

pub use filter::api;

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("No user")]
    NotFound,
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
