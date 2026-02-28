use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post},
};
use sqlx::PgPool;

use crate::{
    app::AppState,
    features::admin::users::{
        handlers::{
            get_all_users_handler_v1, remove_user_handler_v1, toggle_user_is_banned_handler_v1,
        },
        repository::AdminUsersRepository,
        service::AdminUsersService,
    },
};

mod domain;
mod handlers;
mod repository;
mod service;

pub use handlers::AdminUserResponse;

pub use domain::{
    GET_ALL_USERS_DEFAULT_LIMIT, GET_ALL_USERS_MAX_LIMIT, GET_ALL_USERS_SEARCH_MAX_LENGTH,
};

struct AdminUsersState {
    pub service: AdminUsersService,
}

pub struct AdminUsersModule {
    pool: PgPool,
}

impl AdminUsersModule {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn v1(self) -> Router<AppState> {
        let repository = AdminUsersRepository::new(self.pool.clone());
        let service = AdminUsersService::new(repository);

        Router::new()
            .route("/", get(get_all_users_handler_v1))
            .route("/{user_id}", delete(remove_user_handler_v1))
            .route(
                "/{user_id}/toggle-is-banned",
                post(toggle_user_is_banned_handler_v1),
            )
            .with_state(Arc::new(AdminUsersState { service }))
    }
}
