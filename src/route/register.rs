use axum::{routing::post, Router};

use crate::handler::register::{register};
use crate::state::user::UserState;

pub fn routes() -> Router<UserState> {
    let router = Router::new()
        .route("/register", post(register));
    return router;
}
