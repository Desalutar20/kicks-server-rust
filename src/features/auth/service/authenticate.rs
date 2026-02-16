use redis::AsyncTypedCommands;

use crate::{
    Error, Result,
    features::{
        auth::{AuthService, UserID},
        shared::AppUser,
    },
};

impl AuthService {
    pub async fn authenticate(&self, session_id: &str) -> Result<AppUser> {
        let mut redis = self.redis.clone();

        let user_id = redis
            .get_ex(
                self.generate_redis_key(super::KeyType::Session, session_id),
                redis::Expiry::EX(self.app_config.session_ttl_minutes * 60),
            )
            .await?
            .map(|id| UserID::parse(&id).map_err(|_| Error::Unauthorized))
            .transpose()?
            .ok_or(Error::Unauthorized)?;

        let user = self
            .repository
            .get_user_by_id(&user_id)
            .await?
            .filter(|u| !u.is_banned && u.is_verified)
            .ok_or(Error::Unauthorized)?;

        Ok(user.into())
    }
}
