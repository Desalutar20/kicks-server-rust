use crate::{
    Result,
    features::{admin::users::service::AdminUsersService, auth::UserID},
};

impl AdminUsersService {
    pub async fn remove_user(&self, user_id: UserID) -> Result<()> {
        self.repository.remove_user(user_id).await
    }
}
