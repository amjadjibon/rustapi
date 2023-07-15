use axum::{extract::State, Json};
use tracing::info;

use crate::dto::user::{UserReadDto, UserRegisterDto};
use crate::error::{api::ApiError, request::ValidatedRequest};
use crate::response::api::ApiSuccessResponse;
use crate::code::user::get_user_code_object;
use crate::state::user::UserState;


pub async fn register(
    State(state): State<UserState>,
    ValidatedRequest(payload): ValidatedRequest<UserRegisterDto>,
) -> Result<ApiSuccessResponse<UserReadDto>, ApiError> {
    let user = state.user_service.register(payload).await?;
    Ok(ApiSuccessResponse::send(
        get_user_code_object("CODE_AC_201"),
        user,
    ))
}