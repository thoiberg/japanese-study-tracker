use std::fmt::Display;

use anyhow::anyhow;
use chrono::{DateTime, Duration, Utc};
use redis::{AsyncCommands, SetOptions, ToRedisArgs};
use serde::de::DeserializeOwned;
use tokio::sync::Mutex;

pub enum CacheKey {
    WanikaniSummary,
    WanikaniStats,
    Bunpro,
    BunproStats,
    SatoriReviewCards,
    SatoriNewCards,
    SatoriStats,
    Anki,
}

impl Display for CacheKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cache_key = match self {
            CacheKey::WanikaniSummary => "wanikani_summary_data",
            CacheKey::WanikaniStats => "wanikani_stats_data",
            CacheKey::Bunpro => "bunpro_data",
            CacheKey::BunproStats => "bunpro_stats",
            CacheKey::SatoriReviewCards => "satori_review_cards",
            CacheKey::SatoriNewCards => "satori_new_cards",
            CacheKey::SatoriStats => "satori_stats",
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

pub trait Cacheable: DeserializeOwned + serde::Serialize {
    fn cache_key() -> CacheKey;
    fn expires_at() -> DateTime<Utc>;
    async fn api_fetch() -> anyhow::Result<Self>;

    async fn get(
        redis_client: &Option<redis::Client>,
    ) -> anyhow::Result<(Self, Option<DateTime<Utc>>)> {
        let cache_data = Self::cache_read(redis_client).await;
        let expires_at = Self::get_expiry_time(redis_client).await;

        if let Some(cache_data) = cache_data {
            return Ok((cache_data, expires_at));
        }

        let api_data = Self::api_fetch().await?;
        let api_data = Mutex::new(api_data);

        let write_result = Self::cache_write(redis_client, &api_data).await;

        let _ = write_result.map_err(Self::cache_log);

        Ok((api_data.into_inner(), Some(Self::expires_at())))
    }

    async fn get_expiry_time(redis_client: &Option<redis::Client>) -> Option<DateTime<Utc>> {
        let mut conn = redis_client
            .as_ref()?
            .get_multiplexed_tokio_connection()
            .await
            .map_err(Self::cache_log)
            .ok()?;

        let request_time = Utc::now();

        let remaining_ttl: Option<i64> = conn
            .ttl(Self::cache_key())
            .await
            .map_err(Self::cache_log)
            .ok();

        // redis uses -1 and -2 as control codes
        remaining_ttl
            .filter(|ttl| ttl > &0)
            .map(|ttl| request_time + Duration::seconds(ttl))
    }

    async fn cache_read(redis_client: &Option<redis::Client>) -> Option<Self> {
        let client = redis_client.as_ref()?;
        let mut conn = client
            .get_multiplexed_tokio_connection()
            .await
            .map_err(Self::cache_log)
            .ok()?;
        let cached_data = conn
            .get::<CacheKey, Option<String>>(Self::cache_key())
            .await
            .map_err(Self::cache_log)
            .ok()
            .flatten()?;

        serde_json::from_str::<Self>(&cached_data)
            .map_err(Self::cache_log)
            .ok()
    }

    async fn cache_write(
        redis_client: &Option<redis::Client>,
        data: &Mutex<Self>,
    ) -> anyhow::Result<()> {
        let client = redis_client
            .as_ref()
            .ok_or(anyhow!("No Redis Client set"))?;
        let mut conn = client.get_multiplexed_tokio_connection().await?;
        // not sure why it throws a compile error here and not in the value passed to serde_json
        // let unwrapped_data = *data.lock().await;
        let json_data = serde_json::to_string(&*data.lock().await)?;

        let unix_timestamp_expiry = u64::try_from(Self::expires_at().timestamp())?;
        let options =
            SetOptions::default().with_expiration(redis::SetExpiry::EXAT(unix_timestamp_expiry));

        let set_response = conn
            .set_options(Self::cache_key(), json_data, options)
            .await;

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
