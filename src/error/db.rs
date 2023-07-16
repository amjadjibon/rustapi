use axum::{response::{IntoResponse, Response}};
use thiserror::Error;

use crate::response::api::ApiErrorResponse;
use crate::code::common::get_common_code_object;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("{0}")]
    SomethingWentWrong(String),
    #[error("Duplicate entry exists")]
    UniqueConstraintViolation(String),
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let code_object = match self {
            DbError::SomethingWentWrong(_) => get_common_code_object("CODE_UE_400"),
            DbError::UniqueConstraintViolation(_) => get_common_code_object("CODE_UCV_409"),
        };

        ApiErrorResponse::send(code_object)
    }
}