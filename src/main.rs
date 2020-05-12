#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    sqlite::SqliteConnection,
};
use warp::Filter;

mod episode;
pub(crate) mod podcast;
pub mod schema;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
pub type PooledSqliteConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = establish_pool(&database_url);

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    warp::serve(api(pool)).run(bind_address).await;
}

fn establish_pool(database_url: &str) -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::new(manager).expect(&format!("Error connecting to {}", database_url))
}

fn api(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    podcast::api(pool.clone()).or(episode::api(pool))
}

pub(crate) fn with_pool(
    pool: SqlitePool,
) -> impl Filter<Extract = (PooledSqliteConnection,), Error = Infallible> + Clone {
    // TODO: Return 503 or something
    warp::any().map(move || pool.clone().get().expect("Unable to connect to the db"))
}
