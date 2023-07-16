use crate::db::postgres::{Database, DatabaseTrait};
use crate::model::user::User;
use async_trait::async_trait;
use sqlx;
use sqlx::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserRepository {
    pub(crate) db_conn: Arc<Database>,
}

#[async_trait]
pub trait UserRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;
    async fn create(&self, user: User) -> Result<User, Error>;
    async fn find_by_email(&self, email: String) -> Option<User>;
    async fn find(&self, id: u64) -> Result<User, Error>;
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    async fn create(&self, user: User) -> Result<User, Error> {
        let insert = sqlx::query_as::<_, User>(
            "INSERT INTO \"user\" (first_name, last_name, user_name, email, password, is_active) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
            .bind(user.first_name)
            .bind(user.last_name)
            .bind(user.user_name)
            .bind(user.email)
            .bind(user.password)
            .bind(user.is_active)
            .fetch_one(self.db_conn.get_pool())
            .await?;

        return Ok(insert);
    }

    async fn find_by_email(&self, email: String) -> Option<User> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM user WHERE email = ?"
        )
            .bind(email)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None);
        return user;
    }

    async fn find(&self, _id: u64) -> Result<User, Error> {
        unimplemented!("find user by id")
    }
}