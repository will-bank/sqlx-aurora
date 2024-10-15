use std::env;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub host: String,
    pub name: String,
    pub user: String,
    pub pass: String,
    pub port: u16,
    pub min_pool_size: u32,
    pub max_pool_size: u32,
    pub idle_in_transaction_session: u32,
    pub app_name: String,
}

impl DbConfig {

    pub fn from_env() -> Self {
        let db_host = env::var("DB_HOST").expect("DB_HOST not present");
        let db_name = env::var("DB_NAME").expect("DB_NAME not present");
        let db_user = env::var("DB_USER").expect("DB_USER not present");
        let db_pass = env::var("DB_PASS").expect("DB_PASS not present");
        let db_port   = env::var("DB_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(5432);
        let db_min_pool_size = env::var("DB_MIN_POOL_SIZE")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(1);
        let db_max_pool_size = env::var("DB_MAX_POOL_SIZE")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(1);
        let db_app_name = env::var("DB_APP_NAME")
            .ok()
            .unwrap_or("app-name-not-defined".to_string());

        let idle_in_transaction_session = env::var("DB_IDLE_IN_TRANSACTION")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(2000);

        Self {
            host: db_host,
            name: db_name,
            user: db_user,
            pass: db_pass,
            port: db_port,
            min_pool_size: db_min_pool_size,
            max_pool_size: db_max_pool_size,
            idle_in_transaction_session,
            app_name: db_app_name,
        }

    }

    pub fn new(host: String, name: String, user: String, pass: String, app_name: String, port: Option<u16>, min_pool_size: Option<u32>, max_pool_size: Option<u32>, idle_in_transaciton: Option<u32>) -> Self {
        let db_port   = port.unwrap_or(5432);
        let db_min_pool_size = min_pool_size.unwrap_or(1);
        let db_max_pool_size = max_pool_size.unwrap_or(2);
        let idle_in_transaction_session = idle_in_transaciton.unwrap_or(2000);

        Self {
            host,
            name,
            user,
            pass,
            port: db_port,
            min_pool_size: db_min_pool_size,
            max_pool_size: db_max_pool_size,
            idle_in_transaction_session,
            app_name,
        }
    }


    pub fn connection_url(&self) -> String {
        format!(
            "postgresql://{db_host}:{db_port}?dbname={db_name}&user={db_user}&password={db_pass}&application_name={db_app_name}&options=-c%20idle_in_transaction_session_timeout%3D{idle_in_transaction_session}",
            db_host = self.host,
            db_name = self.name,
            db_user = self.user,
            db_pass = self.pass,
            db_port = self.port,
            db_app_name = self.app_name,
            idle_in_transaction_session = self.idle_in_transaction_session
        )
    }

}