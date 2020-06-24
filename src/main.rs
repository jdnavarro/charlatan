use std::env;
use std::net::SocketAddr;

use sqlx::sqlite::SqlitePool;
use warp::Filter;

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

    let jwt_secret: String = env::var("JWT_SECRET").expect("JWT_SECRET is not set");

    warp::serve(charlatan::api(pool, jwt_secret).with(warp::log("charlatan")))
        .run(bind_address)
        .await;

    Ok(())
}
