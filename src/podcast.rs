pub(crate) mod db;
mod entity;
mod filter;
pub(crate) mod handler;

pub use entity::Podcast;
pub use filter::api;

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("No podcast")]
    NotFound,
    #[error(transparent)]
    DB(sqlx::Error),
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
