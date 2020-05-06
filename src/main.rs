use charlatan_server::{filters, models};

#[tokio::main]
async fn main() {
    let pool = models::establish_pool();

    let routes = filters::api(pool);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
