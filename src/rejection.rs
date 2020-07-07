use serde::Serialize;
use warp::{hyper::StatusCode, Rejection, Reply};

use crate::{auth, App};

#[derive(Serialize)]
struct Error {
    #[serde(rename = "type")]
    type_: String,
}

pub(super) async fn handle_rejection(
    app: App,
    err: Rejection,
) -> Result<impl Reply, warp::Rejection> {
    let configured = app.configured().await;

    let code;
    let type_;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        type_ = "NotFound";
    } else if let Some(auth::Error::JWT(_)) = err.find() {
        match configured {
            Ok(true) => {
                code = StatusCode::UNAUTHORIZED;
                type_ = "UnverifiedToken"
            }
            Ok(false) => {
                code = StatusCode::UNAUTHORIZED;
                type_ = "Unconfigured"
            }
            Err(_) => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                type_ = "Internal";
            }
        }
    } else if let Some(_) = err.find::<warp::reject::MissingHeader>() {
        match configured {
            Ok(true) => {
                code = StatusCode::UNAUTHORIZED;
                type_ = "MissingAuthorizationHeader"
            }
            Ok(false) => {
                code = StatusCode::UNAUTHORIZED;
                type_ = "Unconfigured"
            }
            Err(_) => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                type_ = "Internal";
            }
        }
    } else {
        log::error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        type_ = "UnhandledRejection";
    }

    let json = warp::reply::json(&Error {
        type_: type_.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
