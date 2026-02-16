use tracing::instrument;

use crate::{
    Error, Result,
    common::verify,
    features::{
        auth::{
            domain::{EmailAddress, Password},
            service::AuthService,
        },
        shared::AppUser,
    },
};

pub struct SignInInput {
    pub email: EmailAddress,
    pub password: Password,
}

impl AuthService {
    #[instrument(
        name = "auth.sign_in",
        skip(self, data),
        fields(email = %data.email)
    )]
    pub async fn sign_in(&self, data: SignInInput) -> Result<(AppUser, String)> {
        let user = self
            .repository
            .get_user_by_email(&data.email)
            .await?
            .filter(|u| !u.is_banned && u.is_verified)
            .ok_or(Error::Conflict("Invalid credentials".into()))?;

        let stored_password = user
            .password
            .as_ref()
            .ok_or(Error::Conflict("Invalid credentials".into()))?;

        if !verify(data.password.as_ref(), stored_password.as_ref())? {
            return Err(Error::Conflict("Invalid credentials".into()));
        }

        let session_id = self.generate_session(&user.id).await?;

        Ok((user.into(), session_id))
    }
}
