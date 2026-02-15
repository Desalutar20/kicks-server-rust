use std::{env, path::Path};

pub mod app_config;
pub mod cloudinary_config;
pub mod database_config;
pub mod ratelimit_config;
pub mod redis_config;
pub mod smtp_config;

use config::Config;
use serde::Deserialize;
use validator::Validate;

use crate::configuration::{
    app_config::ApplicationConfig, cloudinary_config::CloudinaryConfig,
    database_config::DatabaseConfig, ratelimit_config::RateLimitConfig, redis_config::RedisConfig,
    smtp_config::SmtpConfig,
};

#[derive(Validate, Deserialize, Debug)]
pub struct Configuration {
    #[validate(nested)]
    pub application: ApplicationConfig,
    #[validate(nested)]
    pub database: DatabaseConfig,
    #[validate(nested)]
    pub redis: RedisConfig,
    #[validate(nested)]
    pub smtp: SmtpConfig,
    #[validate(nested)]
    pub cloudinary: CloudinaryConfig,
    #[validate(nested)]
    pub ratelimit: RateLimitConfig,
}

impl Configuration {
    // #[allow(warnings)] is used here to ignore warnings like missing `Default`.
    #[allow(warnings)]
    pub fn new() -> Configuration {
        let environment = env::var("APPLICATION__ENV").unwrap_or("development".into());
        let project_root = env!("CARGO_MANIFEST_DIR");

        let file_name = Path::new(project_root)
            .join("configs")
            .join(format!("{}.yaml", environment));

        let config = Config::builder()
            .add_source(config::File::with_name(&file_name.to_str().unwrap()))
            .add_source(config::Environment::default().separator("__"))
            .build()
            .expect("Failed to read configuration.");

        let config = config.try_deserialize::<Configuration>().unwrap();

        if let Err(err) = config.validate() {
            for (kind, error) in err.errors() {
                eprintln!("{kind} {error:?}")
            }

            panic!("config validation failed")
        }

        config
    }
}
