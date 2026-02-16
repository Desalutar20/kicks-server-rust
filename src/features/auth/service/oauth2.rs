use reqwest::Url;

use crate::{
    Error, Result,
    features::auth::{
        AuthService, GoogleAccessTokenError, GoogleAccessTokenResponse, GoogleAccessTokenSuccess,
        GoogleUserResponse, NewUser, OAuth2Code, OAuth2State, UpdateUser,
    },
};

pub enum OAuth2Provider {
    // Facebook,
    Google,
}

pub struct OAuth2SignInInput {
    pub state: OAuth2State,
    pub cookie_state: OAuth2State,
    pub code: OAuth2Code,
}

impl AuthService {
    pub fn generate_oauth2_redirect_url(
        &self,
        provider: OAuth2Provider,
        redirect_path: Option<String>,
    ) -> Result<(Url, OAuth2State)> {
        let state = if let Some(path) = redirect_path {
            OAuth2State::from(path.as_str())
        } else {
            OAuth2State::default()
        };

        match provider {
            OAuth2Provider::Google => Ok((self.generate_google_redirect_url(&state)?, state)),
        }
    }

    pub async fn oauth2_sign_in(
        &self,
        provider: OAuth2Provider,
        data: OAuth2SignInInput,
    ) -> Result<(String, Option<String>)> {
        if data.state != data.cookie_state {
            return Err(Error::Conflict("Something went wrong".into()));
        }
        let (_, redirect_path) = data.state.into_inner();

        match provider {
            OAuth2Provider::Google => {
                let session_id = self.google_sign_in(data.code).await?;

                Ok((session_id, redirect_path))
            }
        }
    }

    async fn google_sign_in(&self, code: OAuth2Code) -> Result<String> {
        let google_user = self.get_google_user(code).await?;
        let db_user = self
            .repository
            .get_user_by_email(&google_user.email)
            .await?;

        match db_user {
            None => {
                let new_user = NewUser {
                    email: google_user.email,
                    google_id: Some(google_user.sub),
                    facebook_id: None,
                    first_name: None,
                    last_name: None,
                    gender: None,
                    hashed_password: None,
                    is_verified: true,
                };
                let user_id = self.repository.create_user(&new_user).await?;

                self.generate_session(&user_id).await
            }
            Some(db_user) => {
                if db_user.google_id.is_none() {
                    self.repository
                        .update_user(
                            &db_user.id,
                            UpdateUser {
                                google_id: Some(google_user.sub),
                                facebook_id: None,
                                first_name: None,
                                last_name: None,
                                gender: None,
                                is_verified: Some(true),
                                password: None,
                            },
                        )
                        .await?;
                }

                self.generate_session(&db_user.id).await
            }
        }
    }

    async fn get_google_user(&self, code: OAuth2Code) -> Result<GoogleUserResponse> {
        let params = [
            ("code", code.as_ref()),
            ("client_id", self.oauth2_config.google_client_id.as_str()),
            (
                "client_secret",
                self.oauth2_config.google_client_secret.as_str(),
            ),
            (
                "redirect_uri",
                self.oauth2_config.google_redirect_url.as_str(),
            ),
            ("grant_type", "authorization_code"),
        ];

        let token_response = self
            .http_client
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
                access_token, ..
            }) => {
                let user = self
                    .http_client
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

                Ok(user)
            }
        }
    }

    fn generate_google_redirect_url(&self, state: &OAuth2State) -> Result<Url> {
        let url = Url::parse_with_params(
            "https://accounts.google.com/o/oauth2/v2/auth",
            &[
                ("client_id", self.oauth2_config.google_client_id.as_str()),
                (
                    "redirect_uri",
                    self.oauth2_config.google_redirect_url.as_str(),
                ),
                ("response_type", "code"),
                ("scope", "openid email profile"),
                ("access_type", "offline"),
                ("state", &state.to_string()),
            ],
        )
        .map_err(|e| Error::Internal(format!("Failed to generate google redirect url: {:?}", e)))?;

        Ok(url)
    }
}
