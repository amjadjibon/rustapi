use axum::{routing::post, Router};

use crate::handler::auth::{login_handler, login_refresh_handler};

pub fn routes() -> Router {
    let router = Router::new()
        .route("/login", post(login_handler))
        .route("/login-refresh", post(login_refresh_handler));
    return router;
}
