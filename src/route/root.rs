use std::sync::Arc;
use axum::{
    routing::{
        get, IntoMakeService,
    }, Router,
};
use crate::db::postgres::Database;

use crate::route::{auth, profile, register};
use crate::state::{user::UserState, token::TokenState};

pub fn routes(db_conn: Arc<Database>) -> IntoMakeService<Router> {
    let api_router = {
        let user_state = UserState::new(&db_conn);
        let token_state = TokenState::new(&db_conn);

        auth::routes()
            .with_state(token_state)
            .merge(register::routes().with_state(user_state))
            .merge(profile::routes())
            .merge(Router::new().route("/health", get(|| async { "ok" })))
    };

    let router = Router::new()
        .nest("/api", api_router);

    router.into_make_service()
}
