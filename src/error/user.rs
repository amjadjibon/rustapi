use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use crate::code::user::get_user_code_object;
use crate::response::api::ApiErrorResponse;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid password")]
    InvalidPassword,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let code_object = match self {
            UserError::UserNotFound => get_user_code_object("CODE_UNF_400"),
            UserError::UserAlreadyExists => get_user_code_object("CODE_UAE_400"),
            UserError::InvalidPassword => get_user_code_object("CODE_IP_400"),
        };

        ApiErrorResponse::send(code_object)
    }
}