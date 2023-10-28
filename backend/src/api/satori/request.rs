use std::env;

use axum::{extract::State, Json};
use reqwest::{Client, StatusCode};
use tokio::try_join;

use crate::api::{cacheable::Cacheable, internal_error, ErrorResponse};

use super::data::{SatoriCurrentCardsResponse, SatoriData, SatoriNewCardsResponse, SatoriStats};

mod current_cards;
mod new_cards;
mod stats;

pub async fn satori_handler(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<Json<SatoriData>, (StatusCode, Json<ErrorResponse>)> {
    let (current_cards, new_cards, stats) = try_join!(
        SatoriCurrentCardsResponse::get(&redis_client),
        SatoriNewCardsResponse::get(&redis_client),
        SatoriStats::get(&redis_client),
    )
    .map_err(internal_error)?;

    let satori_data = SatoriData::new(current_cards, new_cards, stats);

    Ok(Json(satori_data))
}

pub fn satori_client() -> anyhow::Result<Client> {
    let satori_cookie = env::var("SATORI_COOKIE")?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Cookie",
        format!("SessionToken={}", satori_cookie).parse().unwrap(),
    );

    Ok(Client::builder().default_headers(headers).build()?)
}
