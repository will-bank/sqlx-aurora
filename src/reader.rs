use std::ops::{Deref, DerefMut};

use crate::config::DbConfig;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, PgConnection, Pool, Postgres};

pub struct Reader(Pool<Postgres>);

impl Reader {
    pub async fn new(db_config: DbConfig) -> Result<Self, sqlx::Error> {
        let conn_string = db_config.connection_url();
        let mut conn = PgConnection::connect(&conn_string).await?;
        let sql = "show transaction_read_only";
        let transaction_read_only: String = sqlx::query_scalar(sql).fetch_one(&mut conn).await?;
        if transaction_read_only != "on" {
            return Err(sqlx::Error::Configuration(
                "The host is not a reader instance; please check your configurations".into(),
            ));
        }

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
impl Deref for Reader {
    type Target = Pool<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Reader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod reader_tests {

    use crate::config::{DbConfig, DbConfigBuilder};
    use crate::reader::Reader;
    use std::error::Error;

    #[tokio::test]
    async fn should_return_error_because_instance_is_not_a_reader() -> Result<(), Box<dyn Error>> {
        dotenv::from_filename(".writer.env").ok();
        let db_config = DbConfig::from_env();
        let reader = Reader::new(db_config.clone()).await;

        let err = reader.map_err(|e| e.to_string()).err().unwrap();
        assert_eq!(err, "error with configuration: The host is not a reader instance; please check your configurations");
        Ok(())
    }

    #[tokio::test]
    async fn should_return_data_when_instance_is_a_reader() -> Result<(), Box<dyn Error>> {
        dotenv::from_filename(".reader.env").ok();
        let db_config = DbConfig::from_env();
        let reader = Reader::new(db_config.clone()).await?;
        let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(reader.pool()).await?;
        assert_eq!(row.0, 1);
        Ok(())
    }

    #[tokio::test]
    async fn should_return_data_when_instance_is_a_reader_from_direct_config(
    ) -> Result<(), Box<dyn Error>> {
        let db_config = DbConfigBuilder::new()
            .host("localhost")
            .user("reader_user")
            .name("reader")
            .pass("password")
            .app_name("some-app-to-run")
            .port(5431)
            .min_pool_size(1)
            .max_pool_size(3)
            .idle_in_transaction_session(3000)
            .build()
            .expect("problem to create db config");
        let reader = Reader::new(db_config.clone()).await?;
        let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(reader.pool()).await?;
        assert_eq!(row.0, 1);
        Ok(())
    }

    #[tokio::test]
    async fn should_return_error_because_user_has_not_access() -> Result<(), Box<dyn Error>> {
        dotenv::from_filename(".reader.env").ok();
        let db_config = DbConfig::from_env();
        let reader = Reader::new(db_config.clone()).await?;
        let result = sqlx::query("CREATE TEMP TABLE temp_test (id INT)")
            .execute(reader.pool())
            .await;

        let is_read_only = match result {
            Ok(_) => false,
            Err(sqlx::Error::Database(db_err)) => {
                let cant_write_in_read_only_transaction = "25006";
                db_err.code().as_deref() == Some(cant_write_in_read_only_transaction)
            }
            Err(_) => false,
        };
        assert!(is_read_only);
        Ok(())
    }
}
