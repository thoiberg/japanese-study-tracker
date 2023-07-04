use chrono::{DateTime, Utc};

#[derive(serde::Serialize)]
pub struct SatoriData {
    data_updated_at: DateTime<Utc>,
    active_review_count: u32,
    new_card_count: u32,
}

impl SatoriData {
    pub fn new(
        current_cards: SatoriCurrentCardsResponse,
        new_cards: SatoriNewCardsResponse,
    ) -> Self {
        Self {
            data_updated_at: Utc::now(),
            active_review_count: current_cards.result,
            new_card_count: new_cards.result,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct SatoriCurrentCardsResponse {
    result: u32,
    success: bool,
    message: Option<String>,
    exception: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct SatoriNewCardsResponse {
    result: u32,
    success: bool,
    message: Option<String>,
    exception: Option<String>,
}
