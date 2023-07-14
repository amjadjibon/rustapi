use axum::{
    routing::get, Router,
};

use crate::handler::profile::profile_handler;

pub fn routes() -> Router {
    let router = Router::new()
        .route("/profile", get(profile_handler));
    return router;
}
