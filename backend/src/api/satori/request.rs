use std::env;

use async_trait::async_trait;
use axum::{extract::State, Json};
use reqwest::{Client, StatusCode};
use tokio::try_join;

use crate::api::{
    cacheable::{CacheKey, Cacheable},
    internal_error, ErrorResponse,
};

use super::data::{SatoriCurrentCardsResponse, SatoriData, SatoriNewCardsResponse};

pub async fn satori_handler(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<Json<SatoriData>, (StatusCode, Json<ErrorResponse>)> {
    let satori_data = SatoriData::get(redis_client)
        .await
        .map_err(internal_error)?;

    Ok(Json(satori_data))
}

#[async_trait]
impl Cacheable for SatoriData {
    fn cache_key() -> CacheKey {
        CacheKey::Satori
    }

    fn ttl() -> usize {
        3600
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        let (current_cards, new_cards) = try_join!(get_current_cards(), get_new_cards())?;

        Ok(Self::new(current_cards, new_cards))
    }
}

async fn get_current_cards() -> anyhow::Result<SatoriCurrentCardsResponse> {
    let client = satori_client()?;

    client
        .get("https://www.satorireader.com/api/studylist/due/count")
        .send()
        .await?
        .text()
        .await
        .map(|body| serialize_current_cards_response(&body))?
}

fn serialize_current_cards_response(body: &str) -> anyhow::Result<SatoriCurrentCardsResponse> {
    let json_data: SatoriCurrentCardsResponse = serde_json::from_str(body)?;

    Ok(json_data)
}

async fn get_new_cards() -> anyhow::Result<SatoriNewCardsResponse> {
    let client = satori_client()?;

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

fn satori_client() -> anyhow::Result<Client> {
    let satori_cookie = env::var("SATORI_COOKIE")?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Cookie",
        format!("SessionToken={}", satori_cookie).parse().unwrap(),
    );

    Ok(Client::builder().default_headers(headers).build()?)
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_current_cards_with_pending_reviews() {
        let json_string = include_str!("./fixtures/current_cards_with_pending_reviews.json");
        let serialize_result = serialize_current_cards_response(json_string);

        assert!(serialize_result.is_ok());
    }

    #[test]
    fn test_current_cards_with_no_reviews() {
        let json_string = include_str!("./fixtures/current_cards_with_no_reviews.json");
        let serialize_result = serialize_current_cards_response(json_string);

        assert!(serialize_result.is_ok());
    }

    #[test]
    fn test_new_card_with_pending_cards() {
        let json_string = include_str!("./fixtures/new_cards_with_pending_cards.json");
        let serialized_result = serialize_new_cards_response(json_string);

        assert!(serialized_result.is_ok());
    }

    #[test]
    fn test_new_card_with_no_cards() {
        let json_string = include_str!("./fixtures/new_cards_with_no_cards.json");
        let serialized_result = serialize_new_cards_response(json_string);

        assert!(serialized_result.is_ok());
    }
}
