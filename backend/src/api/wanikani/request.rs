use std::env;

use axum::{http::StatusCode, Json};
use reqwest::Client;

use crate::api::{internal_error, ErrorResponse};

use super::data::{WaniKaniDataForFrontend, WaniKaniResponse};

pub async fn wanikani_handler(
) -> Result<Json<WaniKaniDataForFrontend>, (StatusCode, Json<ErrorResponse>)> {
    let summary = get_summary_data().await.map_err(internal_error)?;
    // TODO: have I studied today (possibly last study time?)

    Ok(Json(summary.into()))
}

async fn get_summary_data() -> anyhow::Result<WaniKaniResponse> {
    let api_token = env::var("WANIKANI_API_TOKEN")?;
    let client = Client::new()
        .get("https://api.wanikani.com/v2/summary")
        .header("Wanikani-Revision", "20170710")
        .bearer_auth(api_token);

    client
        .send()
        .await?
        .text()
        .await
        .map(|body| deserialize_response(&body))?
}

fn deserialize_response(response_body: &str) -> anyhow::Result<WaniKaniResponse> {
    let json_data = serde_json::from_str(response_body)?;

    Ok(json_data)
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_can_deserialize_empty_reviews() {
        let response_data = include_str!("./fixtures/wanikani_with_no_reviews.json");

        let response = deserialize_response(response_data.into());

        assert!(response.is_ok());
    }

    #[test]
    fn test_can_deserialize_with_reviews() {
        let response_data = include_str!("./fixtures/wanikani_with_reviews.json");

        let response = deserialize_response(response_data.into());

        assert!(response.is_ok());
    }
}
