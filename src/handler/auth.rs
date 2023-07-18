use axum::extract::State;
use tracing::info;

use crate::code::auth::get_code_object;
use crate::error::api::ApiError;
use crate::error::request::ValidatedRequest;
use crate::error::user::UserError;
use crate::model::login::{UserLoginRefreshRequestDto, UserLoginRefreshResponseDto, UserLoginRequestDto, UserLoginResponseDto};
use crate::repo::user::UserRepositoryTrait;
use crate::response::api::ApiSuccessResponse;
use crate::state::token::TokenState;
use crate::utils::password::verify_password;

pub async fn login_handler(
    State(token_state): State<TokenState>,
    ValidatedRequest(payload): ValidatedRequest<UserLoginRequestDto>,
) -> Result<ApiSuccessResponse<UserLoginResponseDto>, ApiError> {
    info!("Login user");

    let user = token_state
        .user_repo
        .find_by_email(payload.email)
        .await.ok_or(UserError::UserNotFound)?;


    return match verify_password(&user.password, &payload.password) {
        true => {
            let token_res = token_state.token_service.login(user)?;
            Ok(ApiSuccessResponse::send(
                get_code_object("CODE_UAS_200"),
                token_res,
            ))
        }
        false => Err(UserError::InvalidPassword)?,
    };
}

pub async fn login_refresh_handler(
    State(token_state): State<TokenState>,
    ValidatedRequest(payload): ValidatedRequest<UserLoginRefreshRequestDto>,
) -> Result<ApiSuccessResponse<UserLoginRefreshResponseDto>, ApiError>  {
    let user = UserLoginRefreshResponseDto {
        access_token: "access_token".to_string(),
        refresh_token: "refresh_token".to_string(),
        token_type: "token_type".to_string(),
        expires_in: 3600,
    };

    Ok(ApiSuccessResponse::send(
        get_code_object("CODE_AC_201"),
        user,
    ))
}