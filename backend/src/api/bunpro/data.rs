use std::collections::HashMap;

use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

// TODO: Add custom deserialization for Epoch timestamp in seconds
// to DateTime<Utc> type

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct StudyQueue {
    user_information: UserInformation,
    requested_information: StudyQueueData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BunproData {
    data_updated_at: DateTime<Utc>,
    active_review_count: u32,
    daily_study_goal_met: bool,
}

impl BunproData {
    pub fn new(study_queue: StudyQueue, stats: BunproReviewStats) -> Self {
        let today = Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
        let today_string = today.format("%Y-%m-%d").to_string();
        let todays_stats = stats.count_for(&today_string);
        let daily_study_goal_met = todays_stats.unwrap_or(0) > 0;

        Self {
            data_updated_at: Utc::now(),
            active_review_count: study_queue.requested_information.reviews_available,
            daily_study_goal_met,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct UserInformation {
    username: String,
    grammar_point_count: u32,
    ghost_review_count: u32,
    creation_date: u32,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct StudyQueueData {
    reviews_available: u32,
    next_review_date: u32,
    reviews_available_next_hour: u32,
    reviews_available_next_day: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BunproReviewStats {
    grammar: HashMap<String, u32>,
    vocab: HashMap<String, u32>,
}

impl BunproReviewStats {
    pub fn count_for(self, date: &str) -> Option<u32> {
        self.grammar
            .iter()
            .find_map(|(k, v)| if k == date { Some(*v) } else { None })
    }
}
