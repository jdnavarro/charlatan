mod db;
mod entity;
mod error;
pub(crate) mod filter;
pub(crate) mod handler;

use chrono::{Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub use db::Store;
pub use error::Error;
pub use filter::api;

const TOKEN_PREFIX: &str = "Bearer ";

pub(crate) fn identify(secret: &str, token: &str) -> Result<String, Error> {
    Ok(decode_token(secret, token).map_err(Error::JWT)?.sub)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Claims {
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

fn decode_token(secret: &str, token: &str) -> jsonwebtoken::errors::Result<Claims> {
    jsonwebtoken::decode::<Claims>(
        token.trim_start_matches(TOKEN_PREFIX),
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map(|token_data| token_data.claims)
}

fn encode_token(secret: &str, sub: String) -> String {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims::new(sub),
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = argon2::Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}
