use axum::{Router, middleware::from_fn_with_state};
use sqlx::PgPool;

use crate::{
    app::AppState,
    features::{admin::users::AdminUsersModule, auth::UserRole},
    middlewares::{authenticate, authorize},
};

pub mod users;

pub struct AdminModule {
    pool: PgPool,
}

impl AdminModule {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn v1(self, state: AppState) -> Router<AppState> {
        let admin_users_module = AdminUsersModule::new(self.pool.clone());

        Router::new()
            .nest("/users", admin_users_module.v1())
            .layer(from_fn_with_state(vec![UserRole::Admin], authorize))
            .layer(from_fn_with_state(state.clone(), authenticate))
    }
}
