use std::env;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use reqwest::Client;
use tokio::try_join;

use crate::api::{add_expiry_header, cacheable::Cacheable, internal_error, ErrorResponse};

use super::data::{SatoriCurrentCardsResponse, SatoriData, SatoriNewCardsResponse, SatoriStats};

mod current_cards;
mod new_cards;
mod stats;

pub async fn satori_handler(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<(HeaderMap, Json<SatoriData>), (StatusCode, Json<ErrorResponse>)> {
    let (
        (current_cards, current_cards_expiry),
        (new_cards, new_cards_expiry),
        (stats, stats_expiry),
    ) = try_join!(
        SatoriCurrentCardsResponse::get(&redis_client),
        SatoriNewCardsResponse::get(&redis_client),
        SatoriStats::get(&redis_client),
    )
    .map_err(internal_error)?;

    let satori_data = SatoriData::new(current_cards, new_cards, stats);

    let headers = add_expiry_header(
        HeaderMap::new(),
        &[current_cards_expiry, new_cards_expiry, stats_expiry],
    );

    Ok((headers, Json(satori_data)))
}

pub fn satori_client() -> anyhow::Result<Client> {
    let satori_cookie = env::var("SATORI_COOKIE")?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Cookie",
        format!("SessionToken={satori_cookie}").parse().unwrap(),
    );

    Ok(Client::builder().default_headers(headers).build()?)
}
