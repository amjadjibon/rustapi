use axum::{
    routing::get, Router,
};

use crate::handler::profile::profile_handler;
use crate::state::user::UserState;

pub fn routes() -> Router<UserState> {
    let router = Router::new()
        .route("/profile", get(profile_handler));
    return router;
}
