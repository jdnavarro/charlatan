use std::convert::Infallible;

use sqlx::sqlite::SqliteQueryAs;
use warp::Filter;

use crate::auth;
use crate::episode;
use crate::podcast;

#[derive(Debug, Clone)]
pub struct App {
    pool: sqlx::SqlitePool,
    pub(crate) jwt_secret: String,
    pub episode: episode::Store,
    pub podcast: podcast::Store,
    pub auth: auth::Store,
}

impl App {
    pub fn new(pool: sqlx::SqlitePool, jwt_secret: String) -> Self {
        Self {
            pool: pool.clone(),
            jwt_secret,
            episode: episode::Store::new(pool.clone()),
            podcast: podcast::Store::new(pool.clone()),
            auth: auth::Store::new(pool),
        }
    }

    pub fn identify(&self, token: &str) -> Result<String, auth::Error> {
        auth::identify(&self.jwt_secret, token)
    }

    pub(super) async fn configured(self) -> Result<bool, sqlx::Error> {
        let (n,): (i32,) = sqlx::query_as(
            r#"
SELECT COUNT() FROM USER;
            "#,
        )
        .fetch_one(&self.pool)
        .await?;
        if n > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

pub fn with_app(app: App) -> impl Filter<Extract = (App,), Error = Infallible> + Clone {
    warp::any().map(move || app.clone())
}
