use std::{convert::Infallible, result::Result as StdResult};

use serde::Serialize;
use warp::{hyper::StatusCode, Reply};

pub(super) type Result<T, E> = std::result::Result<T, E>;

#[derive(Debug)]
struct WrappedResult<T, E>(Result<T, E>);

impl<T, E> Reply for WrappedResult<T, E>
where
    T: Send + Serialize,
    E: Send + std::fmt::Debug,
{
    fn into_response(self) -> warp::reply::Response {
        match self.0 {
            Ok(r) => warp::reply::json(&r).into_response(),
            Err(e) => {
                log::error!("sqlx returned err -- {:#?}", &e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

impl<T, E> From<Result<T, E>> for WrappedResult<T, E> {
    fn from(r: Result<T, E>) -> Self {
        Self(r)
    }
}

pub(super) fn respond<T, E>(db: Result<T, E>) -> StdResult<impl warp::Reply, Infallible>
where
    T: Send + Serialize,
    E: Send + std::fmt::Debug,
{
    let r: WrappedResult<T, E> = db.into();
    Ok(r)
}
