use std::fmt::Display;

use anyhow::anyhow;
use async_trait::async_trait;
use redis::{AsyncCommands, RedisError, ToRedisArgs};
use serde::de::DeserializeOwned;

pub enum CacheKey {
    WanikaniSummary,
    WanikaniStats,
    Bunpro,
    SatoriReviewCards,
    SatoriNewCards,
    Anki,
}

impl Display for CacheKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cache_key = match self {
            CacheKey::WanikaniSummary => "wanikani_summary_data",
            CacheKey::WanikaniStats => "wanikani_stats_data",
            CacheKey::Bunpro => "bunpro_data",
            CacheKey::SatoriReviewCards => "satori_review_cards",
            CacheKey::SatoriNewCards => "satori_new_cards",
            CacheKey::Anki => "anki_data",
        };

        f.write_str(cache_key)
    }
}

impl ToRedisArgs for CacheKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg_fmt(self);
    }
}

#[async_trait]
pub trait Cacheable: DeserializeOwned + serde::Serialize + Clone {
    fn cache_key() -> CacheKey;
    fn ttl() -> usize;
    async fn api_fetch() -> anyhow::Result<Self>;

    async fn get(redis_client: &Option<redis::Client>) -> anyhow::Result<Self> {
        let cache_data = Self::cache_read(redis_client).await;

        if let Some(cache_data) = cache_data {
            return Ok(cache_data);
        }

        let api_data = Self::api_fetch().await?;

        // TODO: Figure out how to do this without cloning. Currently it returns with:
        //  future cannot be sent between threads safely
        {
            let cloned_data = api_data.clone();
            let write_result = Self::cache_write(redis_client, cloned_data).await;
            let _ = write_result.map_err(Self::cache_log);
        }

        Ok(api_data)
    }

    async fn cache_read(redis_client: &Option<redis::Client>) -> Option<Self> {
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

        Some(wanikani_data)
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
