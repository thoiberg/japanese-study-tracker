use anyhow::anyhow;
use async_trait::async_trait;
use redis::{AsyncCommands, RedisError};
use serde::de::DeserializeOwned;

#[async_trait]
pub trait Cacheable: DeserializeOwned + serde::Serialize + Clone {
    fn cache_key() -> String;
    fn ttl() -> usize;
    async fn get(redis_client: Option<redis::Client>) -> anyhow::Result<Box<Self>>;

    async fn cache_read(redis_client: &Option<redis::Client>) -> Option<Box<Self>> {
        let client = redis_client.as_ref()?;
        let mut conn = client
            .get_async_connection()
            .await
            .map_err(Self::cache_log)
            .ok()?;
        let cached_data: String = conn
            .get(Self::cache_key())
            .await
            .map_err(Self::cache_log)
            .ok()?;

        let wanikani_data = serde_json::from_str::<Self>(&cached_data)
            .map_err(Self::cache_log)
            .ok()?;

        Some(Box::new(wanikani_data))
    }

    async fn cache_write(
        redis_client: &Option<redis::Client>,
        // TODO: I'd rather pass in a ref but then Rust errors because
        // "future cannot be sent between threads safely"
        data: Self,
    ) -> anyhow::Result<()> {
        let client = redis_client
            .as_ref()
            .ok_or(anyhow!("No Redis Client set"))?;
        let mut conn = client.get_async_connection().await?;
        let json_data = serde_json::to_string(&data)?;
        let set_response: Result<(), RedisError> =
            conn.set_ex(Self::cache_key(), json_data, Self::ttl()).await;

        Ok(set_response?)
    }

    fn cache_log<E>(err: E)
    where
        E: Into<anyhow::Error>,
    {
        let redis_warning = format!("redis issue: {}", err.into());
        tracing::warn!(redis_warning);
    }
}
