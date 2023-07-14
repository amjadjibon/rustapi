use axum::{
    routing::{
        get, IntoMakeService,
    }, Router,
};

use crate::route::{auth, profile, register};

pub fn routes() -> IntoMakeService<Router> {
    let api_router = {
        auth::routes()
            .merge(register::routes())
            .merge(profile::routes())
            .merge(Router::new().route("/health", get(|| async { "ok" })))
    };

    let router = Router::new()
        .nest("/api", api_router);

    router.into_make_service()
}
