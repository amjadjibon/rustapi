use crate::db::postgres::{Database, DatabaseTrait};
use crate::dto::user::{UserReadDto, UserRegisterDto};
// use crate::entity::user::User;
use crate::error::api::ApiError;
// use crate::error::db::DbError;
use crate::error::user::UserError;
use crate::repo::user::{UserRepository, UserRepositoryTrait};
use sqlx::Error as SqlxError;
use std::sync::Arc;
use crate::model::user::User;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    db_conn: Arc<Database>,
}

impl UserService {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn),
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn register(&self, payload: UserRegisterDto) -> Result<UserReadDto, ApiError> {
        return match self.user_repo.find_by_email(payload.email.to_owned()).await {
            Some(_) => Err(UserError::UserAlreadyExists)?,
            None => {
                let user = User::new(
                    payload.first_name,
                    payload.last_name,
                    payload.user_name,
                    payload.email,
                    payload.password,
                );
                Ok(UserReadDto::from(user))
            }
        }
    }
}