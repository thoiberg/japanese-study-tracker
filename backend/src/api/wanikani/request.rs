use std::env;

use askama::Template;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Html,
    Json,
};
use chrono::{DateTime, Datelike, Duration, SecondsFormat, TimeZone, Utc};
use chrono_tz::Asia::Tokyo;
use reqwest::Client;
use tokio::try_join;

use crate::api::{
    add_expiry_header,
    cacheable::{CacheKey, Cacheable},
    internal_error, internal_error_html, ErrorResponse, HtmlErrorResponse,
};

use super::data::{WanikaniData, WanikaniReviewStats, WanikaniSummaryResponse};

pub async fn wanikani_handler(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<(HeaderMap, Json<WanikaniData>), (StatusCode, Json<ErrorResponse>)> {
    let ((summary_response, summary_expiry_time), (stats_response, stats_expiry_time)) = try_join!(
        WanikaniSummaryResponse::get(&redis_client),
        WanikaniReviewStats::get(&redis_client)
    )
    .map_err(internal_error)?;

    let wanikani_data = WanikaniData::new(summary_response, stats_response);

    let headers = add_expiry_header(HeaderMap::new(), &[summary_expiry_time, stats_expiry_time]);

    Ok((headers, Json(wanikani_data)))
}

pub async fn wanikani_htmx_handler(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<(HeaderMap, Html<String>), HtmlErrorResponse> {
    let ((summary_response, summary_expiry_time), (stats_response, stats_expiry_time)) = try_join!(
        WanikaniSummaryResponse::get(&redis_client),
        WanikaniReviewStats::get(&redis_client)
    )
    .map_err(internal_error_html)?;

    let wanikani_data = WanikaniData::new(summary_response, stats_response);

    let headers = add_expiry_header(HeaderMap::new(), &[summary_expiry_time, stats_expiry_time]);

    let html_string = wanikani_data.render().map_err(internal_error_html)?;

    Ok((headers, Html(html_string)))
}

impl Cacheable for WanikaniSummaryResponse {
    fn cache_key() -> CacheKey {
        CacheKey::WanikaniSummary
    }

    fn expires_at() -> DateTime<Utc> {
        Utc::now() + Duration::hours(1)
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        let client = wanikani_client()?;

        client
            .get("https://api.wanikani.com/v2/summary")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
            .map(|body| Self::try_from_response_body(&body))?
    }
}

impl Cacheable for WanikaniReviewStats {
    fn cache_key() -> CacheKey {
        CacheKey::WanikaniStats
    }

    fn expires_at() -> DateTime<Utc> {
        let one_hour = Duration::hours(1);

        Utc::now() + one_hour
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        let url = stats_api_url(None);
        let client = wanikani_client()?;

        client
            .get(url)
            .send()
            .await?
            .error_for_status()?
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
        format!("Bearer {api_token}").parse().unwrap(),
    );

    Ok(Client::builder().default_headers(headers).build()?)
}

fn today_jst_midnight_in_utc(from_date: Option<DateTime<Utc>>) -> DateTime<Utc> {
    let dt = from_date.unwrap_or(Utc::now()).with_timezone(&Tokyo);

    let midnight = Tokyo.with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 0, 0, 0);

    midnight.unwrap().with_timezone(&Utc)
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
