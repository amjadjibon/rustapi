use axum::response::{IntoResponse, Response};
use thiserror::Error;
use crate::code::common::get_common_code_object;
use crate::response::api::ApiErrorResponse;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Invalid token")]
    InvalidToken(String),
    #[error("Token has expired")]
    TokenExpired,
    #[error("Missing Bearer token")]
    MissingToken,
    #[error("Token error: {0}")]
    TokenCreationError(String),
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        let status_code = match self {
            TokenError::InvalidToken(_) => get_common_code_object("CODE_UE_400"),
            TokenError::TokenExpired => get_common_code_object("CODE_UE_400"),
            TokenError::MissingToken => get_common_code_object("CODE_UE_400"),
            TokenError::TokenCreationError(_) => get_common_code_object("CODE_UE_400"),
        };

        ApiErrorResponse::send(status_code)
    }
}