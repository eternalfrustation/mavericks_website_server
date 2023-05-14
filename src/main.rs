use log::{info, warn};

use warp::{
    filters::BoxedFilter,
    fs::dir,
    hyper::StatusCode,
    path::{self, path},
    Filter, Reply,
};

#[tokio::main]
async fn main() {
    femme::start();
    info!("Initializing routes");
    let routes = src_routes()
        .with(warp::compression::gzip())
        .or(bin_routes());
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn src_routes() -> BoxedFilter<(impl Reply,)> {
    warp::fs::file("static/index.html")
        .and(path::end())
        .or(warp::path::param().and(path::end()).map(|path: String| {
            match std::fs::read(format!("static/html/{path}.html")) {
                Err(err) => warp::reply::with_status(
                    warp::reply::html(err.to_string()),
                    StatusCode::NOT_FOUND,
                ),
                Ok(v) => warp::reply::with_status(
                    warp::reply::html(String::from_utf8(v).unwrap()),
                    StatusCode::OK,
                ),
            }
        }))
        .or(path("css").and(dir("static/css")))
        .or(path("js").and(dir("static/js")))
        .boxed()
}

fn bin_routes() -> BoxedFilter<(impl Reply,)> {
    path("icons")
        .and(dir("static/icons"))
        .or(path("images").and(dir("static/images")))
        .boxed()
}
