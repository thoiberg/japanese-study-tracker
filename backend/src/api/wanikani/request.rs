use std::env;

use async_trait::async_trait;
use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, FixedOffset, Timelike, Utc};
use reqwest::Client;
use tokio::try_join;

use crate::api::{
    cacheable::{CacheKey, Cacheable},
    internal_error, ErrorResponse,
};

use super::data::{WanikaniData, WanikaniReviewStats, WanikaniSummaryResponse};

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
    fn cache_key() -> CacheKey {
        CacheKey::Wanikani
    }

    fn ttl() -> usize {
        3600
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        let (summary_response, stats_response) =
            try_join!(Self::fetch_summary_data(), Self::fetch_stats_data())?;

        Ok(Self::new(summary_response, stats_response))
    }
}

impl WanikaniData {
    fn deserialize_summary_response(
        response_body: &str,
    ) -> anyhow::Result<WanikaniSummaryResponse> {
        let json_data = serde_json::from_str(response_body)?;

        Ok(json_data)
    }

    async fn fetch_summary_data() -> anyhow::Result<WanikaniSummaryResponse> {
        let api_token = env::var("WANIKANI_API_TOKEN")?;
        let client = Client::new()
            .get("https://api.wanikani.com/v2/summary")
            .header("Wanikani-Revision", "20170710")
            .bearer_auth(api_token);

        client
            .send()
            .await?
            .text()
            .await
            .map(|body| Self::deserialize_summary_response(&body))?
    }

    async fn fetch_stats_data() -> anyhow::Result<WanikaniReviewStats> {
        let url = stats_api_url(None);
        let api_token = env::var("WANIKANI_API_TOKEN")?;
        let client = Client::new()
            .get(url)
            .header("Wanikani-Revision", "20170710")
            .bearer_auth(api_token);

        client
            .send()
            .await?
            .text()
            .await
            .map(|body| Self::deserialize_stats_response(&body))?
    }

    fn deserialize_stats_response(response_body: &str) -> anyhow::Result<WanikaniReviewStats> {
        let json_data = serde_json::from_str(response_body)?;

        Ok(json_data)
    }
}

fn stats_api_url(from_date: Option<DateTime<Utc>>) -> String {
    let cutoff_date = today_jst_midnight_in_utc(from_date);

    format!(
        "https://api.wanikani.com/v2/review_statistics?updated_after={}",
        cutoff_date.format("%Y-%m-%dT%H:%M:%S%.3fZ")
    )
}

// TODO: Find a cleaner way to express this
fn today_jst_midnight_in_utc(from_date: Option<DateTime<Utc>>) -> DateTime<Utc> {
    let dt = from_date
        .unwrap_or(Utc::now())
        .with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());

    let midnight = dt
        .with_hour(0)
        .and_then(|dt| dt.with_minute(0))
        .and_then(|dt| dt.with_second(0))
        .and_then(|dt| dt.with_nanosecond(0));

    midnight.unwrap().naive_utc().and_utc()
}

#[cfg(test)]
mod test_super {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_can_deserialize_empty_reviews() {
        let response_data = include_str!("./fixtures/wanikani_with_no_reviews.json");

        let response = WanikaniData::deserialize_summary_response(response_data);

        assert!(response.is_ok());
    }

    #[test]
    fn test_can_deserialize_with_reviews() {
        let response_data = include_str!("./fixtures/wanikani_with_reviews.json");

        let response = WanikaniData::deserialize_summary_response(response_data);

        assert!(response.is_ok());
    }

    #[test]
    fn test_returns_todays_jst_date_in_utc() {
        let from_date = Utc.with_ymd_and_hms(2023, 7, 15, 15, 2, 15).unwrap();
        let url = stats_api_url(Some(from_date));

        assert_eq!(
            url,
            "https://api.wanikani.com/v2/review_statistics?updated_after=2023-07-15T15:00:00.000Z"
        );

        let from_date = Utc.with_ymd_and_hms(2023, 7, 15, 14, 58, 15).unwrap();
        let url = stats_api_url(Some(from_date));

        assert_eq!(
            url,
            "https://api.wanikani.com/v2/review_statistics?updated_after=2023-07-14T15:00:00.000Z"
        );
    }

    #[test]
    fn test_can_deserialize_stats() {
        let goal_met_response_data = include_str!("./fixtures/daily_goal_met.json");
        let stats = WanikaniData::deserialize_stats_response(goal_met_response_data);

        assert!(stats.is_ok());

        let goal_not_met_response_data = include_str!("./fixtures/daily_goal_not_met.json");
        let stats = WanikaniData::deserialize_stats_response(goal_not_met_response_data);

        assert!(stats.is_ok());
    }
}
