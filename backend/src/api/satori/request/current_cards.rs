use chrono::{DateTime, Duration, Utc};

use crate::api::{
    cacheable::{CacheKey, Cacheable},
    satori::data::SatoriCurrentCardsResponse,
};

use super::satori_client;

impl Cacheable for SatoriCurrentCardsResponse {
    fn cache_key() -> CacheKey {
        CacheKey::SatoriReviewCards
    }

    fn expires_at() -> DateTime<Utc> {
        Utc::now() + Duration::hours(1)
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        get_current_cards().await
    }
}

async fn get_current_cards() -> anyhow::Result<SatoriCurrentCardsResponse> {
    let client = satori_client()?;

    client
        .get("https://www.satorireader.com/api/studylist/due/count")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await
        .map(|body| serialize_current_cards_response(&body))?
}

fn serialize_current_cards_response(body: &str) -> anyhow::Result<SatoriCurrentCardsResponse> {
    let mut json_data: SatoriCurrentCardsResponse = serde_json::from_str(body)?;

    json_data.fetched_at = Some(Utc::now());

    Ok(json_data)
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_current_cards_with_pending_reviews() {
        let json_string = include_str!("../fixtures/current_cards_with_pending_reviews.json");
        let serialize_result = serialize_current_cards_response(json_string);

        assert!(serialize_result.is_ok());
    }

    #[test]
    fn test_current_cards_with_no_reviews() {
        let json_string = include_str!("../fixtures/current_cards_with_no_reviews.json");
        let serialize_result = serialize_current_cards_response(json_string);

        assert!(serialize_result.is_ok());
    }
}
