use sqlx::Error as SqlxError;
use std::sync::Arc;
use tracing::error;

use crate::db::postgres::Database;
use crate::model::user::{User, UserReadDto, UserRegisterDto};
use crate::error::api::ApiError;
use crate::error::user::UserError;
use crate::repo::user::{UserRepository, UserRepositoryTrait};
use crate::error::db::DbError;
use crate::utils::password::hash_password;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn),
        }
    }

    pub async fn register(&self, payload: UserRegisterDto) -> Result<UserReadDto, ApiError> {
        return match self.user_repo.find_by_email(payload.email.to_owned()).await {
            Some(_) => Err(UserError::UserAlreadyExists)?,
            None => {
                let password = match hash_password(payload.password) {
                    Ok(password) => password,
                    Err(e) => {
                        error!("Error hashing password: {}", e);
                        Err(UserError::UserCreationFailed)?
                    },
                };


                let user = User::new(
                    payload.first_name,
                    payload.last_name,
                    payload.user_name,
                    payload.email,
                    password,
                );

                let user = self.user_repo.create(user).await;

                return match user {
                    Ok(user) => Ok(UserReadDto::from(user)),
                    Err(e) => match e {
                        SqlxError::Database(e) => match e.code() {
                            Some(code) => {
                                if code == "23505" {
                                    Err(DbError::UniqueConstraintViolation(e.to_string()))?
                                } else {
                                    Err(DbError::SomethingWentWrong(e.to_string()))?
                                }
                            }
                            _ => Err(DbError::SomethingWentWrong(e.to_string()))?,
                        },
                        _ => Err(DbError::SomethingWentWrong(e.to_string()))?,
                    },
                };
            }
        };
    }
}
