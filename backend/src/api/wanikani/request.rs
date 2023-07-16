use std::env;

use async_trait::async_trait;
use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, FixedOffset, SecondsFormat, Timelike, Utc};
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
    let (summary_response, stats_response) = try_join!(
        WanikaniSummaryResponse::get(redis_client.clone()),
        WanikaniReviewStats::get(redis_client.clone())
    )
    .map_err(internal_error)?;

    let wanikani_data = WanikaniData::new(summary_response, stats_response);

    Ok(Json(wanikani_data))
}

#[async_trait]
impl Cacheable for WanikaniSummaryResponse {
    fn cache_key() -> CacheKey {
        CacheKey::WanikaniSummary
    }

    fn ttl() -> usize {
        3600
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        let client = wanikani_client()?;

        client
            .get("https://api.wanikani.com/v2/summary")
            .send()
            .await?
            .text()
            .await
            .map(|body| Self::try_from_response_body(&body))?
    }
}

#[async_trait]
impl Cacheable for WanikaniReviewStats {
    fn cache_key() -> CacheKey {
        CacheKey::WanikaniStats
    }

    fn ttl() -> usize {
        3600
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        let url = stats_api_url(None);
        let client = wanikani_client()?;

        client
            .get(url)
            .send()
            .await?
            .text()
            .await
            .map(|body| Self::try_from_response_body(&body))?
    }
}

fn stats_api_url(from_date: Option<DateTime<Utc>>) -> String {
    let cutoff_date = today_jst_midnight_in_utc(from_date);

    format!(
        "https://api.wanikani.com/v2/review_statistics?updated_after={}",
        cutoff_date.to_rfc3339_opts(SecondsFormat::Millis, true)
    )
}

fn wanikani_client() -> anyhow::Result<reqwest::Client> {
    let api_token = env::var("WANIKANI_API_TOKEN")?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Wanikani-Revision", "20170710".parse().unwrap());

    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", api_token).parse().unwrap(),
    );

    Ok(Client::builder().default_headers(headers).build()?)
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
}
