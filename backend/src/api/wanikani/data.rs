use chrono::{DateTime, Utc};

#[derive(serde::Deserialize)]
pub struct WanikaniSummaryResponse {
    object: String,
    url: String,
    data_updated_at: DateTime<Utc>,
    data: SummaryDataStructure,
}

#[derive(serde::Deserialize)]
pub struct SummaryDataStructure {
    lessons: Vec<Lesson>,
    next_reviews_at: DateTime<Utc>,
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

#[derive(serde::Deserialize)]
struct Lesson {
    available_at: DateTime<Utc>,
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
    fn daily_study_goal_met(&self) -> bool {
        self.total_count > 0
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
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
