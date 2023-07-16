use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::error::{user::UserError};
use crate::error::db::DbError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error(transparent)]
    DbError(#[from] DbError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::UserError(error) => error.into_response(),
            ApiError::DbError(error) => error.into_response(),
        }
    }
}
