use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;

use crate::error::{user::UserError};
use crate::error::db::DbError;
use crate::error::password::PasswordError;
use crate::error::token::TokenError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error(transparent)]
    DbError(#[from] DbError),
    #[error(transparent)]
    PasswordError(#[from] PasswordError),
    #[error(transparent)]
    TokenError(#[from] TokenError),

}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::UserError(error) => error.into_response(),
            ApiError::DbError(error) => error.into_response(),
            ApiError::PasswordError(error) => error.into_response(),
            ApiError::TokenError(error) => error.into_response(),
        }
    }
}
