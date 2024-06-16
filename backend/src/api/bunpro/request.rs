use std::env;

use async_trait::async_trait;
use axum::{extract::State, Json};
use chrono::Utc;
use reqwest::{Client, StatusCode};
use tokio::try_join;

use crate::api::{
    bunpro::data::BunproReviewStats,
    cacheable::{CacheKey, Cacheable},
    internal_error, ErrorResponse,
};

use super::data::{BunproData, StudyQueue};

mod stats;

pub async fn bunpro_handler(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<Json<BunproData>, (StatusCode, Json<ErrorResponse>)> {
    let (study_queue_data, stats_data) = try_join!(
        StudyQueue::get(&redis_client),
        BunproReviewStats::get(&redis_client)
    )
    .map_err(internal_error)?;

    let bunpro_data = BunproData::new(study_queue_data, stats_data);

    Ok(Json(bunpro_data))
}

#[async_trait]
impl Cacheable for StudyQueue {
    fn cache_key() -> CacheKey {
        CacheKey::Bunpro
    }

    fn ttl() -> usize {
        3600
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        let bunpro_api_token = env::var("BUNPRO_API_TOKEN")?;
        let url = format!("https://bunpro.jp/api/user/{bunpro_api_token}/study_queue");

        let study_queue = Client::new()
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
            .map_err(Into::into)
            .and_then(|body| serialize_response(&body))?;

        Ok(study_queue)
    }
}

fn serialize_response(body: &str) -> anyhow::Result<StudyQueue> {
    let mut json: StudyQueue = serde_json::from_str(body)?;

    json.fetched_at = Some(Utc::now());

    Ok(json)
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_bunpro_with_reviews() {
        let with_reviews = include_str!("./fixtures/bunpro_with_reviews.json");
        let response = serialize_response(with_reviews);
        assert!(response.is_ok());
    }

    #[test]
    fn test_bunpro_with_no_reviews() {
        let with_no_reviews = include_str!("./fixtures/bunpro_with_no_reviews.json");
        assert!(serialize_response(with_no_reviews).is_ok());
    }
}
