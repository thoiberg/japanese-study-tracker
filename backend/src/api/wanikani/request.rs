use core::fmt;
use std::env;

use axum::{extract::State, http::StatusCode, Json};
use redis::{AsyncCommands, RedisError};
use reqwest::Client;

use crate::api::{self, internal_error, ErrorResponse};

use super::data::{WanikaniData, WanikaniSummaryResponse};

pub async fn wanikani_handler(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<Json<WanikaniData>, (StatusCode, Json<ErrorResponse>)> {
    let wanikani_data = WanikaniData::get(redis_client)
        .await
        .map_err(internal_error)?;
    // TODO: have I studied today (possibly last study time?)

    Ok(Json(wanikani_data))
}

#[derive(Debug)]
struct MissingRedisError {}

impl std::error::Error for MissingRedisError {}

impl fmt::Display for MissingRedisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Redis no there")
    }
}

impl WanikaniData {
    pub async fn retrieve_from_cache(
        redis_client: &Option<redis::Client>,
        cache_key: &str,
    ) -> anyhow::Result<Self> {
        let client = redis_client.as_ref().ok_or(MissingRedisError {})?;
        let mut conn = client.get_async_connection().await?;
        let cached_data: String = conn.get(cache_key).await?;

        let wanikani_data = serde_json::from_str::<Self>(&cached_data)?;

        Ok(wanikani_data)
    }

    pub async fn write_to_cache(
        redis_client: &Option<redis::Client>,
        cache_key: &str,
        data: &Self,
    ) -> anyhow::Result<()> {
        let client = redis_client.as_ref().ok_or(MissingRedisError {})?;
        let mut conn = client.get_async_connection().await?;

        let json_data = serde_json::to_string(&data)?;
        let set_response: Result<(), RedisError> = conn.set(cache_key, json_data).await;

        Ok(set_response?)
    }

    pub async fn get(redis_client: Option<redis::Client>) -> anyhow::Result<Self> {
        let cache_key = "wanikani_data";

        let cache_data = Self::retrieve_from_cache(&redis_client, cache_key).await;

        if cache_data.is_ok() {
            return cache_data;
        }

        let api_data = Self::get_summary_data().await?;

        let _ = Self::write_to_cache(&redis_client, cache_key, &api_data).await;

        Ok(api_data)
    }

    pub async fn get_summary_data() -> anyhow::Result<Self> {
        let api_token = env::var("WANIKANI_API_TOKEN")?;
        let client = Client::new()
            .get("https://api.wanikani.com/v2/summary")
            .header("Wanikani-Revision", "20170710")
            .bearer_auth(api_token);

        let api_response = client
            .send()
            .await?
            .text()
            .await
            .map(|body| Self::deserialize_response(&body))?;

        Ok(api_response?.into())
    }

    fn deserialize_response(response_body: &str) -> anyhow::Result<WanikaniSummaryResponse> {
        let json_data = serde_json::from_str(response_body)?;

        Ok(json_data)
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_can_deserialize_empty_reviews() {
        let response_data = include_str!("./fixtures/wanikani_with_no_reviews.json");

        let response = WanikaniData::deserialize_response(response_data);

        assert!(response.is_ok());
    }

    #[test]
    fn test_can_deserialize_with_reviews() {
        let response_data = include_str!("./fixtures/wanikani_with_reviews.json");

        let response = WanikaniData::deserialize_response(response_data);

        assert!(response.is_ok());
    }
}
