use std::env;

use axum::{extract::State, http::HeaderMap, Json};
use reqwest::{Client, StatusCode};
use tokio::try_join;

use crate::api::{cacheable::Cacheable, generate_expiry_header, internal_error, ErrorResponse};

use super::data::{SatoriCurrentCardsResponse, SatoriData, SatoriNewCardsResponse, SatoriStats};

mod current_cards;
mod new_cards;
mod stats;

pub async fn satori_handler(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<(HeaderMap, Json<SatoriData>), (StatusCode, Json<ErrorResponse>)> {
    let (current_cards, new_cards, stats) = try_join!(
        SatoriCurrentCardsResponse::get(&redis_client),
        SatoriNewCardsResponse::get(&redis_client),
        SatoriStats::get(&redis_client),
    )
    .map_err(internal_error)?;

    let satori_data = SatoriData::new(current_cards, new_cards, stats);

    let headers = create_headers();

    Ok((headers, Json(satori_data)))
}

fn create_headers() -> HeaderMap {
    let expires_at = [
        SatoriCurrentCardsResponse::expires_at(),
        SatoriNewCardsResponse::expires_at(),
        SatoriStats::expires_at(),
    ]
    .into_iter()
    .min();

    let mut header_map = HeaderMap::new();
    if let Some(expires_at) = expires_at {
        let expiry_header = generate_expiry_header(&expires_at);
        header_map.insert(expiry_header.0, expiry_header.1);
    }

    header_map
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
