use crate::features::auth::{FirstName, HashedPassword, LastName, ProviderID, UserGender};

#[derive(Debug)]
pub struct UpdateUser {
    pub password: Option<HashedPassword>,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub gender: Option<UserGender>,
    pub is_verified: Option<bool>,
    pub google_id: Option<ProviderID>,
    pub facebook_id: Option<ProviderID>,
}
