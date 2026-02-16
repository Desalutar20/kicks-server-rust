use std::{env, panic::AssertUnwindSafe, sync::LazyLock, time::Duration};

use futures::FutureExt;
use redis::aio::MultiplexedConnection;
use reqwest::Client;

use kicksapi::{
    app::Application,
    configuration::{
        Configuration, app_config::ApplicationConfig, ratelimit_config::RateLimitConfig,
    },
};

use sqlx::PgPool;
use tokio_util::sync::CancellationToken;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt};
use uuid::Uuid;

use crate::e2e::testapp::setup_database::{setup_postgres, setup_redis};

mod auth_requests;
mod database;
mod setup_database;

pub use database::RedisKeyType;

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let env_filter = EnvFilter::new("info");

    let fmt_layer = fmt::layer::<Registry>()
        .with_writer(std::io::stdout)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .pretty();

    set_global_default(Registry::default().with(fmt_layer).with(env_filter))
        .expect("Failed to set subscriber");
});

pub struct TestApp {
    address: String,
    pool: PgPool,
    redis: MultiplexedConnection,
    http_client: Client,
    pub ratelimit_config: RateLimitConfig,
    pub application_config: ApplicationConfig,
}

pub async fn setup<T>(func: T)
where
    T: AsyncFnOnce(TestApp),
{
    LazyLock::force(&TRACING);

    let var = env::var("APPLICATION__ENV").unwrap_or("".into());

    if var != "test" && var != "test-ci" {
        unsafe {
            env::set_var("APPLICATION__ENV", "test");
        }
    }

    let mut config = Configuration::new();
    config.application.port = 0;
    config.database.name = format!("test-{}", Uuid::new_v4());
    config.ratelimit.sign_up = 15;
    config.ratelimit.reset_password = 15;

    let (redis, host, port, cleanup_redis) = setup_redis().await;
    let (pool, cleanup_postgres) = setup_postgres(&config.database).await;

    config.redis.host = host;
    config.redis.port = port;

    let app = Application::build(&config)
        .await
        .expect("Failed to build app");

    let app_port = app.port();

    let token = CancellationToken::new();
    let server_handle = tokio::spawn(app.run(token.clone()));

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .cookie_store(true)
        .build()
        .expect("Failed to build http client");

    let test_app = TestApp {
        address: format!("http://localhost:{}/api/v1", app_port),
        pool: pool.clone(),
        redis,
        http_client: client,
        ratelimit_config: config.ratelimit,
        application_config: config.application,
    };

    let result = AssertUnwindSafe(func(test_app)).catch_unwind().await;

    token.cancel();
    server_handle.await.unwrap().unwrap();

    cleanup_redis().await;
    cleanup_postgres().await;

    if let Err(err) = result {
        panic!("test panicked: {err:?}");
    }
}
