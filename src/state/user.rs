use crate::db::postgres::Database;
use crate::repo::user::{UserRepository, UserRepositoryTrait};
use crate::service::user::UserService;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserState {
    pub user_service: UserService,
    pub user_repo: UserRepository,
}

impl UserState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_service: UserService::new(db_conn),
            user_repo: UserRepository::new(db_conn),
        }
    }
}