use crate::{
    Result,
    features::{admin::users::service::AdminUsersService, auth::UserID},
};

impl AdminUsersService {
    pub async fn toggle_user_is_bannned(&self, user_id: UserID) -> Result<()> {
        self.repository.toggle_user_is_banned(user_id).await
    }
}
