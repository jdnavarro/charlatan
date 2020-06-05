pub(crate) mod db;
mod entity;
mod filter;
mod handler;

pub use entity::{Episode, NewEpisode};
pub use filter::api;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No episodes")]
    NotFound,
    #[error(transparent)]
    DB(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        log::debug!("sqlx returned err -- {:#?}", &e);
        match e {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::DB(e),
        }
    }
}
