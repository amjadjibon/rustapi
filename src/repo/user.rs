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
        unimplemented!("create user")
    }

    async fn find_by_email(&self, email: String) -> Option<User> {
        unimplemented!("find user by email")
    }

    async fn find(&self, id: u64) -> Result<User, Error> {
        unimplemented!("find user by id")
    }
}