use redis::{AsyncConnectionConfig, AsyncTypedCommands, RedisError, aio::MultiplexedConnection};

use std::{string::String, time::Duration};

async fn get_connection() -> Option<MultiplexedConnection> {
    let config: AsyncConnectionConfig = AsyncConnectionConfig::new().set_connection_timeout(Some(Duration::from_millis(10000)));

    let connection = match redis::Client::open("redis://127.0.0.1") {
        Ok(client) => match client
            .get_multiplexed_async_connection_with_config(&config)
            .await
        {
            Ok(conn) => conn,
            Err(e) => {
                println!("Failed to connect to Redis: {e}");
                return None;
            }
        },
        Err(e) => {
            println!("Failed to create Redis client: {e}");
            return None;
        }
    };
    Some(connection)
}

async fn get_string(mut connection: MultiplexedConnection, key: &str) -> Result<Option<String>, RedisError> {
    connection.get(key).await
}

async fn set_string(mut connection: MultiplexedConnection, key: &str, value: &str) -> Option<()> {
    connection.set(key, value).await.ok()
}

async fn do_stuff(connection: MultiplexedConnection) {
    set_string(connection.clone(), "foo", "bar").await;

    match get_string(connection.clone(), "foo").await {
        Ok(s) => println!("Received: {}", s.unwrap_or(String::from("EMPTY"))),
        Err(_) => println!("ERROR")
    };

    match get_string(connection.clone(), "foo2").await {
        Ok(s) => println!("Received: {}", s.unwrap_or(String::from("EMPTY"))),
        Err(_) => println!("ERROR")
    };
}

#[tokio::main]
async fn main() {
    let connection: MultiplexedConnection = get_connection().await.unwrap();
    do_stuff(connection).await
    // Connect to S3
    // Connect to EC2 and create min instances
    // Connect to SQS
    // Polling for /status endpoint with image reqId query parameter
    // POST endpoint for 
}
