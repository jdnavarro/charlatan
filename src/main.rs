use std::env;
use std::net::SocketAddr;

use sqlx::sqlite::SqlitePool;
use warp::Filter;

use charlatan_server::episode;
use charlatan_server::podcast;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "charlatan=debug");

    pretty_env_logger::init();

    let pool = SqlitePool::builder()
        .build(&env::var("DATABASE_URL")?)
        .await?;

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    warp::serve(
        podcast::api(pool.clone())
            .or(episode::api(pool))
            .with(warp::log("charlatan")),
    )
    .run(bind_address)
    .await;

    Ok(())
}
