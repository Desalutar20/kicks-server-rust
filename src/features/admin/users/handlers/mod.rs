mod get_all_users_handler;
mod remove_user_handler;
mod toggle_user_is_banned_handler;

use time::serde::rfc3339;

pub use get_all_users_handler::get_all_users_handler_v1;
pub use remove_user_handler::remove_user_handler_v1;
pub use toggle_user_is_banned_handler::toggle_user_is_banned_handler_v1;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::features::{
    admin::users::service::get_all_users::AdminUser,
    auth::{UserGender, UserRole},
};

#[derive(Serialize, Deserialize)]
pub struct AdminUserResponse {
    pub id: String,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub updated_at: OffsetDateTime,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: UserRole,
    pub gender: Option<UserGender>,
    pub is_verified: bool,
    pub is_banned: bool,
    pub google_id: Option<String>,
    pub facebook_id: Option<String>,
}

impl From<AdminUser> for AdminUserResponse {
    fn from(value: AdminUser) -> Self {
        Self {
            id: value.id.to_string(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            email: value.email.into_inner(),
            first_name: value.first_name.map(|v| v.into_inner()),
            last_name: value.last_name.map(|v| v.into_inner()),
            role: value.role,
            gender: value.gender,
            is_verified: value.is_verified,
            is_banned: value.is_banned,
            google_id: value.google_id.map(|v| v.into_inner()),
            facebook_id: value.facebook_id.map(|v| v.into_inner()),
        }
    }
}
