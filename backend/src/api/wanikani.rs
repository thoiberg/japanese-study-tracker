use std::env;

use axum::http::StatusCode;
use reqwest::Client;

pub async fn wanikani_handler() -> Result<&'static str, (StatusCode, &'static str)> {
    // current lessons
    // current review queue
    let summary = get_summary_data().await;
    // TODO: have I studied today (possibly last study time?)

    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Errrr it's still in progress",
    ))
}

#[derive(serde::Deserialize)]
pub struct WaniKaniResponse {
    object: String,
}

async fn get_summary_data() -> anyhow::Result<WaniKaniResponse> {
    let api_token = env::var("WANIKANI_API_TOKEN").expect("WANIKANI_API_TOKEN must be set");
    let client = Client::new()
        .get("https://api.wanikani.com/v2/summary")
        .header("Wanikani-Revision", "20170710")
        .bearer_auth(api_token);

    let summary_data = client.send().await?.json::<WaniKaniResponse>().await?;

    Ok(summary_data)
}
