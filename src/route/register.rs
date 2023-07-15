use axum::{routing::post, Router};

use crate::handler::register::{register};

pub fn routes() -> Router {
    let router = Router::new()
        .route("/register", post(register));
    return router;
}
