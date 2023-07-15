use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserRegisterDto {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    #[validate(length(
    min = 3,
    max = 20,
    message = "Password must be between 3 and 20 characters"
    ))]
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(length(
    min = 3,
    max = 20,
    message = "Username must be between 3 and 20 characters"
    ))]
    pub user_name: String,
}