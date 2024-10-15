use std::ops::{Deref, DerefMut};

use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use crate::config::DbConfig;

pub struct Writer(Pool<Postgres>);

impl Writer {
    pub async fn new(db_config: DbConfig) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .min_connections(db_config.min_pool_size)
            .max_connections(db_config.max_pool_size)
            .test_before_acquire(true)
            .connect(&db_config.connection_url())
            .await?;
        Ok(Self(pool))
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.0
    }
}

impl Deref for Writer {
    type Target = Pool<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Writer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod writer_tests {
    use std::error::Error;

    use crate::config::DbConfig;
    use crate::writer::Writer;

    #[tokio::test]
    async fn should_execute_command_in_writer_instance() -> Result<(), Box<dyn Error>> {
        dotenv::from_filename(".writer.env").ok();
        let db_config = DbConfig::from_env();
        let writer = Writer::new(db_config.clone()).await?;
        let row: (i32,) = sqlx::query_as("SELECT 1")
            .fetch_one(writer.pool())
            .await?;
        assert_eq!(row.0, 1);
        Ok(())
    }

    #[tokio::test]
    async fn should_create_table() -> Result<(), Box<dyn Error>> {
        dotenv::from_filename(".writer.env").ok();
        let db_config = DbConfig::from_env();
        let writer = Writer::new(db_config.clone()).await?;
        let _ = sqlx::query("CREATE TEMP TABLE temp_test (id INT)")
            .execute(writer.pool())
            .await;

        let _ = sqlx::query("insert into temp_test values (10)")
            .execute(writer.pool())
            .await;

        let row: (i32,) = sqlx::query_as("SELECT id from temp_test")
            .fetch_one(writer.pool())
            .await?;

        assert_eq!(row.0, 10);
        Ok(())
    }
}
