use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Trace,
}

impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
            LogLevel::Trace => "trace",
        }
    }
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct ApplicationConfig {
    #[validate(length(min = 1))]
    pub host: String,
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,
    #[validate(length(min = 1), url)]
    pub client_url: String,
    #[validate(length(min = 1))]
    pub account_verification_path: String,
    #[validate(length(min = 1))]
    pub reset_password_path: String,
    #[validate(length(min = 1))]
    pub session_cookie_name: String,
    pub cookie_secure: bool,
    #[validate(length(min = 40))]
    pub cookie_secret: String,
    #[validate(range(min = 60, max = 1440))]
    pub account_verification_ttl_minutes: u64,
    #[validate(range(min = 1440, max = 43200))]
    pub session_ttl_minutes: u64,
    #[validate(range(min = 5, max = 10))]
    pub reset_password_ttl_minutes: u64,
    pub log_level: LogLevel,
    pub pretty_log: bool,
}
