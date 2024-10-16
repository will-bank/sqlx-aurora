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

pub struct DbConfigBuilder {
    host: Option<String>,
    name: Option<String>,
    user: Option<String>,
    pass: Option<String>,
    port: Option<u16>,
    min_pool_size: Option<u32>,
    max_pool_size: Option<u32>,
    idle_in_transaction_session: Option<u32>,
    app_name: Option<String>,
}

impl DbConfigBuilder {
    pub fn new() -> Self {
        Self {
            host: None,
            name: None,
            user: None,
            pass: None,
            port: None,
            min_pool_size: None,
            max_pool_size: None,
            idle_in_transaction_session: None,
            app_name: None,
        }
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = Some(host.to_string());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_string());
        self
    }

    pub fn pass(mut self, pass: &str) -> Self {
        self.pass = Some(pass.to_string());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn min_pool_size(mut self, min_pool_size: u32) -> Self {
        self.min_pool_size = Some(min_pool_size);
        self
    }

    pub fn max_pool_size(mut self, max_pool_size: u32) -> Self {
        self.max_pool_size = Some(max_pool_size);
        self
    }

    pub fn idle_in_transaction_session(mut self, idle: u32) -> Self {
        self.idle_in_transaction_session = Some(idle);
        self
    }

    pub fn app_name(mut self, app_name: &str) -> Self {
        self.app_name = Some(app_name.to_string());
        self
    }

    pub fn build(self) -> Result<DbConfig, &'static str> {
        Ok(DbConfig {
            host: self.host.ok_or("host is required")?,
            name: self.name.ok_or("name is required")?,
            user: self.user.ok_or("user is required")?,
            pass: self.pass.ok_or("pass is required")?,
            port: self.port.ok_or("port is required")?,
            min_pool_size: self.min_pool_size.ok_or("min_pool_size is required")?,
            max_pool_size: self.max_pool_size.ok_or("max_pool_size is required")?,
            idle_in_transaction_session: self
                .idle_in_transaction_session
                .ok_or("idle_in_transaction_session is required")?,
            app_name: self.app_name.ok_or("app_name is required")?,
        })
    }
}

impl DbConfig {
    pub fn from_env() -> Self {
        let db_host = env::var("DB_HOST").expect("DB_HOST not present");
        let db_name = env::var("DB_NAME").expect("DB_NAME not present");
        let db_user = env::var("DB_USER").expect("DB_USER not present");
        let db_pass = env::var("DB_PASS").expect("DB_PASS not present");
        let db_port = env::var("DB_PORT")
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
