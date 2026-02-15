use redis::{AsyncTypedCommands, aio::MultiplexedConnection};

use crate::{Result, configuration::redis_config::RedisConfig};

pub async fn build_redis_client(config: &RedisConfig) -> Result<MultiplexedConnection> {
    let client = redis::Client::open(config.connection_string())?;

    let mut conn = client.get_multiplexed_async_connection().await?;

    conn.ping().await?;

    Ok(conn)
}
