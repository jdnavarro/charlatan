#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

pub mod filters;
pub mod handlers;
pub mod models;
pub mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = models::establish_pool(&database_url);

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    let routes = filters::api(pool);

    warp::serve(routes).run(bind_address).await;
}
