use std::env;

use async_trait::async_trait;
use axum::{extract::State, Json};
use reqwest::{Client, StatusCode};

use crate::api::{cacheable::Cacheable, internal_error, ErrorResponse};

use super::data::{BunproData, StudyQueue};

pub async fn bunpro_handler(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<Json<BunproData>, (StatusCode, Json<ErrorResponse>)> {
    let bunpro_data = BunproData::get(redis_client)
        .await
        .map_err(internal_error)?;

    Ok(Json(bunpro_data))
}

#[async_trait]
impl Cacheable for BunproData {
    fn cache_key() -> String {
        "bunpro_data".into()
    }

    fn ttl() -> usize {
        3600
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        let bunpro_api_token = env::var("BUNPRO_API_TOKEN")?;
        let url = format!(
            "https://bunpro.jp/api/user/{}/study_queue",
            bunpro_api_token
        );

        let study_queue = Client::new()
            .get(url)
            .send()
            .await?
            .text()
            .await
            .map(|body| Self::serialize_response(&body))??;

        Ok(study_queue.into())
    }
}

impl BunproData {
    fn serialize_response(body: &str) -> anyhow::Result<StudyQueue> {
        let json = serde_json::from_str(body)?;

        Ok(json)
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_bunpro_with_reviews() {
        let with_reviews = include_str!("./fixtures/bunpro_with_reviews.json");
        let response = BunproData::serialize_response(with_reviews);
        assert!(response.is_ok());
    }

    #[test]
    fn test_bunpro_with_no_reviews() {
        let with_no_reviews = include_str!("./fixtures/bunpro_with_no_reviews.json");
        assert!(BunproData::serialize_response(with_no_reviews).is_ok());
    }
}
