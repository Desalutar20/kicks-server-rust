use crate::features::auth::{
    EmailAddress, FirstName, LastName, User, UserGender, UserID, UserRole,
};

#[derive(Debug, Clone)]
pub struct AppUser {
    pub id: UserID,
    pub email: EmailAddress,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub role: UserRole,
    pub gender: Option<UserGender>,
}

impl From<User> for AppUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role,
            gender: user.gender,
        }
    }
}
