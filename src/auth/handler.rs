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

fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = argon2::Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}
