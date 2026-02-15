use kicksapi::configuration::database_config::DatabaseConfig;
use redis::aio::MultiplexedConnection;
use sqlx::postgres::PgConnectOptions;
use sqlx::{ConnectOptions, Connection, Executor, PgPool};
use testcontainers::core::WaitFor;
use testcontainers::{GenericImage, core::IntoContainerPort, runners::AsyncRunner};

pub async fn setup_postgres(config: &DatabaseConfig) -> (PgPool, impl AsyncFnOnce()) {
    let mut connection = PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username("postgres")
        .password("password")
        .database("postgres")
        .connect()
        .await
        .expect("Failed to connec to postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
        .await
        .expect("Failed to create database.");

    let pool = PgPool::connect_with(config.connect_options())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    let cloned = pool.clone();

    let cleanup = async move || {
        cloned.close().await;
        connection
            .execute(format!(r#"DROP DATABASE "{}";"#, config.name).as_str())
            .await
            .expect("Failed to drop database.");
        connection
            .close()
            .await
            .expect("Failed to close postgres connection");
    };

    (pool, cleanup)
}
pub async fn setup_redis() -> (MultiplexedConnection, String, u16, impl AsyncFnOnce()) {
    let container = GenericImage::new("redis", "8")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .start()
        .await
        .expect("Failed to setup redis container");

    let host_ip = container
        .get_host()
        .await
        .expect("Failed to get redis host");
    let host_port = container
        .get_host_port_ipv4(6379)
        .await
        .expect("Failed to get redis port");

    let url = format!("redis://{host_ip}:{host_port}");

    let client = redis::Client::open(url).expect("Failed to connect to Redis");

    let conn = client
        .get_multiplexed_async_connection()
        .await
        .expect("Failed to connect to Redis");

    let cleanup = async || {
        container.stop().await.expect("Failed to stop container");
        container.rm().await.expect("Failed to remove container");
    };

    (conn, host_ip.to_string(), host_port, cleanup)
}
