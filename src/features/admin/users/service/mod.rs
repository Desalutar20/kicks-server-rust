use crate::features::admin::users::repository::AdminUsersRepository;

pub mod get_all_users;
pub mod remove_user;
pub mod toggle_user_is_banned;

pub struct AdminUsersService {
    repository: AdminUsersRepository,
}

impl AdminUsersService {
    pub fn new(repository: AdminUsersRepository) -> Self {
        Self { repository }
    }
}
