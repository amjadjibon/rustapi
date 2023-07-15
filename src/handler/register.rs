use axum::{extract::State, Json};
use tracing::info;

use crate::dto::user::{ UserRegisterDto};
use crate::error::{api::ApiError, request::ValidatedRequest};
use crate::response::api::ApiSuccessResponse;
use crate::code::user::get_user_code_object;


pub async fn register(
    ValidatedRequest(payload): ValidatedRequest<UserRegisterDto>,
) -> ApiSuccessResponse<UserRegisterDto> {
    info!("Registering user: {:?}", payload);
    ApiSuccessResponse::send(
        get_user_code_object("CODE_AC_201"),
        payload,
    )
}