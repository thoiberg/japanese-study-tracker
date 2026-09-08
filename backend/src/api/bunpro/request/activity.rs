use crate::api::{
    bunpro::data::BunproActivityStats,
    cacheable::{CacheKey, Cacheable},
};
use anyhow::anyhow;
use chrono::{DateTime, Duration, Utc};
use reqwest::Client;

impl Cacheable for BunproActivityStats {
    fn cache_key() -> CacheKey {
        CacheKey::BunproStats
    }

    fn expires_at() -> DateTime<Utc> {
        Utc::now() + Duration::hours(1)
    }

    async fn api_fetch(client: Option<&Client>) -> anyhow::Result<Self> {
        let client = client.ok_or(anyhow!("no client passed in"))?;

        client
            .get("https://api.bunpro.jp/api/frontend/user_stats/activity_daily")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
            .map_err(anyhow::Error::from)
            .and_then(serialize_activity_response)
    }
}

fn serialize_activity_response(body: String) -> anyhow::Result<BunproActivityStats> {
    let json = serde_json::from_str(&body)?;

    Ok(json)
}

#[cfg(test)]
mod test_super {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_serialize_stats_response() {
        let json_response = include_str!("../fixtures/bunpro_review_history.json");
        let stats = serialize_activity_response(json_response.to_string());

        assert!(stats.is_ok());

        let date = NaiveDate::parse_from_str("2023-09-30", "%Y-%m-%d").unwrap();
        let count_for = stats.unwrap().count_for(date);
        assert_eq!(count_for, 21);
    }

    #[test]
    fn test_serialize_stats_response_for_missing_day() {
        let json_response = include_str!("../fixtures/bunpro_review_history.json");
        let stats = serialize_activity_response(json_response.to_string());

        assert!(stats.is_ok());

        let date = NaiveDate::parse_from_str("2023-01-01", "%Y-%m-%d").unwrap();
        let count_for = stats.unwrap().count_for(date);
        assert_eq!(count_for, 0);
    }
}
