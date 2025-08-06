use crate::config::{CONFIG, RedisMode};
use redis::{
    ToRedisArgs,
    Client, ConnectionInfo, ProtocolVersion, RedisConnectionInfo, aio::MultiplexedConnection };
use url::Url;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RedisArray {
    pub key: String,
    pub user: String,
    pub data: String,
    pub expires_at: Option<u64>, // секунды до истечения TTL
}


/// redis_read(&connection,key)

pub async fn redis_read(
    conn: &mut MultiplexedConnection,
    key: &str,
) -> redis::RedisResult<Option<RedisArray>> {
    // 1. Читаем значение
    let data: Option<String> = redis::cmd("GET")
        .arg(key)
        .query_async(conn)
        .await?;

    // Если значение отсутствует — сразу возвращаем None
    let Some(data) = data else {
        return Ok(None);
    };

    // 2. Получаем TTL
    let ttl: i64 = redis::cmd("TTL")
        .arg(key)
        .query_async(conn)
        .await?;

    let expires_at = if ttl >= 0 {
        Some(ttl as u64)
    } else {
        None // -1 (нет TTL), -2 (нет ключа)
    };

    // 3. Формируем структуру
    Ok(Some(RedisArray {
        key: key.to_string(),
        user: "system".to_string(), // или бери из контекста, если надо
        data,
        expires_at,
    }))
}



/// redis_save(&connection,key,value,ttl)
/*
pub async fn redis_save(
    conn: &mut MultiplexedConnection,
    key: &str,
    value: &str,
    ttl_seconds: usize,
) -> redis::RedisResult<()> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("EX")
        .arg(ttl_seconds)
        .query_async::<()>(&mut *conn)
        .await
}
*/
pub async fn redis_save<T: ToRedisArgs>(
    conn: &mut MultiplexedConnection,
    key: &str,
    value: T,
    ttl_seconds: usize,
) -> redis::RedisResult<()> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("EX")
        .arg(ttl_seconds)
        .query_async::<()>(&mut *conn)
        .await
}


pub async fn redis_delete(
    conn: &mut MultiplexedConnection,
    key: &str,
) -> redis::RedisResult<bool> {
    let deleted: i32 = redis::cmd("DEL")
        .arg(key)
        .query_async(conn)
        .await?;

    Ok(deleted > 0)
}



/// redis_connect()
pub async fn redis_connect() -> anyhow::Result<MultiplexedConnection> {
    let default_port = match CONFIG.redis_mode {
        RedisMode::Sentinel => 6379,
        RedisMode::Direct => 6380,
    };

    let urls = CONFIG
        .redis_urls
        .iter()
        .map(|url| {
            redis::ConnectionAddr::Tcp(
                url.host().unwrap().to_string(),
                url.port().unwrap_or(default_port),
            )
        })
        .collect::<Vec<_>>();

    let conn = if CONFIG.redis_mode == RedisMode::Sentinel {
        use redis::sentinel::{SentinelClientBuilder, SentinelServerType};

        let mut sentinel = SentinelClientBuilder::new(
            urls,
            CONFIG.redis_service.to_owned(),
            SentinelServerType::Master,
        )
        .unwrap()
        .set_client_to_redis_protocol(ProtocolVersion::RESP3)
        .set_client_to_redis_db(0)
        .set_client_to_redis_password(CONFIG.redis_password.clone())
        .set_client_to_sentinel_password(CONFIG.redis_password.clone())
        .build()?;

        sentinel.get_async_connection().await?
    } else {
        let single = urls
            .first()
            .ok_or_else(|| anyhow::anyhow!("No redis URL provided"))?;

        let redis_connection_info = RedisConnectionInfo {
            db: 0,
            username: None,
            password: Some(CONFIG.redis_password.clone()),
            protocol: ProtocolVersion::RESP3,
        };

        let connection_info = ConnectionInfo {
            addr: single.clone(),
            redis: redis_connection_info,
        };

        let client = Client::open(connection_info)?;
        client.get_multiplexed_async_connection().await?
    };

    Ok(conn)
}
