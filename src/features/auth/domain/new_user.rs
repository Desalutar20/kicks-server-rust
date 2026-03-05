use crate::features::auth::{
    EmailAddress, FirstName, HashedPassword, LastName, ProviderID, UserGender,
};

#[derive(Debug)]
pub struct NewUser {
    pub email: EmailAddress,
    pub hashed_password: Option<HashedPassword>,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub gender: Option<UserGender>,
    pub google_id: Option<ProviderID>,
    pub facebook_id: Option<ProviderID>,
    pub is_verified: bool,
}
