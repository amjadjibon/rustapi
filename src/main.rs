mod conf;

use axum::{
    routing::get, Router,
};

use conf::env;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // load .env file
    env::init();

    tracing_subscriber::fmt::init();

    let addr = format!("{}:{}", env::get("HOST"), env::get("PORT"));
    let app = Router::new()
        .route("/", get(root));

    println!("listening on: {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|e| panic!("server error: {}", e.to_string()));
}

async fn root() -> &'static str {
    info!("Hello, World!");
    "Hello, World!"
}