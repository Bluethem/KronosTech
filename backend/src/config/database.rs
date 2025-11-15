use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

#[derive(Clone)]
pub struct DatabaseConfig {
    pub pool: PgPool,
}

impl DatabaseConfig {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
