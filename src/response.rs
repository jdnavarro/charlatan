use warp::{hyper::StatusCode, Reply};

pub type Response = std::result::Result<warp::reply::Json, Error>;

#[derive(Debug)]
pub struct Error(pub warp::reply::Response);

impl warp::Reply for Error {
    fn into_response(self) -> warp::reply::Response {
        self.0
    }
}

pub fn unify(response: Response) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    Ok(response.map_or_else(|j| j.into_response(), |e| e.into_response()))
}

pub fn bad() -> Error {
    Error(StatusCode::BAD_REQUEST.into_response())
}
