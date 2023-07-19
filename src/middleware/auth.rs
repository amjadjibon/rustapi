use axum::extract::State;
use axum::{http, http::Request, middleware::Next, response::IntoResponse};
use axum::headers::authorization::{Authorization, Bearer};
use axum::headers::Header;
use tracing::info;

use crate::error::api::ApiError;
use crate::error::token::TokenError;
use crate::state::token::TokenState;

pub async fn auth<B>(
    State(state): State<TokenState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, ApiError> {
    info!("auth middleware");

    let mut headers = req
        .headers_mut()
        .iter()
        .filter_map(|(header_name, header_value)| {
            if header_name == http::header::AUTHORIZATION {
                return Some(header_value);
            }
            None
        });
    let header: Authorization<Bearer> = Authorization::decode(&mut headers)
        .map_err(|_| TokenError::MissingToken)?;
    match state
        .token_service.decode_token(header.token()) {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data.claims);
            Ok(next.run(req).await)
        }
        Err(err) => {
            return Err(err)?
        },
    }
}