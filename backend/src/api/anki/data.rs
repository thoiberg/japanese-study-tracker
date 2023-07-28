use chrono::{DateTime, Utc};

use super::proto_definitions::DeckInfo;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct AnkiData {
    active_review_count: u32,
    new_card_count: u32,
    data_updated_at: DateTime<Utc>,
}

impl From<DeckInfo> for AnkiData {
    fn from(deck: DeckInfo) -> Self {
        deck.review_card_count();

        Self {
            active_review_count: deck.review_card_count(),
            new_card_count: deck.new_card_count(),
            data_updated_at: Utc::now(),
        }
    }
}
