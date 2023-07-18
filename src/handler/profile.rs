use axum::Extension;
use axum::extract::State;
use jsonwebtoken::TokenData;
use tracing::info;
use crate::code::common::get_code_object;
use crate::error::api::ApiError;
use crate::error::user::UserError;
use crate::model::token::Claims;
use crate::model::user::{User, UserReadDto};
use crate::repo::user::UserRepositoryTrait;
use crate::response::api::ApiSuccessResponse;
use crate::state::user::UserState;

pub async fn profile_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<UserState>,
) -> Result<ApiSuccessResponse<UserReadDto>, ApiError> {

    info!("current_user: {:?}", claims);

    let user = state.user_repo.find(claims.user_id).await.ok_or(UserError::UserNotFound)?;
    Ok(ApiSuccessResponse::send(
        get_code_object("CODE_RS_200"),
        UserReadDto::from(user),
    ))
}