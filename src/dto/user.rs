use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::model::user::User;


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

#[derive(Clone, Serialize, Deserialize, Validate, Debug)]
pub struct UserReadDto {
    pub id: u64,
    pub user_name: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl From<User> for UserReadDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id as u64,
            user_name: user.user_name,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        }
    }
}