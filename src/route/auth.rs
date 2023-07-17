use axum::{routing::post, Router};

use crate::handler::auth::{login_handler, login_refresh_handler};
use crate::state::token::TokenState;

pub fn routes() -> Router<TokenState> {
    let router = Router::new()
        .route("/login", post(login_handler))
        .route("/login-refresh", post(login_refresh_handler));
    return router;
}
