use std::convert::Infallible;

use warp::Filter;

use crate::auth;
use crate::episode;
use crate::podcast;

#[derive(Debug, Clone)]
pub struct App {
    pool: sqlx::SqlitePool,
    pub(crate) jwt_secret: String,
    pub episode: episode::App,
    pub podcast: podcast::Store,
    pub auth: auth::Store,
}

impl App {
    pub fn new(pool: sqlx::SqlitePool, jwt_secret: String) -> Self {
        Self {
            pool: pool.clone(),
            jwt_secret,
            episode: episode::App::new(pool.clone()),
            podcast: podcast::Store::new(pool.clone()),
            auth: auth::Store::new(pool),
        }
    }

    pub fn identify(&self, token: &str) -> Result<String, auth::Error> {
        auth::identify(&self.jwt_secret, token)
    }
}

pub fn with_app(app: App) -> impl Filter<Extract = (App,), Error = Infallible> + Clone {
    warp::any().map(move || app.clone())
}
