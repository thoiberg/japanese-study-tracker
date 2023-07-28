use std::{env, io::Cursor};

use anyhow::anyhow;
use async_trait::async_trait;
use axum::{extract::State, Json};
use bytes::Bytes;
use prost::Message;
use reqwest::{Client, StatusCode};

use crate::api::{
    anki::proto_definitions,
    cacheable::{CacheKey, Cacheable},
    internal_error, ErrorResponse,
};

use super::{
    data::AnkiData,
    proto_definitions::{DeckInfo, DeckListInfo},
};

pub async fn anki_handler(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<Json<AnkiData>, (StatusCode, Json<ErrorResponse>)> {
    let anki_data = AnkiData::get(&redis_client).await.map_err(internal_error)?;

    Ok(Json(anki_data))
}

#[async_trait]
impl Cacheable for AnkiData {
    fn cache_key() -> CacheKey {
        CacheKey::Anki
    }

    fn ttl() -> usize {
        3600
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        Ok(Self::from(get_html_data().await?))
    }
}

async fn get_html_data() -> anyhow::Result<DeckInfo> {
    let cookie = env::var("ANKIWEB_COOKIE")?;

    let encoded_message = Client::new()
        .post("https://ankiweb.net/svc/decks/deck-list-info")
        .header("Cookie", format!("ankiweb={}", cookie))
        .header("Content-Type", "application/octet-stream")
        .send()
        .await?
        .bytes()
        .await?;

    let deck_list_info = decode_protobuf_response(encoded_message)?;

    let japanese_deck =
        get_japanese_deck(&deck_list_info).ok_or(anyhow!("Could not find Japanese deck"))?;

    Ok(japanese_deck)
}

fn decode_protobuf_response(encoded_message: Bytes) -> anyhow::Result<DeckListInfo> {
    Ok(proto_definitions::DeckListInfo::decode(&mut Cursor::new(
        encoded_message,
    ))?)
}

fn get_japanese_deck(deck_list_info: &DeckListInfo) -> Option<DeckInfo> {
    // TODO: refactor to remove clone
    deck_list_info
        .all_decks_info
        .clone()?
        .decks
        .into_iter()
        .find(|deck| deck.deck_name == "Japanese")
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_with_no_pending_or_new_cards() {
        let html = include_str!("./fixtures/no_pending_reviews_or_cards.html");
        let count_values = parse_html(html);

        assert_eq!(count_values, vec!["0", "0"]);
    }

    #[test]
    fn test_with_pending_and_new_cards() {
        let html = include_str!("./fixtures/pending_reviews_and_cards.html");
        let count_values = parse_html(html);

        assert_eq!(count_values, vec!["79", "1"]);
    }
}
