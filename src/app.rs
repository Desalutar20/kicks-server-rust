use std::{net::SocketAddr, ops::Deref, sync::Arc};

use axum::{
    Extension, Json, Router, extract::FromRef, http::StatusCode, middleware::from_fn,
    response::IntoResponse,
};
use axum_extra::extract::cookie::Key;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use tokio::{net::TcpListener, signal};
use tokio_util::sync::CancellationToken;
use tower::ServiceBuilder;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tracing::info;

use axum::http::{HeaderValue, Method};

use tower_http::{compression::CompressionLayer, cors::CorsLayer, timeout::TimeoutLayer};

use crate::{
    ErrorResponse, Result,
    clients::{email_client::build_email_client, redis_client::build_redis_client},
    configuration::{Configuration, app_config::ApplicationConfig},
    features::auth::{AuthModule, AuthService},
    middlewares::{error_logging, request_logging},
};

pub struct Application {
    port: u16,
    pool: PgPool,
    listener: TcpListener,
    router: Router,
}

#[derive(Clone)]
pub struct AppState(Arc<InnerState>);

pub struct InnerState {
    key: Key,
    pub config: ApplicationConfig,
    pub auth_service: AuthService,
}

impl Deref for AppState {
    type Target = InnerState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.0.key.clone()
    }
}

impl Application {
    pub async fn build(config: &Configuration) -> Result<Self> {
        let addr: SocketAddr = format!("{}:{}", config.application.host, config.application.port)
            .parse()
            .unwrap();
        let listener = TcpListener::bind(addr).await?;

        let database_pool =
            PgPoolOptions::new().connect_lazy_with(config.database.connect_options());
        let redis_client = build_redis_client(&config.redis).await?;
        let email_client = build_email_client(&config.smtp, &config.application).await?;

        let auth_module = AuthModule::new(
            config.application.clone(),
            database_pool.clone(),
            redis_client.clone(),
            email_client,
        );

        let state = AppState(Arc::new(InnerState {
            key: Key::from(config.application.cookie_secret.as_bytes()),
            config: config.application.clone(),
            auth_service: auth_module.auth_service.clone(),
        }));

        let app = Router::new()
            .nest(
                "/api/v1/auth",
                auth_module.v1(state.clone(), &config.ratelimit),
            )
            .with_state(state.clone())
            .fallback(handler_404)
            .layer(
                ServiceBuilder::new()
                    .layer(GovernorLayer::new(
                        GovernorConfigBuilder::default()
                            .per_second(60)
                            .burst_size(100)
                            .finish()
                            .unwrap(),
                    ))
                    .layer(from_fn(request_logging))
                    .layer(from_fn(error_logging))
                    .layer(
                        CorsLayer::new()
                            .allow_methods(vec![
                                Method::GET,
                                Method::POST,
                                Method::DELETE,
                                Method::PUT,
                                Method::PATCH,
                            ])
                            .allow_origin(
                                config
                                    .application
                                    .client_url
                                    .parse::<HeaderValue>()
                                    .unwrap(),
                            )
                            .allow_credentials(true),
                    )
                    .layer(CompressionLayer::new())
                    .layer(TimeoutLayer::with_status_code(
                        StatusCode::REQUEST_TIMEOUT,
                        Duration::from_secs(10),
                    ))
                    .layer(Extension(state.clone())),
            );

        Ok(Self {
            pool: database_pool.clone(),
            port: listener
                .local_addr()
                .expect("Failed to get tcp port")
                .port(),
            listener,
            router: app,
        })
    }

    pub async fn run(self, token: CancellationToken) -> Result<()> {
        info!("Running on port {}", self.port);

        axum::serve(
            self.listener,
            self.router
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(shutdown_signal(token))
        .await?;

        self.pool.close().await;

        Ok(())
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

async fn shutdown_signal(token: CancellationToken) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
        },
        _ = terminate => {
        },
        _ = token.cancelled() => {
        }
    }
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            message: "Not found".to_string(),
        }),
    )
}
