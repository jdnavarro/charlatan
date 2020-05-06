use std::collections::HashMap;

use diesel::prelude::{QueryDsl, RunQueryDsl};
use warp::Filter;

use charlatan_server::{
    create_podcast, establish_connection, fetch_all_episodes, models::Podcast, schema,
};

#[tokio::main]
async fn main() {
    let list_podcasts = warp::path!("podcasts").map(|| {
        // TODO: Pool connection
        let connection = establish_connection();
        let results = schema::podcast::table
            .load::<Podcast>(&connection)
            .expect("Error loading posts");
        warp::reply::json(&results)
    });

    let get_podcast = warp::path!("podcasts" / i32).map(|id| {
        // TODO: Pool connection
        let connection = establish_connection();
        let results = schema::podcast::table
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

    let fetch_episodes = warp::post().and(warp::path!("fetch")).map(|| {
        let connection = establish_connection();
        fetch_all_episodes(&connection);
        "Episodes refreshed"
    });

    let routes = web
        .or(add_podcast)
        .or(list_podcasts)
        .or(get_podcast)
        .or(fetch_episodes);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
