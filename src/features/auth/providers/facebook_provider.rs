use std::pin::Pin;

use reqwest::{Client, Url};

use crate::{
    Error, Result,
    features::auth::{
        FacebookAccessTokenResponse, FacebookUserResponse, OAuth2Client, OAuth2Code,
        OAuth2Provider, OAuth2State, OAuth2User,
    },
};

pub struct FacebookProvider {
    pub client_secret: String,
    pub client_id: String,
    pub redirect_url: String,
    pub http_client: Client,
}

impl OAuth2Client for FacebookProvider {
    fn generate_redirect_url(&self, state: &OAuth2State) -> Result<Url> {
        let url = Url::parse_with_params(
            "https://www.facebook.com/v20.0/dialog/oauth",
            &[
                ("client_id", self.client_id.as_str()),
                ("redirect_uri", self.redirect_url.as_str()),
                ("response_type", "code"),
                ("scope", "email"),
                ("state", &state.to_string()),
            ],
        )
        .map_err(|e| {
            Error::Internal(format!("Failed to generate facebook redirect url: {:?}", e))
        })?;

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
            let url = Url::parse_with_params(
                "https://graph.facebook.com/v20.0/oauth/access_token",
                &[
                    ("code", code.as_ref()),
                    ("client_id", client_id.as_str()),
                    ("client_secret", client_secret.as_str()),
                    ("redirect_uri", redirect_url.as_str()),
                ],
            )
            .map_err(|e| Error::Internal(format!("Failed to parse facebook url {e:?}")))?;

            let token_response = http_client
                .get(url)
                .send()
                .await?
                .json::<FacebookAccessTokenResponse>()
                .await
                .map_err(|e| Error::Internal(format!("Failed to get facebook user: {:?}", e)))?;

            match token_response {
                FacebookAccessTokenResponse::Error(err) => Err(Error::Internal(format!(
                    "Faceebook returned an error: {}",
                    err.error.message
                ))),
                FacebookAccessTokenResponse::Success(success) => {
                    let url = Url::parse_with_params(
                        "https://graph.facebook.com/v20.0/me",
                        &[
                            ("fields", "id,first_name,last_name,gender,email"),
                            ("access_token", success.access_token.as_ref()),
                        ],
                    )
                    .map_err(|e| Error::Internal(format!("Failed to parse the Facebook API URL with the provided parameters. Error: {e:?}")))?;

                    let facebook_user = http_client
                        .get(url)
                        .send()
                        .await?
                        .json::<FacebookUserResponse>()
                        .await
                        .map_err(|e| {
                            Error::Internal(format!(
                                "Failed to deserialize Facebook user response: {:?}",
                                e
                            ))
                        })?;

                    Ok(OAuth2User {
                        provider_id: facebook_user.id,
                        email: facebook_user.email,
                    })
                }
            }
        })
    }

    fn provider(&self) -> OAuth2Provider {
        OAuth2Provider::Facebook
    }
}
