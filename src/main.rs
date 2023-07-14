mod conf;
mod route;

use conf::env;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // load .env file
    env::init();

    tracing_subscriber::fmt::init();

    let addr = format!("{}:{}", env::get("HOST"), env::get("PORT"));

    println!("listening on: {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(route::root::routes())
        .await
        .unwrap_or_else(|e| panic!("server error: {}", e.to_string()));
}
