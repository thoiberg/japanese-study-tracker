use chrono::{DateTime, Utc};
use serde::Deserialize;

// TODO: Add custom deserialization for Epoch timestamp in seconds
// to DateTime<Utc> type

#[derive(Debug, Deserialize)]
pub struct StudyQueue {
    user_information: UserInformation,
    requested_information: StudyQueueData,
}

#[derive(serde::Serialize)]
pub struct BunproData {
    data_updated_at: DateTime<Utc>,
    active_review_count: u32,
}

impl From<StudyQueue> for BunproData {
    fn from(value: StudyQueue) -> Self {
        Self {
            data_updated_at: Utc::now(),
            active_review_count: value.requested_information.reviews_available,
        }
    }
}

#[derive(Debug, Deserialize)]
struct UserInformation {
    username: String,
    grammar_point_count: u32,
    ghost_review_count: u32,
    creation_date: u32,
}

#[derive(Debug, Deserialize)]
struct StudyQueueData {
    reviews_available: u32,
    next_review_date: u32,
    reviews_available_next_hour: u32,
    reviews_available_next_day: u32,
}
