use std::{env, vec};

use axum::Json;
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};

use crate::api::{internal_error, ErrorResponse};

use super::data::AnkiData;

pub async fn anki_handler() -> Result<Json<AnkiData>, (StatusCode, Json<ErrorResponse>)> {
    let element_data = get_html_data().await.map_err(internal_error)?;

    let anki_data = AnkiData::new(element_data).map_err(internal_error)?;

    Ok(Json(anki_data))
}

async fn get_html_data() -> anyhow::Result<Vec<String>> {
    let cookie = env::var("ANKIWEB_COOKIE")?;

    let html = Client::new()
        .get("https://ankiweb.net/decks/")
        .header("Accept", "text/html")
        .header("Cookie", format!("ankiweb={}", cookie))
        .send()
        .await?
        .text()
        .await?;

    Ok(parse_html(&html))
}

fn parse_html(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);

    let card_numbers_selector = Selector::parse(".deckDueNumber > font").unwrap();

    let elements = document.select(&card_numbers_selector);

    let mut element_texts: Vec<String> = vec![];

    for element in elements {
        element_texts.push(element.inner_html());
    }

    element_texts
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_with_no_pending_or_new_cards() {
        let html = include_str!("./fixtures/no_pending_reviews_or_cards.html");
        let count_values = parse_html(html);

        assert_eq!(count_values, vec!["0", "0"]);
    }
}
