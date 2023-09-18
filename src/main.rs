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
use tokio::signal;
use tracing::log::info;
use conf::env;
use tracing_subscriber;
use db::postgres::{Database,DatabaseTrait};

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
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|e| panic!("server error: {}", e.to_string()));
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
