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
    fn total_reviews(&self) -> u32 {
        self.reviews
            .iter()
            .fold(0, |acc, review| acc + review.total_count())
    }

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

#[derive(serde::Serialize)]
pub struct WanikaniData {
    data_updated_at: DateTime<Utc>,
    active_lesson_count: u32,
    active_review_count: u32,
}

impl From<WanikaniSummaryResponse> for WanikaniData {
    fn from(value: WanikaniSummaryResponse) -> Self {
        WanikaniData {
            data_updated_at: value.data_updated_at,
            active_lesson_count: value.data.total_lessons(),
            active_review_count: value.data.current_reviews(),
        }
    }
}
