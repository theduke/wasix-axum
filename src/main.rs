use axum::{
    response::IntoResponse,
    routing::{any, get},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "wasix_axum=trace");
    }
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // /
        .route("/", get(root))
        .fallback(any(not_found));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello! Axum <3 WASIX!"
}

async fn not_found() -> impl IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "Not Found!")
}



