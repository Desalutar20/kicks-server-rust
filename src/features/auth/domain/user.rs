use time::OffsetDateTime;

use crate::features::auth::{
    EmailAddress, FacebookID, FirstName, GoogleID, HashedPassword, LastName, UserID,
    domain::{user_gender::UserGender, user_role::UserRole},
};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: UserID,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub email: EmailAddress,
    pub password: Option<HashedPassword>,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub role: UserRole,
    pub gender: Option<UserGender>,
    pub is_verified: bool,
    pub is_banned: bool,
    pub google_id: Option<GoogleID>,
    pub facebook_id: Option<FacebookID>,
}
