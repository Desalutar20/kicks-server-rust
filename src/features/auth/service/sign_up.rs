use redis::AsyncTypedCommands;
use tokio::join;
use tracing::instrument;

use crate::{
    Error, Result,
    common::{generate_secure_random_string, hash_password},
    features::{
        auth::{
            EmailAddress, FirstName, HashedPassword, LastName, UserGender,
            domain::{NewUser, Password},
            service::{AuthService, KeyType},
        },
        shared::map_unique_violation,
    },
};

pub struct SignUpInput {
    pub email: EmailAddress,
    pub password: Password,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub gender: Option<UserGender>,
}

impl AuthService {
    #[instrument(
        name = "auth.sign_up",
        skip(self, data),
        fields(email = %data.email)
    )]
    pub async fn sign_up(&self, data: SignUpInput) -> Result<()> {
        let hashed_password = hash_password(data.password.as_ref())?;

        let new_user = NewUser {
            email: data.email,
            hashed_password: HashedPassword::parse(hashed_password)?,
            first_name: data.first_name,
            last_name: data.last_name,
            gender: data.gender,
        };

        let new_user_id =
            self.repository
                .create_user(&new_user)
                .await
                .map_err(map_unique_violation(Some(Error::Conflict(
                    "An account with this email already exists".into(),
                ))))?;

        let token = generate_secure_random_string(42);
        let mut redis = self.redis.clone();

        let (redis_result, email_result) = join!(
            redis.set_ex(
                self.generate_redis_key(KeyType::Verification, &token),
                new_user_id.to_string(),
                self.app_config.account_verification_ttl_minutes * 60,
            ),
            self.email_client
                .send_account_verification_email(new_user.email.as_ref(), &token)
        );

        redis_result?;
        email_result?;

        Ok(())
    }
}
