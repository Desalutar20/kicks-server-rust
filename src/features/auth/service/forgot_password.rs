use redis::AsyncTypedCommands;
use tokio::join;
use tracing::instrument;

use crate::{
    Result,
    common::generate_secure_random_string,
    features::auth::{AuthService, EmailAddress},
};

pub struct ForgotPasswordInput {
    pub email: EmailAddress,
}

impl AuthService {
    #[instrument(
        name = "auth.forgot_password",
        skip(self, data),
        fields(email = %data.email)
    )]
    pub async fn forgot_password(&self, data: ForgotPasswordInput) -> Result<()> {
        if let Some(user) = self
            .repository
            .get_user_by_email(&data.email)
            .await?
            .filter(|u| !u.is_banned && u.is_verified)
        {
            let token = generate_secure_random_string(42);
            let mut redis = self.redis.clone();

            let (redis_result, email_result) = join!(
                redis.set_ex(
                    self.generate_redis_key(super::KeyType::ResetPassword, &token),
                    user.id.to_string(),
                    self.app_config.reset_password_ttl_minutes * 60
                ),
                self.email_client
                    .send_account_verification_email(user.email.as_ref(), &token)
            );

            redis_result?;
            email_result?;

            Ok(())
        } else {
            Ok(())
        }
    }
}
