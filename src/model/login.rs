use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UserLoginRequestDto {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    #[validate(length(
    min = 3,
    max = 20,
    message = "Password must be between 3 and 20 characters"
    ))]
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserLoginResponseDto {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UserLoginRefreshRequestDto {
    pub refresh_token: String,
}

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserLoginRefreshResponseDto {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

