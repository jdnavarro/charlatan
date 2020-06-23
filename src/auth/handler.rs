use rand::Rng;
use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;

use super::db;
use super::entity::User;
use crate::auth;

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
    credentials: User,
) -> Result<impl warp::Reply, warp::Rejection> {
    match db::get(p, &credentials.name).await {
        Err(auth::Error::NotFound) => {
            log::warn!(
                "Trying to login with unexisting user: {}",
                &credentials.name
            );
            Ok(StatusCode::BAD_REQUEST)
        }
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(user) => {
            if verify(&user.password, credentials.password.as_bytes()) {
                Ok(StatusCode::OK)
            } else {
                Ok(StatusCode::UNAUTHORIZED)
            }
        }
    }
}

fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = argon2::Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}
