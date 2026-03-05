use reqwest::Url;

use crate::{
    Error, Result,
    features::auth::{AuthService, NewUser, OAuth2Code, OAuth2Provider, OAuth2State, UpdateUser},
};

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

        let client = self
            .providers
            .get(&provider)
            .ok_or(Error::Conflict("Provider not supported".into()))?;

        Ok((client.generate_redirect_url(&state)?, state))
    }

    pub async fn oauth2_sign_in(
        &self,
        provider: OAuth2Provider,
        data: OAuth2SignInInput,
    ) -> Result<(String, Option<String>)> {
        if data.state != data.cookie_state {
            return Err(Error::Conflict("Something went wrong".into()));
        }
        let redirect_path = data.state.into_inner().1;

        let client = self
            .providers
            .get(&provider)
            .ok_or(Error::Conflict("Provider not supported".into()))?;

        let oauth2_user = client.get_user(data.code).await?;
        let db_user = self
            .repository
            .get_user_by_email(&oauth2_user.email)
            .await?;

        let provider = client.provider();

        match db_user {
            None => {
                let mut new_user = NewUser {
                    email: oauth2_user.email,
                    google_id: None,
                    facebook_id: None,
                    first_name: None,
                    last_name: None,
                    gender: None,
                    hashed_password: None,
                    is_verified: true,
                };

                match provider {
                    OAuth2Provider::Google => new_user.google_id = Some(oauth2_user.provider_id),
                    OAuth2Provider::Facebook => {
                        new_user.facebook_id = Some(oauth2_user.provider_id)
                    }
                }

                let user_id = self.repository.create_user(&new_user).await?;
                let session_id = self.generate_session(&user_id).await?;

                Ok((session_id, redirect_path))
            }
            Some(db_user) => {
                let mut update = UpdateUser {
                    google_id: None,
                    facebook_id: None,
                    first_name: None,
                    last_name: None,
                    gender: None,
                    is_verified: Some(true),
                    password: None,
                };

                match provider {
                    OAuth2Provider::Google => {
                        if db_user.google_id.is_none() {
                            update.google_id = Some(oauth2_user.provider_id.clone());
                        }
                    }
                    OAuth2Provider::Facebook => {
                        if db_user.facebook_id.is_none() {
                            update.facebook_id = Some(oauth2_user.provider_id.clone());
                        }
                    }
                }

                if update.google_id.is_some() || update.facebook_id.is_some() {
                    self.repository.update_user(&db_user.id, update).await?;
                }

                let session_id = self.generate_session(&db_user.id).await?;
                Ok((session_id, redirect_path))
            }
        }
    }
}
