use axum::{routing::post, Router};

pub fn routes() -> Router {
    let router = Router::new()
        .route("/register", post(register_handler));
    return router;
}

async fn register_handler() -> &'static str {
    "register handler"
}