use diesel::prelude::*;
use warp::Filter;

use charlatan_server::establish_connection;
use charlatan_server::models::Podcast;
use charlatan_server::schema::podcasts::dsl::podcasts;

#[tokio::main]
async fn main() {
    let route = warp::path!("podcasts").map(|| {
        // TODO: Pool connection
        let connection = establish_connection();
        let results = podcasts
            .load::<Podcast>(&connection)
            .expect("Error loading posts");
        serde_json::to_string(&results).unwrap()
    });

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
