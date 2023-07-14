use axum::{
    routing::get, Router,
};

pub fn routes() -> Router {
    let router = Router::new()
        .route("/profile", get(profile_handler));
    return router;
}

async fn profile_handler() -> &'static str {
    "profile handler"
}
