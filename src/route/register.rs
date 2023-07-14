use axum::{routing::post, Router};

use crate::handler::register::{register_handler};

pub fn routes() -> Router {
    let router = Router::new()
        .route("/register", post(register_handler));
    return router;
}
