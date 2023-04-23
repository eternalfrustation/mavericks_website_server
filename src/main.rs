use log::{info, warn};

use warp::Filter;

#[tokio::main]
async fn main() {
    femme::start();
    info!("Initializing routes");
    let routes = warp::fs::file("index.html").and(warp::path::end()).or(warp::fs::dir("./").and(warp::path::end()));
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
