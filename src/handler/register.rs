use axum::{extract::State};
use tracing::info;

use crate::model::user::{UserReadDto, UserRegisterDto};
use crate::error::{api::ApiError, request::ValidatedRequest};
use crate::response::api::ApiSuccessResponse;
use crate::code::user::{CODE_AC_201, get_code_object};
use crate::state::user::UserState;


pub async fn register(
    State(state): State<UserState>,
    ValidatedRequest(payload): ValidatedRequest<UserRegisterDto>,
) -> Result<ApiSuccessResponse<UserReadDto>, ApiError> {
    info!("register user");
    let user = state.user_service.register(payload).await?;
    Ok(ApiSuccessResponse::send(
        get_code_object(CODE_AC_201),
        user,
    ))
}