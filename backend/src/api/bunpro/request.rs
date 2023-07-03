use std::env;

use axum::Json;
use reqwest::{Client, StatusCode};

use crate::api::{internal_error, ErrorResponse};

use super::data::{BunproData, StudyQueue};

pub async fn bunpro_handler() -> Result<Json<BunproData>, (StatusCode, Json<ErrorResponse>)> {
    let response = get_review_queue().await.map_err(internal_error)?;

    Ok(Json(response.into()))
}

async fn get_review_queue() -> anyhow::Result<StudyQueue> {
    let bunpro_api_token = env::var("BUNPRO_API_TOKEN")?;
    let url = format!(
        "https://bunpro.jp/api/user/{}/study_queue",
        bunpro_api_token
    );

    Client::new()
        .get(url)
        .send()
        .await?
        .text()
        .await
        .map(|body| serialize_response(&body))?
}

fn serialize_response(body: &str) -> anyhow::Result<StudyQueue> {
    let json = serde_json::from_str(body)?;

    Ok(json)
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_bunpro_with_reviews() {
        let with_reviews = include_str!("./fixtures/bunpro_with_reviews.json");
        let response = serialize_response(String::from(with_reviews));
        assert!(response.is_ok());
    }

    #[test]
    fn test_bunpro_with_no_reviews() {
        let with_no_reviews = include_str!("./fixtures/bunpro_with_no_reviews.json");
        assert!(serialize_response(String::from(with_no_reviews)).is_ok());
    }
}
