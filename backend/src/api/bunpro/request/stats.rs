use std::env;

use crate::api::{
    bunpro::data::BunproReviewStats,
    cacheable::{CacheKey, Cacheable},
};
use async_trait::async_trait;
use reqwest::{header, Client};

#[async_trait]
impl Cacheable for BunproReviewStats {
    fn cache_key() -> CacheKey {
        CacheKey::BunproStats
    }

    fn ttl() -> usize {
        3600
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        let client = bunpro_client()?;
        client
            .get("https://bunpro.jp/api/frontend/user_stats/review_activity")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
            .map(serialize_stats_response)?
    }
}

fn bunpro_client() -> anyhow::Result<Client> {
    let bunpro_cookie = env::var("BUNPRO_COOKIE")?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!("Token token={}", bunpro_cookie).parse().unwrap(),
    );

    Ok(Client::builder().default_headers(headers).build()?)
}

fn serialize_stats_response(body: String) -> anyhow::Result<BunproReviewStats> {
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
        let stats = serialize_stats_response(json_response.to_string());

        assert!(stats.is_ok());

        let date = NaiveDate::parse_from_str("2023-09-30", "%Y-%m-%d").unwrap();
        let count_for = stats.unwrap().count_for(date);
        assert_eq!(count_for, 21);
    }

    #[test]
    fn test_serialize_stats_response_for_missing_day() {
        let json_response = include_str!("../fixtures/bunpro_review_history.json");
        let stats = serialize_stats_response(json_response.to_string());

        assert!(stats.is_ok());

        let date = NaiveDate::parse_from_str("2023-01-01", "%Y-%m-%d").unwrap();
        let count_for = stats.unwrap().count_for(date);
        assert_eq!(count_for, 0);
    }
}
