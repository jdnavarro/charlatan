use diesel::prelude::*;
use warp::Filter;

use charlatan_server::models::Podcast;
use charlatan_server::schema::podcast::dsl as schema;
use charlatan_server::{create_podcast, establish_connection};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let list_podcasts = warp::path!("podcasts").map(|| {
        // TODO: Pool connection
        let connection = establish_connection();
        let results = schema::podcast
            .load::<Podcast>(&connection)
            .expect("Error loading posts");
        warp::reply::json(&results)
    });

    let get_podcast = warp::path!("podcasts" / i32).map(|id| {
        // TODO: Pool connection
        let connection = establish_connection();
        let results = schema::podcast
            .find(id)
            .load::<Podcast>(&connection)
            .expect("Error loading posts");
        warp::reply::json(&results)
    });

    let add_podcast = warp::post()
        .and(warp::path!("podcasts"))
        // TODO: Limit payload size
        .and(warp::body::json())
        .map(|hm: HashMap<String, String>| {
            // TODO: Pool connection
            let connection = establish_connection();

            let title = hm.get("title").unwrap();
            let url = hm.get("url").unwrap();

            let podcast = create_podcast(&connection, &title, &url);

            warp::reply::json(&podcast)
        });

    let web = warp::fs::dir("./web/build/");

    let routes = web.or(list_podcasts).or(get_podcast).or(add_podcast);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
