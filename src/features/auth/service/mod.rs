use std::{fmt::Display, sync::Arc};

use redis::{AsyncTypedCommands, aio::MultiplexedConnection};

use crate::{
    Error, Result,
    clients::email_client::EmailClient,
    configuration::app_config::ApplicationConfig,
    features::auth::{
        EmailAddress, REDIS_ACCOUNT_VERIFICATION_PREFIX, REDIS_RESET_PASSWORD_PREFIX,
        REDIS_SESSION_PREFIX, User, UserID, repository::AuthRepository,
    },
};

pub mod authenticate;
pub mod forgot_password;
pub mod logout;
pub mod reset_password;
pub mod sign_in;
pub mod sign_up;
pub mod verify_account;

#[derive(Debug, Clone)]
pub struct AuthService {
    app_config: ApplicationConfig,
    redis: MultiplexedConnection,
    email_client: Arc<EmailClient>,
    repository: AuthRepository,
}

#[derive(Debug)]
enum KeyType {
    Verification,
    ResetPassword,
    Session,
}

#[derive(Debug)]
enum TokenType {
    ResetPassword,
    Verification,
}

impl AuthService {
    pub fn new(
        app_config: ApplicationConfig,
        redis: MultiplexedConnection,
        email_client: Arc<EmailClient>,
        repository: AuthRepository,
    ) -> Self {
        Self {
            app_config,
            redis,
            email_client,
            repository,
        }
    }

    async fn get_user_by_token(
        &self,
        token_type: TokenType,
        email: EmailAddress,
        token: &str,
    ) -> Result<User> {
        let mut redis = self.redis.clone();

        let key_type = match token_type {
            TokenType::ResetPassword => KeyType::ResetPassword,
            TokenType::Verification => KeyType::Verification,
        };

        let user_id = redis
            .get_del(self.generate_redis_key(key_type, token))
            .await?
            .map(|id| UserID::parse(&id).map_err(|_| Error::Conflict("Invalid token".into())))
            .transpose()?
            .ok_or(Error::Conflict("Invalid token".into()))?;

        let user = self
            .repository
            .get_user_by_id(&user_id)
            .await?
            .filter(|u| {
                !u.is_banned
                    && u.email == email
                    && match token_type {
                        TokenType::ResetPassword => u.is_verified,
                        _ => true,
                    }
            })
            .ok_or(Error::Conflict("Invalid token".into()))?;

        Ok(user)
    }

    fn generate_redis_key<T: Display>(&self, key_type: KeyType, value: T) -> String {
        match key_type {
            KeyType::Verification => format!("{}{}", REDIS_ACCOUNT_VERIFICATION_PREFIX, value),
            KeyType::Session => format!("{}{}", REDIS_SESSION_PREFIX, value),
            KeyType::ResetPassword => format!("{}{}", REDIS_RESET_PASSWORD_PREFIX, value),
        }
    }
}
