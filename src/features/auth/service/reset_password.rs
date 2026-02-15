use tracing::instrument;

use crate::{
    Result,
    common::hash_password,
    features::auth::{
        AuthService, EmailAddress, HashedPassword, Password, UpdateUser, service::TokenType,
    },
};

pub struct ResetPasswordInput {
    pub email: EmailAddress,
    pub new_password: Password,
    pub token: String,
}

impl AuthService {
    #[instrument(
        name = "auth.reset_password",
        skip(self, data),
        fields(email = %data.email)
    )]
    pub async fn reset_password(&self, data: ResetPasswordInput) -> Result<()> {
        let user = self
            .get_user_by_token(TokenType::ResetPassword, data.email, &data.token)
            .await?;

        let hashed_password = hash_password(data.new_password.as_ref())?;

        self.repository
            .update_user(
                &user.id,
                UpdateUser {
                    password: Some(HashedPassword::parse(hashed_password)?),
                    first_name: None,
                    last_name: None,
                    gender: None,
                    is_verified: None,
                    google_id: None,
                    facebook_id: None,
                },
            )
            .await?;

        Ok(())
    }
}
