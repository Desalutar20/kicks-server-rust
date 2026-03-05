use std::future::Future;
use std::pin::Pin;

use reqwest::{Client, Url};

use crate::{
    Error, Result,
    features::auth::{
        GoogleAccessTokenError, GoogleAccessTokenResponse, GoogleAccessTokenSuccess,
        GoogleUserResponse, OAuth2Client, OAuth2Code, OAuth2Provider, OAuth2State, OAuth2User,
    },
};

pub struct GoogleProvider {
    pub client_secret: String,
    pub client_id: String,
    pub redirect_url: String,
    pub http_client: Client,
}

impl OAuth2Client for GoogleProvider {
    fn provider(&self) -> OAuth2Provider {
        OAuth2Provider::Google
    }

    fn generate_redirect_url(&self, state: &OAuth2State) -> Result<Url> {
        let url = Url::parse_with_params(
            "https://accounts.google.com/o/oauth2/v2/auth",
            &[
                ("client_id", self.client_id.as_str()),
                ("redirect_uri", self.redirect_url.as_str()),
                ("response_type", "code"),
                ("scope", "openid email profile"),
                ("access_type", "offline"),
                ("state", &state.to_string()),
            ],
        )
        .map_err(|e| Error::Internal(format!("Failed to generate google redirect url: {:?}", e)))?;

        Ok(url)
    }

    fn get_user(
        &self,
        code: OAuth2Code,
    ) -> Pin<Box<dyn Future<Output = Result<OAuth2User>> + Send>> {
        let client_id = self.client_id.clone();
        let client_secret = self.client_secret.clone();
        let redirect_url = self.redirect_url.clone();
        let http_client = self.http_client.clone();

        Box::pin(async move {
            let params = [
                ("code", code.as_ref()),
                ("client_id", client_id.as_str()),
                ("client_secret", client_secret.as_str()),
                ("redirect_uri", redirect_url.as_str()),
                ("grant_type", "authorization_code"),
            ];

            let token_response = http_client
                .post("https://oauth2.googleapis.com/token")
                .form(&params)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .send()
                .await?
                .json::<GoogleAccessTokenResponse>()
                .await
                .map_err(|e| Error::Internal(format!("Failed to get google user: {:?}", e)))?;

            match token_response {
                GoogleAccessTokenResponse::Error(GoogleAccessTokenError {
                    error,
                    error_description,
                }) => Err(Error::Internal(format!(
                    "Google returned an error: {} – {}",
                    error, error_description
                ))),

                GoogleAccessTokenResponse::Success(GoogleAccessTokenSuccess {
                    access_token,
                    ..
                }) => {
                    let user = http_client
                        .post("https://openidconnect.googleapis.com/v1/userinfo")
                        .header("Authorization", format!("Bearer {}", access_token))
                        .send()
                        .await?
                        .json::<GoogleUserResponse>()
                        .await
                        .map_err(|e| {
                            Error::Internal(format!(
                                "Failed to deserialize Google user response: {:?}",
                                e
                            ))
                        })?;

                    if !user.email_verified {
                        return Err(Error::Conflict("Email is not verified".into()));
                    }

                    Ok(OAuth2User {
                        provider_id: user.sub,
                        email: user.email,
                    })
                }
            }
        })
    }
}
