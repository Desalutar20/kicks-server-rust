use redis::AsyncTypedCommands;
use tracing::instrument;

use crate::{
    Result,
    features::auth::{AuthService, service::KeyType},
};

impl AuthService {
    #[instrument(name = "auth.logout", skip_all)]
    pub async fn logout(&self, session_id: &str) -> Result<()> {
        let mut redis = self.redis.clone();
        redis
            .del(self.generate_redis_key(KeyType::Session, session_id))
            .await?;

        Ok(())
    }
}
