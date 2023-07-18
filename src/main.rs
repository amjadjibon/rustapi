mod conf;
mod route;
mod handler;
mod model;
mod error;
mod response;
mod code;
mod state;
mod db;
mod repo;
mod service;
mod utils;
mod middleware;

use std::sync::Arc;
use tracing::log::info;
use conf::env;
use tracing_subscriber;

use crate::db::postgres::{Database,DatabaseTrait};

#[tokio::main]
async fn main() {
    // load .env file
    env::init();

    let db_conn = Database::init()
        .await
        .unwrap_or_else(|e| panic!("Database error: {}", e.to_string()));

    tracing_subscriber::fmt::init();

    let addr = format!("{}:{}", env::get("HOST"), env::get("PORT"));

    info!("Listening on {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(route::root::routes(Arc::new(db_conn)))
        .await
        .unwrap_or_else(|e| panic!("server error: {}", e.to_string()));
}
