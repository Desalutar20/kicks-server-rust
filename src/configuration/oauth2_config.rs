use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct OAuth2Config {
    #[validate(length(min = 1))]
    pub google_client_id: String,
    #[validate(length(min = 1))]
    pub google_client_secret: String,
    #[validate(length(min = 1))]
    pub google_redirect_url: String,
    #[validate(length(min = 1))]
    pub facebook_client_id: String,
    #[validate(length(min = 1))]
    pub facebook_client_secret: String,
    #[validate(length(min = 1))]
    pub facebook_redirect_url: String,
}
