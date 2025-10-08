use std::env;

use askama::Template;
use axum::{extract::State, http::HeaderMap, response::Html};
use reqwest::Client;
use tokio::try_join;

use crate::api::{add_expiry_header, cacheable::Cacheable, internal_error_html, HtmlErrorResponse};

use super::data::{SatoriCurrentCardsResponse, SatoriData, SatoriNewCardsResponse, SatoriStats};

mod current_cards;
mod new_cards;
mod stats;

pub async fn satori_htmx_handler(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<(HeaderMap, Html<String>), HtmlErrorResponse> {
    let (
        (current_cards, current_cards_expiry),
        (new_cards, new_cards_expiry),
        (stats, stats_expiry),
    ) = try_join!(
        SatoriCurrentCardsResponse::get(&redis_client),
        SatoriNewCardsResponse::get(&redis_client),
        SatoriStats::get(&redis_client),
    )
    .map_err(internal_error_html)?;

    let satori_data = SatoriData::new(current_cards, new_cards, stats);

    let headers = add_expiry_header(
        HeaderMap::new(),
        &[current_cards_expiry, new_cards_expiry, stats_expiry],
    );

    let html_string = satori_data.render().map_err(internal_error_html)?;

    Ok((headers, Html(html_string)))
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
