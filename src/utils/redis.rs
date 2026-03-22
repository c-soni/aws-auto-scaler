use std::{string::String, time::Duration};
use redis::{AsyncConnectionConfig, AsyncTypedCommands, RedisError, aio::MultiplexedConnection};
use super::constants::REDIS_URL_VAR;

pub async fn get_connection() -> Option<MultiplexedConnection> {
    let config: AsyncConnectionConfig =
        AsyncConnectionConfig::new().set_connection_timeout(Some(Duration::from_millis(10000)));
    let connection_url = std::env::var(REDIS_URL_VAR).unwrap();
    let connection = match redis::Client::open(connection_url) {
        Ok(client) => match client
            .get_multiplexed_async_connection_with_config(&config)
            .await
        {
            Ok(conn) => conn,
            Err(e) => {
                log::error!("Failed to connect to Redis: {e}");
                return None;
            }
        },
        Err(e) => {
            log::error!("Failed to create Redis client: {e}");
            return None;
        }
    };
    Some(connection)
}

pub async fn get_string(
    mut connection: MultiplexedConnection,
    key: &str,
) -> Result<Option<String>, RedisError> {
    connection.get(key).await
}

pub async fn set_string(
    mut connection: MultiplexedConnection,
    key: &str,
    value: &str,
) -> Option<()> {
    connection.set(key, value).await.ok()
}
