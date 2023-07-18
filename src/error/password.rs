use axum::response::{IntoResponse, Response};
use thiserror::Error;
use crate::code::common::get_code_object;
use crate::response::api::ApiErrorResponse;


#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Invalid Password")]
    InvalidPassword(String),
    #[error("Something went wrong")]
    SomethingWentWrong(String),
}

impl IntoResponse for PasswordError {
    fn into_response(self) -> Response {
        let code_object = match self {
            PasswordError::InvalidPassword(error) => get_code_object("CODE_UE_400"),
            PasswordError::SomethingWentWrong(error) => get_code_object("CODE_UE_400"),
        };

        ApiErrorResponse::send(code_object)
    }
}