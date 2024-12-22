use std::env;

use crate::api::{
    bunpro::data::BunproReviewStats,
    cacheable::{CacheKey, Cacheable},
};
use anyhow::anyhow;
use chrono::{DateTime, Duration, Utc};
use reqwest::{header, Client};

impl Cacheable for BunproReviewStats {
    fn cache_key() -> CacheKey {
        CacheKey::BunproStats
    }

    fn expires_at() -> DateTime<Utc> {
        Utc::now() + Duration::hours(1)
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        let frontend_session_cookie = get_frontend_auth_token().await?;
        let client = bunpro_stats_client(frontend_session_cookie)?;

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

fn bunpro_stats_client(frontend_session_token: String) -> anyhow::Result<Client> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!("Token token={}", frontend_session_token)
            .parse()
            .unwrap(),
    );

    Ok(Client::builder().default_headers(headers).build()?)
}

async fn get_frontend_auth_token() -> anyhow::Result<String> {
    const TOKEN_NAME: &str = "frontend_api_token";

    let bunpro_grammar_cookie = env::var("BUNPRO_GRAMMAR_COOKIE")?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        header::COOKIE,
        format!("_grammar_app_session={}", bunpro_grammar_cookie).parse()?,
    );

    let client = Client::builder()
        .default_headers(headers)
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let bunpro_login = client
        .get("https://bunpro.jp/login")
        .send()
        .await?
        .error_for_status()?;

    let cookie = bunpro_login.cookies().find_map(|cookie| {
        if cookie.name() == TOKEN_NAME {
            Some(cookie.value().to_string())
        } else {
            None
        }
    });

    cookie.ok_or(anyhow!(format!("{} cookie could not be found", TOKEN_NAME)))
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
