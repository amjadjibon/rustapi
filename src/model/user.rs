use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use sqlx::FromRow;

#[derive(Clone, Deserialize, Serialize, FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub is_active: i16,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        first_name: Option<String>,
        last_name: Option<String>,
        user_name: String,
        email: String,
        password: String,
    ) -> Self {
        Self {
            id: 0,
            first_name,
            last_name,
            user_name,
            email,
            password,
            is_active: 1,
            updated_at: None,
            created_at: Utc::now(),
        }
    }
}

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
