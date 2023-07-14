use axum::{routing::post, Router};

pub fn routes() -> Router {
    let router = Router::new()
        .route("/login", post(login_handler))
        .route("/login-refresh", post(login_refresh_handler));
    return router;
}

async fn login_handler() -> &'static str {
    "login handler"
}

async fn login_refresh_handler() -> &'static str {
    "login refresh handler"
}
