use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct DatabaseConfig {
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,
    #[validate(length(min = 1))]
    pub host: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub user: String,
    #[validate(length(min = 1))]
    pub password: String,
    pub ssl: bool,
}

impl DatabaseConfig {
    pub fn connect_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.user)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
            .database(&self.name)
    }
}
