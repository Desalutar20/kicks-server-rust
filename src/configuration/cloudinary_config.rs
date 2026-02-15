use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct CloudinaryConfig {
    #[validate(length(min = 1))]
    pub api_key: String,
    #[validate(length(min = 1))]
    pub secret: String,
    #[validate(length(min = 1))]
    pub cloud_name: String,
    #[validate(length(min = 1))]
    pub folder: String,
}
