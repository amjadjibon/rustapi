use axum::http::StatusCode;
use axum::{
    response::{IntoResponse, Response}, Json,
};
use serde::{Deserialize, Serialize};

use crate::code::model::CodeObject;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ApiSuccessResponse<T: Serialize> {
    code: String,
    message: String,
    status: u16,
    data: T,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ApiErrorResponse {
    code: String,
    message: String,
    status: u16,
}

impl<T: Serialize> ApiSuccessResponse<T>
    where
        T: Serialize,
{
    pub(crate) fn send(code_object: CodeObject, data: T) -> Self {
        return ApiSuccessResponse {
            message: code_object.message,
            code: code_object.code,
            status: code_object.status,
            data,
        };
    }
}

impl<T: Serialize> IntoResponse for ApiSuccessResponse<T>
    where
        T: Serialize,
{
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        ).into_response()
    }
}

impl ApiErrorResponse {
    pub(crate) fn send(code_object: CodeObject) -> Response {
        return ApiErrorResponse {
            code: code_object.code,
            message: code_object.message,
            status: code_object.status,
        }.into_response();
    }
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        ).into_response()
    }
}