use chrono::{DateTime, Utc};

#[derive(serde::Serialize)]
pub struct SatoriData {
    data_updated_at: DateTime<Utc>,
    active_review_count: u32,
}

impl SatoriData {
    pub fn new(current_count: SatoriCurrentCardsResponse) -> Self {
        Self {
            data_updated_at: Utc::now(),
            active_review_count: current_count.result,
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
