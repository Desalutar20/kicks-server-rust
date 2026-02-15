use tracing::instrument;

use crate::{
    Result,
    features::auth::{
        domain::{EmailAddress, UpdateUser},
        service::AuthService,
    },
};

pub struct VerifyAccountInput {
    pub token: String,
    pub email: EmailAddress,
}

impl AuthService {
    #[instrument(
        name = "auth.verify_account",
        skip(self, data),
        fields(email = %data.email)
    )]
    pub async fn verify_account(&self, data: VerifyAccountInput) -> Result<()> {
        let user = self
            .get_user_by_token(super::TokenType::Verification, data.email, &data.token)
            .await?;

        self.repository
            .update_user(
                &user.id,
                UpdateUser {
                    first_name: None,
                    last_name: None,
                    gender: None,
                    password: None,
                    google_id: None,
                    facebook_id: None,
                    is_verified: Some(true),
                },
            )
            .await?;

        Ok(())
    }
}
