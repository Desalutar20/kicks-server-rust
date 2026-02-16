use crate::features::auth::{
    FacebookID, GoogleID, HashedPassword,
    domain::{
        email_address::EmailAddress, first_name::FirstName, last_name::LastName,
        user_gender::UserGender,
    },
};

#[derive(Debug)]
pub struct NewUser {
    pub email: EmailAddress,
    pub hashed_password: Option<HashedPassword>,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub gender: Option<UserGender>,
    pub google_id: Option<GoogleID>,
    pub facebook_id: Option<FacebookID>,
    pub is_verified: bool,
}
