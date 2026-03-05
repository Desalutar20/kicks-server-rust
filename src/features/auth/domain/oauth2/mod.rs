mod facebook_oauth2;
mod google_oauth2;
mod oauth2_code;
mod oauth2_state;
mod provider_id;

use std::pin::Pin;

pub use facebook_oauth2::*;
pub use google_oauth2::*;
pub use oauth2_code::*;
pub use oauth2_state::*;
pub use provider_id::*;
use reqwest::Url;
use serde::Deserialize;

use crate::{Result, features::auth::EmailAddress};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuth2Provider {
    Facebook,
    Google,
}

#[derive(Debug)]
pub struct OAuth2User {
    pub provider_id: ProviderID,
    pub email: EmailAddress,
}

pub trait OAuth2Client: Send + Sync {
    fn provider(&self) -> OAuth2Provider;
    fn generate_redirect_url(&self, state: &OAuth2State) -> Result<Url>;

    fn get_user(
        &self,
        code: OAuth2Code,
    ) -> Pin<Box<dyn Future<Output = Result<OAuth2User>> + Send>>;
}
