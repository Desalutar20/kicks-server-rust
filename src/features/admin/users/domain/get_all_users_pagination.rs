use crate::features::shared::Pagination;

pub const GET_ALL_USERS_DEFAULT_LIMIT: usize = 30;
pub const GET_ALL_USERS_MAX_LIMIT: usize = 100;

pub type GetAllUsersPagination = Pagination<GET_ALL_USERS_DEFAULT_LIMIT, GET_ALL_USERS_MAX_LIMIT>;
