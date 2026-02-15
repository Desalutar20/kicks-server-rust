use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct RedisConfig {
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,
    #[validate(length(min = 1))]
    pub host: String,
    pub user: String,
    pub password: String,
    #[validate(range(min = 0, max = 15))]
    pub database: u8,
}

impl RedisConfig {
    pub fn connection_string(&self) -> String {
        if self.password.is_empty() {
            format!("redis://{}:{}/{}", self.host, self.port, self.database)
        } else {
            format!(
                "redis://{}:{}@{}:{}/{}",
                self.user, self.password, self.host, self.port, self.database
            )
        }
    }
}
