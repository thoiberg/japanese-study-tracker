use askama::Template;
use chrono::{DateTime, Utc};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct WanikaniSummaryResponse {
    data_updated_at: DateTime<Utc>,
    data: SummaryDataStructure,
}

impl WanikaniSummaryResponse {
    pub fn try_from_response_body(response_body: &str) -> anyhow::Result<Self> {
        let json_data = serde_json::from_str(response_body)?;

        Ok(json_data)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SummaryDataStructure {
    lessons: Vec<Lesson>,
    reviews: Vec<Review>,
}

impl SummaryDataStructure {
    fn total_lessons(&self) -> u32 {
        self.lessons
            .iter()
            .fold(0, |acc, lesson| acc + lesson.total_count())
    }

    fn current_reviews(&self) -> u32 {
        // first item in the list is the current active review queue
        // if no active reviews then it's empty
        match self.reviews.first() {
            Some(reviews) => reviews.total_count(),
            None => 0, // no reviews I guess - yay!
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Lesson {
    subject_ids: Vec<u32>,
}

impl Lesson {
    fn total_count(&self) -> u32 {
        self.subject_ids.iter().fold(0, |acc, _| acc + 1)
    }
}

type Review = Lesson;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WanikaniReviewStats {
    total_count: u32,
}

impl WanikaniReviewStats {
    pub fn try_from_response_body(response_body: &str) -> anyhow::Result<Self> {
        let json_data = serde_json::from_str(response_body)?;

        Ok(json_data)
    }

    fn daily_study_goal_met(&self) -> bool {
        self.total_count > 0
    }
}

#[derive(serde::Serialize, Template)]
#[template(path = "wanikani.html")]
pub struct WanikaniData {
    data_updated_at: DateTime<Utc>,
    active_lesson_count: u32,
    active_review_count: u32,
    daily_study_goal_met: bool,
}

impl WanikaniData {
    pub fn new(summary: WanikaniSummaryResponse, review_stats: WanikaniReviewStats) -> Self {
        WanikaniData {
            data_updated_at: summary.data_updated_at,
            active_lesson_count: summary.data.total_lessons(),
            active_review_count: summary.data.current_reviews(),
            daily_study_goal_met: review_stats.daily_study_goal_met(),
        }
    }
}

#[cfg(test)]
mod test_wanikani_stats {
    use super::*;

    #[test]
    fn test_can_deserialize_stats() {
        let goal_met_response_data = include_str!("./fixtures/daily_goal_met.json");
        let stats = WanikaniReviewStats::try_from_response_body(goal_met_response_data);

        assert!(stats.is_ok());

        let goal_not_met_response_data = include_str!("./fixtures/daily_goal_not_met.json");
        let stats = WanikaniReviewStats::try_from_response_body(goal_not_met_response_data);

        assert!(stats.is_ok());
    }
}

#[cfg(test)]
mod test_wanikani_summary_response {
    use super::*;

    #[test]
    fn test_can_deserialize_empty_reviews() {
        let response_data = include_str!("./fixtures/wanikani_with_no_reviews.json");

        let response = WanikaniSummaryResponse::try_from_response_body(response_data);

        assert!(response.is_ok());
    }

    #[test]
    fn test_can_deserialize_with_reviews() {
        let response_data = include_str!("./fixtures/wanikani_with_reviews.json");

        let response = WanikaniSummaryResponse::try_from_response_body(response_data);

        assert!(response.is_ok());
    }
}
