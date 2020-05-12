use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

use sqlx::sqlite::SqlitePool;
use warp::Filter;

mod episode;
pub(crate) mod podcast;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let pool = SqlitePool::builder()
        .build(&env::var("DATABASE_URL")?)
        .await?;

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    warp::serve(podcast::api(pool.clone()).or(episode::api(pool)))
        .run(bind_address)
        .await;

    Ok(())
}
