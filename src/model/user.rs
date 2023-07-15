use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub is_active: i8,
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