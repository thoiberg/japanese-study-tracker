use std::env;

use axum::{http::StatusCode, Json};
use chrono::{DateTime, Utc};
use reqwest::Client;

use super::{internal_error, ErrorResponse};

pub async fn wanikani_handler(
) -> Result<Json<WaniKaniDataForFrontend>, (StatusCode, Json<ErrorResponse>)> {
    let summary = get_summary_data().await.map_err(internal_error)?;
    // TODO: have I studied today (possibly last study time?)

    Ok(Json(summary.into()))
}

async fn get_summary_data() -> anyhow::Result<WaniKaniResponse> {
    // TODO: Don't use expect, as it causes the thread to panic. Return the result and then map to a 500 error in the handler
    let api_token = env::var("WANIKANI_API_TOKEN").expect("WANIKANI_API_TOKEN must be set");
    let client = Client::new()
        .get("https://api.wanikani.com/v2/summary")
        .header("Wanikani-Revision", "20170710")
        .bearer_auth(api_token);

    let summary_data = client.send().await?.json::<WaniKaniResponse>().await?;

    Ok(summary_data)
}

#[derive(serde::Deserialize)]
struct WaniKaniResponse {
    object: String,
    url: String,
    data_updated_at: DateTime<Utc>,
    data: DataStructure,
}

#[derive(serde::Deserialize)]
struct DataStructure {
    lessons: Vec<Lesson>,
    next_reviews_at: DateTime<Utc>,
    reviews: Vec<Review>,
}

impl DataStructure {
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

// TODO: Find a better name
#[derive(serde::Serialize)]
pub struct WaniKaniDataForFrontend {
    data_updated_at: DateTime<Utc>,
    active_lesson_count: u32,
    active_review_count: u32,
}

impl From<WaniKaniResponse> for WaniKaniDataForFrontend {
    fn from(value: WaniKaniResponse) -> Self {
        WaniKaniDataForFrontend {
            data_updated_at: value.data_updated_at,
            active_lesson_count: value.data.total_lessons(),
            active_review_count: value.data.current_reviews(),
        }
    }
}
