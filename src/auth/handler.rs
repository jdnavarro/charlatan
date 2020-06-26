use rand::Rng;
use sqlx::sqlite::SqlitePool;
use warp::http::StatusCode;

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use super::db;
use super::entity::User;
use crate::auth;

const TOKEN_PREFIX: &str = "Bearer ";

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

fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = argon2::Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}

fn encode_token(secret: &str, sub: String) -> String {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims::new(sub),
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub(crate) fn decode_token(secret: &str, token: &str) -> jsonwebtoken::errors::Result<Claims> {
    jsonwebtoken::decode::<Claims>(
        token.trim_start_matches(TOKEN_PREFIX),
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map(|token_data| token_data.claims)
}

pub(crate) fn identify(secret: &str, token: &str) -> Result<String, auth::Error> {
    Ok(decode_token(secret, token).map_err(auth::Error::JWT)?.sub)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    fn new(user_name: String) -> Self {
        Self {
            sub: user_name,
            exp: (Utc::now() + Duration::weeks(3)).timestamp() as usize,
        }
    }
}
