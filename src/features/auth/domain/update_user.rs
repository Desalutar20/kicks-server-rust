use crate::features::auth::{
    HashedPassword,
    domain::{
        facebook_id::FacebookID, first_name::FirstName, google_id::GoogleID, last_name::LastName,
        user_gender::UserGender,
    },
};

#[derive(Debug)]
pub struct UpdateUser {
    pub password: Option<HashedPassword>,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub gender: Option<UserGender>,
    pub is_verified: Option<bool>,
    pub google_id: Option<GoogleID>,
    pub facebook_id: Option<FacebookID>,
}
