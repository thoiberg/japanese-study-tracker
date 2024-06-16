use std::{env, io::Cursor};

use anyhow::anyhow;
use async_trait::async_trait;
use axum::{extract::State, Json};
use bytes::Bytes;
use chrono::{DateTime, Duration, Utc};
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

    fn expires_at() -> DateTime<Utc> {
        Utc::now() + Duration::hours(1)
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        Ok(Self::from(get_decks_data().await?))
    }
}

async fn get_decks_data() -> anyhow::Result<DeckInfo> {
    let cookie = env::var("ANKIWEB_COOKIE")?;

    let encoded_message = Client::new()
        .post("https://ankiweb.net/svc/decks/deck-list-info")
        .header("Cookie", format!("ankiweb={cookie}"))
        .header("Content-Type", "application/octet-stream")
        .send()
        .await?
        .error_for_status()?
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

    use base64::{engine::general_purpose, Engine as _};

    #[test]
    fn test_can_decode_protobuf_message_with_data() {
        let encoded_message = include_str!("./fixtures/protobuf_with_reviews_and_new_cards");
        let decoded_message = general_purpose::STANDARD
            .decode(encoded_message)
            .expect("base64 decode failed");

        let deck_list_response = decode_protobuf_response(Bytes::from(decoded_message));

        assert!(deck_list_response.is_ok());

        let decks = deck_list_response
            .unwrap()
            .all_decks_info
            .expect("deck lists was empty")
            .decks;
        assert_eq!(decks.len(), 1);

        let japanese_deck = decks.first().unwrap();

        assert_eq!(japanese_deck.review_card_count(), 59);
        assert_eq!(japanese_deck.new_card_count(), 40);
        assert_eq!(japanese_deck.uncapped_new_card_count(), 159);
    }

    #[test]
    fn test_can_decode_protobuf_message_with_no_data() {
        let encoded_message = include_str!("./fixtures/protobuf_with_no_reviews_or_new_cards");
        let decoded_message = general_purpose::STANDARD
            .decode(encoded_message)
            .expect("base64 decode failed");

        let deck_list_response = decode_protobuf_response(Bytes::from(decoded_message));

        assert!(deck_list_response.is_ok());

        let decks = deck_list_response
            .unwrap()
            .all_decks_info
            .expect("deck lists was empty")
            .decks;
        assert_eq!(decks.len(), 1);

        let japanese_deck = decks.first().unwrap();

        assert_eq!(japanese_deck.review_card_count(), 0);
        assert_eq!(japanese_deck.new_card_count(), 0);
    }

    #[test]
    fn test_can_decode_protobuf_message_with_learning_count() {
        let encoded_message = include_str!("./fixtures/protobuf_with_review_and_learning_cards");
        let decoded_message = general_purpose::STANDARD
            .decode(encoded_message)
            .expect("base64 decode failed");

        let deck_list_response = decode_protobuf_response(Bytes::from(decoded_message));

        assert!(deck_list_response.is_ok());

        let decks = deck_list_response
            .unwrap()
            .all_decks_info
            .expect("deck lists was empty")
            .decks;
        assert_eq!(decks.len(), 1);

        let japanese_deck = decks.first().unwrap();

        assert_eq!(japanese_deck.review_card_count(), 6);
        assert_eq!(japanese_deck.learn_count(), 8);
    }
}
