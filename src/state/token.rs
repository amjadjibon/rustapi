use crate::db::postgres::Database;
use crate::repo::user::{UserRepository, UserRepositoryTrait};
use std::sync::Arc;
use crate::service::token::TokenService;

#[derive(Clone)]
pub struct TokenState {
    pub token_service: TokenService,
    pub user_repo: UserRepository,
}

impl TokenState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            token_service: TokenService::new(),
            user_repo: UserRepository::new(db_conn),
        }
    }
}