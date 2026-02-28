use time::OffsetDateTime;

use crate::{
    Result,
    features::{
        admin::users::{
            domain::{GetAllUsersPagination, GetAllUsersSearch},
            service::AdminUsersService,
        },
        auth::{
            EmailAddress, FacebookID, FirstName, GoogleID, LastName, User, UserGender, UserID,
            UserRole,
        },
        shared::Metadata,
    },
};

#[derive(Debug)]
pub struct AdminUser {
    pub id: UserID,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub email: EmailAddress,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub role: UserRole,
    pub gender: Option<UserGender>,
    pub is_verified: bool,
    pub is_banned: bool,
    pub google_id: Option<GoogleID>,
    pub facebook_id: Option<FacebookID>,
}

impl From<User> for AdminUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            role: value.role,
            gender: value.gender,
            is_verified: value.is_verified,
            is_banned: value.is_banned,
            google_id: value.google_id,
            facebook_id: value.facebook_id,
        }
    }
}

#[derive(Debug)]
pub struct GetAllUsersInput {
    pub is_banned: Option<bool>,
    pub is_verified: Option<bool>,
    pub search: Option<GetAllUsersSearch>,
    pub gender: Option<UserGender>,
    pub start_date: Option<OffsetDateTime>,
    pub end_date: Option<OffsetDateTime>,
    pub pagination: GetAllUsersPagination,
}

impl AdminUsersService {
    pub async fn get_all_users(
        &self,
        data: GetAllUsersInput,
    ) -> Result<(Vec<AdminUser>, Metadata)> {
        let pagination = data.pagination;
        self.repository
            .get_all_users(data)
            .await
            .map(|(users, total_count)| {
                let admin_users = users.into_iter().map(AdminUser::from).collect();

                (admin_users, Metadata::new(total_count, pagination))
            })
    }
}
