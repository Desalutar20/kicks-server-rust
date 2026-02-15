use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

use crate::{
    app::AppState,
    clients::email_client::EmailClient,
    configuration::{app_config::ApplicationConfig, ratelimit_config::RateLimitConfig},
    features::auth::repository::AuthRepository,
    middlewares::authenticate,
};

mod constants;
mod domain;
mod handlers;
mod repository;
mod service;

pub use constants::*;
pub use domain::*;

pub use handlers::{UserResponse, generate_session_cookie};
pub use service::AuthService;

use handlers::*;

pub struct AuthModule {
    pub auth_service: AuthService,
}

impl AuthModule {
    pub fn new(
        app_config: ApplicationConfig,
        pool: PgPool,
        redis: MultiplexedConnection,
        email_client: Arc<EmailClient>,
    ) -> Self {
        let repository = AuthRepository::new(pool);

        Self {
            auth_service: AuthService::new(app_config, redis, email_client, repository),
        }
    }

    pub fn v1(&self, state: AppState, ratelimit: &RateLimitConfig) -> Router<AppState> {
        Router::new()
            .route(
                "/sign-up",
                post(sign_up_v1).layer(GovernorLayer::new(
                    GovernorConfigBuilder::default()
                        .per_second(60)
                        .burst_size(ratelimit.sign_up)
                        .finish()
                        .unwrap(),
                )),
            )
            .route(
                "/verify-account",
                post(verify_account_v1).layer(GovernorLayer::new(
                    GovernorConfigBuilder::default()
                        .per_second(60)
                        .burst_size(ratelimit.verify_account)
                        .finish()
                        .unwrap(),
                )),
            )
            .route(
                "/sign-in",
                post(sign_in_v1).layer(GovernorLayer::new(
                    GovernorConfigBuilder::default()
                        .per_second(60)
                        .burst_size(ratelimit.sign_in)
                        .finish()
                        .unwrap(),
                )),
            )
            .route(
                "/forgot-password",
                post(forgot_password_v1).layer(GovernorLayer::new(
                    GovernorConfigBuilder::default()
                        .per_second(60)
                        .burst_size(ratelimit.forgot_password)
                        .finish()
                        .unwrap(),
                )),
            )
            .route(
                "/reset-password",
                post(reset_password_v1).layer(GovernorLayer::new(
                    GovernorConfigBuilder::default()
                        .per_second(60)
                        .burst_size(ratelimit.reset_password)
                        .finish()
                        .unwrap(),
                )),
            )
            .route(
                "/logout",
                post(logout_v1)
                    .route_layer(middleware::from_fn_with_state(state.clone(), authenticate))
                    .layer(GovernorLayer::new(
                        GovernorConfigBuilder::default()
                            .per_second(60)
                            .burst_size(ratelimit.logout)
                            .finish()
                            .unwrap(),
                    )),
            )
            .route(
                "/me",
                get(get_me_v1)
                    .route_layer(middleware::from_fn_with_state(state.clone(), authenticate))
                    .layer(GovernorLayer::new(
                        GovernorConfigBuilder::default()
                            .per_second(60)
                            .burst_size(ratelimit.get_me)
                            .finish()
                            .unwrap(),
                    )),
            )
    }
}
