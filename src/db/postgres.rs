use crate::conf::env;
use async_trait::async_trait;
use sqlx::{Error, Postgres, Pool};
use sqlx::postgres::PgPoolOptions;

pub struct Database {
    pool: Pool<Postgres>,
}


#[async_trait]
pub trait DatabaseTrait {
    async fn init() -> Result<Self, Error>
        where
            Self: Sized;
    fn get_pool(&self) -> &Pool<Postgres>;
}

#[async_trait]
impl DatabaseTrait for Database {
    async fn init() -> Result<Self, Error> {
        let database_url = env::get("DSN");
        let pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(5))
            .max_connections(10)
            .min_connections(1)
            .idle_timeout(std::time::Duration::from_secs(10*60))
            .max_lifetime(std::time::Duration::from_secs(30*60))
            .connect(&database_url).await?;
        Ok(Self { pool })
    }

    fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}