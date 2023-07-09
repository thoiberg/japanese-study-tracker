use std::env;

use async_trait::async_trait;
use axum::{extract::State, http::StatusCode, Json};
use reqwest::Client;

use crate::api::{cacheable::Cacheable, internal_error, ErrorResponse};

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

#[async_trait]
impl Cacheable for WanikaniData {
    fn cache_key() -> String {
        "wanikani_data".into()
    }

    fn ttl() -> usize {
        3600
    }

    async fn api_fetch() -> anyhow::Result<Self> {
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
}

impl WanikaniData {
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
