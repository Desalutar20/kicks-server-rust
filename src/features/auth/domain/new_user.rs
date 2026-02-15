use crate::features::auth::{
    HashedPassword,
    domain::{
        email_address::EmailAddress, first_name::FirstName, last_name::LastName,
        user_gender::UserGender,
    },
};

#[derive(Debug)]
pub struct NewUser {
    pub email: EmailAddress,
    pub hashed_password: HashedPassword,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub gender: Option<UserGender>,
}
