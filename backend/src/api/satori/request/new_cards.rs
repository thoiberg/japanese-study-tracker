use chrono::{DateTime, Duration, Utc};

use crate::api::{
    cacheable::{CacheKey, Cacheable},
    satori::data::SatoriNewCardsResponse,
};

impl Cacheable for SatoriNewCardsResponse {
    fn cache_key() -> CacheKey {
        CacheKey::SatoriNewCards
    }

    fn expires_at() -> DateTime<Utc> {
        Utc::now() + Duration::hours(1)
    }

    async fn api_fetch(client: Option<&reqwest::Client>) -> anyhow::Result<Self> {
        get_new_cards(client).await
    }
}

async fn get_new_cards(client: Option<&reqwest::Client>) -> anyhow::Result<SatoriNewCardsResponse> {
    let client = client.expect("client not passed in");

    client
        .get("https://www.satorireader.com/api/studylist/pending-auto-importable/count")
        .send()
        .await?
        .text()
        .await
        .map(|body| serialize_new_cards_response(&body))?
}

fn serialize_new_cards_response(body: &str) -> anyhow::Result<SatoriNewCardsResponse> {
    let json_data: SatoriNewCardsResponse = serde_json::from_str(body)?;

    Ok(json_data)
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_new_card_with_pending_cards() {
        let json_string = include_str!("../fixtures/new_cards_with_pending_cards.json");
        let serialized_result = serialize_new_cards_response(json_string);

        assert!(serialized_result.is_ok());
    }

    #[test]
    fn test_new_card_with_no_cards() {
        let json_string = include_str!("../fixtures/new_cards_with_no_cards.json");
        let serialized_result = serialize_new_cards_response(json_string);

        assert!(serialized_result.is_ok());
    }
}
