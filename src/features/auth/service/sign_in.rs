use redis::AsyncTypedCommands;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    Error, Result,
    common::verify,
    features::{
        auth::{
            domain::{EmailAddress, Password},
            service::{AuthService, KeyType},
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

        let session_id = Uuid::new_v4().to_string();
        let mut redis = self.redis.clone();

        redis
            .set_ex(
                self.generate_redis_key(KeyType::Session, &session_id),
                user.id.to_string(),
                self.app_config.session_ttl_minutes * 60,
            )
            .await?;

        Ok((user.into(), session_id))
    }
}
