mod filter;
mod handler;

use crate::episode;

pub use filter::api;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing audio source for episode: `{0}`")]
    MissingSource(i32),
    #[error(transparent)]
    Episode(episode::Error),
    #[error(transparent)]
    Rss(rss::Error),
}

impl From<rss::Error> for Error {
    fn from(e: rss::Error) -> Self {
        log::error!("rss returned err -- {:#?}", &e);
        Error::Rss(e)
    }
}

impl From<episode::Error> for Error {
    fn from(e: episode::Error) -> Self {
        log::error!("Episode error -- {:#?}", &e);
        Error::Episode(e)
    }
}
