use std::env;

use askama::Template;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Html,
    Json,
};
use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use tokio::try_join;

use crate::api::{
    add_expiry_header,
    bunpro::data::BunproReviewStats,
    cacheable::{CacheKey, Cacheable},
    internal_error, ErrorResponse,
};

use super::data::{BunproData, StudyQueue};

mod stats;

pub async fn bunpro_handler(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<(HeaderMap, Json<BunproData>), (StatusCode, Json<ErrorResponse>)> {
    let ((study_queue_data, study_queue_expiry), (stats_data, stats_expiry)) = try_join!(
        StudyQueue::get(&redis_client),
        BunproReviewStats::get(&redis_client)
    )
    .map_err(internal_error)?;

    let bunpro_data = BunproData::new(study_queue_data, stats_data);

    let headers = add_expiry_header(HeaderMap::new(), &[study_queue_expiry, stats_expiry]);

    Ok((headers, Json(bunpro_data)))
}

pub async fn bunpro_htmx_hander(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<(HeaderMap, Html<String>), StatusCode> {
    let ((study_queue_data, study_queue_expiry), (stats_data, stats_expiry)) = try_join!(
        StudyQueue::get(&redis_client),
        BunproReviewStats::get(&redis_client)
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let bunpro_data = BunproData::new(study_queue_data, stats_data);

    let headers = add_expiry_header(HeaderMap::new(), &[study_queue_expiry, stats_expiry]);
    let html_string = bunpro_data.render().unwrap();

    Ok((headers, Html(html_string)))
}

impl Cacheable for StudyQueue {
    fn cache_key() -> CacheKey {
        CacheKey::Bunpro
    }

    fn expires_at() -> DateTime<Utc> {
        Utc::now() + Duration::hours(1)
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
