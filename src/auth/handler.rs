use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;

use super::db;
use super::entity::User;
use crate::auth::{self, encode_token, hash, verify};

pub(super) async fn register(
    p: SqlitePool,
    new_user: User,
) -> Result<impl warp::Reply, warp::Rejection> {
    match db::get(p.clone(), &new_user.name).await {
        Err(auth::Error::NotFound) => {
            match db::add(p, &new_user.name, &hash(&new_user.password.as_bytes())).await {
                Ok(()) => {
                    log::info!("Registered user: {}", &new_user.name);
                    Ok(StatusCode::CREATED)
                }
                Err(e) => {
                    log::error!("Error while registering user -- {:#?}", &e);
                    Ok(StatusCode::BAD_REQUEST)
                }
            }
        }
        _ => {
            log::warn!(
                "Trying to register an already existing user: {}",
                new_user.name
            );
            Ok(StatusCode::BAD_REQUEST)
        }
    }
}

pub(super) async fn login(
    p: SqlitePool,
    jwt_secret: String,
    credentials: User,
) -> Result<impl warp::Reply, warp::Rejection> {
    match db::get(p, &credentials.name).await {
        Err(auth::Error::NotFound) => {
            log::warn!("Unknown user: {}", &credentials.name);
            Ok(warp::reply::with_status(
                warp::reply::json(&"Unable to verify credentials".to_string()),
                StatusCode::UNAUTHORIZED,
            ))
        }
        Err(_) => Ok(warp::reply::with_status(
            warp::reply::json(&"Something went wrong".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
        Ok(user) => {
            if verify(&user.password, credentials.password.as_bytes()) {
                let token = encode_token(&jwt_secret, user.name);
                Ok(warp::reply::with_status(
                    warp::reply::json(&token),
                    StatusCode::OK,
                ))
            } else {
                Ok(warp::reply::with_status(
                    warp::reply::json(&"Unable to verify credentials".to_string()),
                    StatusCode::UNAUTHORIZED,
                ))
            }
        }
    }
}
