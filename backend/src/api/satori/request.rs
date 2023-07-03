use std::env;

use axum::Json;
use reqwest::{Client, StatusCode};

use crate::api::{internal_error, ErrorResponse};

use super::data::{SatoriCurrentCardsResponse, SatoriData};

pub async fn satori_handler() -> Result<Json<SatoriData>, (StatusCode, Json<ErrorResponse>)> {
    let current_cards = get_current_cards().await.map_err(internal_error)?;

    let satori_data = SatoriData::new(current_cards);

    Ok(Json(satori_data))
}

async fn get_current_cards() -> anyhow::Result<SatoriCurrentCardsResponse> {
    let satori_cookie = env::var("SATORI_COOKIE")?;

    Client::new()
        .get("https://www.satorireader.com/api/studylist/due/count")
        .header("Cookie", format!("SessionToken={}", satori_cookie))
        .send()
        .await?
        .text()
        .await
        .map(|body| serialize_current_cards_response(&body))?
}

fn serialize_current_cards_response(body: &str) -> anyhow::Result<SatoriCurrentCardsResponse> {
    let json_data: SatoriCurrentCardsResponse = serde_json::from_str(body)?;

    Ok(json_data)
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_current_cards_with_pending_reviews() {
        let json_string = include_str!("./fixtures/current_cards_with_pending_reviews.json");
        let serialize_result = serialize_current_cards_response(json_string);

        assert!(serialize_result.is_ok());
    }

    #[test]
    fn test_current_cards_with_no_reviews() {
        let json_string = include_str!("./fixtures/current_cards_with_no_reviews.json");
        let serialize_result = serialize_current_cards_response(json_string);

        assert!(serialize_result.is_ok());
    }
}
