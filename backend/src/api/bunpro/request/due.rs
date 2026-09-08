use crate::api::{
    bunpro::data::BunproDueStats,
    cacheable::{CacheKey, Cacheable},
};
use anyhow::anyhow;
use chrono::{DateTime, Duration, Utc};
use reqwest::Client;

impl Cacheable for BunproDueStats {
    fn cache_key() -> CacheKey {
        CacheKey::BunproDue
    }

    fn expires_at() -> DateTime<Utc> {
        Utc::now() + Duration::hours(1)
    }

    async fn api_fetch(client: Option<&Client>) -> anyhow::Result<Self> {
        let client = client.ok_or(anyhow!("no client passed in"))?;

        client
            .get("https://api.bunpro.jp/api/frontend/user/due")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
            .map_err(anyhow::Error::from)
            .and_then(serialize_due_response)
    }
}

fn serialize_due_response(body: String) -> anyhow::Result<BunproDueStats> {
    let json = serde_json::from_str(&body)?;

    Ok(json)
}

#[cfg(test)]
mod test_super {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_serialize_stats_response() {
        let json_response = include_str!("../fixtures/bunpro_due.json");
        let due = serialize_due_response(json_response.to_string());

        assert!(due.is_ok());

        let due = due.unwrap();

        assert_eq!(due.total_due_grammar, 3);
        assert_eq!(due.total_due_vocab, 17);
    }
}
