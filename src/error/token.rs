use axum::response::{IntoResponse, Response};
use thiserror::Error;
use crate::code::common::get_code_object;
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
    #[error("Token decode error: {0}")]
    TokenDecodeError(String),
    #[error("Invalid token type")]
    InvalidTokenType,
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        let status_code = match self {
            TokenError::InvalidToken(_) => get_code_object("CODE_UE_400"),
            TokenError::TokenExpired => get_code_object("CODE_UE_400"),
            TokenError::MissingToken => get_code_object("CODE_UE_400"),
            TokenError::TokenCreationError(_) => get_code_object("CODE_UE_400"),
            TokenError::TokenDecodeError(_) => get_code_object("CODE_UE_400"),
            TokenError::InvalidTokenType => get_code_object("CODE_UE_400"),
        };

        ApiErrorResponse::send(status_code)
    }
}