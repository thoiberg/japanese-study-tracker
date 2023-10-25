use std::env;

use async_trait::async_trait;
use axum::{extract::State, Json};
use chrono::{DateTime, FixedOffset, Utc};
use regex::Regex;
use reqwest::{Client, StatusCode};
use scraper::{ElementRef, Html, Selector};
use tokio::try_join;

use crate::api::{
    cacheable::{CacheKey, Cacheable},
    internal_error,
    satori::data::SatoriHeatLevel,
    ErrorResponse,
};

use super::data::{
    SatoriCurrentCardsResponse, SatoriData, SatoriHeatData, SatoriNewCardsResponse, SatoriStats,
};

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

#[async_trait]
impl Cacheable for SatoriCurrentCardsResponse {
    fn cache_key() -> CacheKey {
        CacheKey::SatoriReviewCards
    }

    fn ttl() -> usize {
        3600
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        get_current_cards().await
    }
}

#[async_trait]
impl Cacheable for SatoriNewCardsResponse {
    fn cache_key() -> CacheKey {
        CacheKey::SatoriNewCards
    }

    fn ttl() -> usize {
        3600
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        get_new_cards().await
    }
}

#[async_trait]
impl Cacheable for SatoriStats {
    fn cache_key() -> CacheKey {
        CacheKey::SatoriStats
    }

    fn ttl() -> usize {
        3600
    }

    async fn api_fetch() -> anyhow::Result<Self> {
        let client = satori_client()?;

        let html = client
            .get("https://www.satorireader.com/dashboard")
            .header(reqwest::header::ACCEPT, "text/html")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let document = Html::parse_document(html.as_str());

        let heatmap_js_selector = Selector::parse("script[type=\"text/javascript\"]").unwrap();
        let elements: Vec<_> = document.select(&heatmap_js_selector).collect();

        if elements.len() != 1 {
            anyhow::bail!(format!(
                "Expected to find 1 element, found {} elements",
                elements.len()
            ))
        }

        let heat_data_json = extract_heat_data_from_js(elements.first().unwrap());

        let heat_data = deserialize_heat_data(&heat_data_json)?;

        let todays_date = date_for_heatmap(None);

        let todays_heat_data = heat_data.iter().find(|hd| hd.date == todays_date);

        let todays_heat_level = match todays_heat_data {
            Some(hd) => hd.heat_level(),
            None => SatoriHeatLevel::Zero,
        };

        Ok(Self {
            heat_level: todays_heat_level,
        })
    }
}

fn date_for_heatmap(date: Option<DateTime<Utc>>) -> String {
    let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap();
    let date = date.unwrap_or(Utc::now()).with_timezone(&jst_offset);

    date.format("%Y-%m-%d").to_string()
}

fn extract_heat_data_from_js(element: &ElementRef) -> String {
    // look for var activityScores = $();
    let re = Regex::new("var activityScores = (.+);").unwrap();

    let beep = element.inner_html();
    let heat_data = re.captures(beep.as_str());

    // TODO: bail if heat_data amount is not 2 (0 is the regex, 1 is the capture group I think???)
    let boop = heat_data.unwrap().get(1).unwrap();

    boop.as_str().to_string()
}

fn deserialize_heat_data(json_data: &str) -> anyhow::Result<Vec<SatoriHeatData>> {
    let heat_data: Vec<SatoriHeatData> = serde_json::from_str(json_data)?;

    Ok(heat_data)
}

async fn get_current_cards() -> anyhow::Result<SatoriCurrentCardsResponse> {
    let client = satori_client()?;

    client
        .get("https://www.satorireader.com/api/studylist/due/count")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await
        .map(|body| serialize_current_cards_response(&body))?
}

fn serialize_current_cards_response(body: &str) -> anyhow::Result<SatoriCurrentCardsResponse> {
    let json_data: SatoriCurrentCardsResponse = serde_json::from_str(body)?;

    Ok(json_data)
}

async fn get_new_cards() -> anyhow::Result<SatoriNewCardsResponse> {
    let client = satori_client()?;

    client
        .get("https://www.satorireader.com/api/studylist/pending-auto-importable/count")
        .send()
        .await?
        .text()
        .await
        .map(|body| serialize_new_cards_response(&body))?
}

fn serialize_new_cards_response(body: &str) -> anyhow::Result<SatoriNewCardsResponse> {
    let json_data: SatoriNewCardsResponse = serde_json::from_str(body)?;

    Ok(json_data)
}

fn satori_client() -> anyhow::Result<Client> {
    let satori_cookie = env::var("SATORI_COOKIE")?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Cookie",
        format!("SessionToken={}", satori_cookie).parse().unwrap(),
    );

    Ok(Client::builder().default_headers(headers).build()?)
}

#[cfg(test)]
mod test_super {
    use chrono::TimeZone;

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

    #[test]
    fn test_new_card_with_pending_cards() {
        let json_string = include_str!("./fixtures/new_cards_with_pending_cards.json");
        let serialized_result = serialize_new_cards_response(json_string);

        assert!(serialized_result.is_ok());
    }

    #[test]
    fn test_new_card_with_no_cards() {
        let json_string = include_str!("./fixtures/new_cards_with_no_cards.json");
        let serialized_result = serialize_new_cards_response(json_string);

        assert!(serialized_result.is_ok());
    }

    #[test]
    fn test_extract_heat_data_from_js() {
        let html = include_str!("./fixtures/dashboard_minimal.html");
        let document = Html::parse_document(html);
        let heatmap_js_selector = Selector::parse("script[type=\"text/javascript\"]").unwrap();
        let elements: Vec<_> = document.select(&heatmap_js_selector).collect();

        let heatmap_data = extract_heat_data_from_js(elements.first().unwrap());
        let expected_data =
            r#"[{"userID":"[REDACTED]","date":"2023-04-18","score":9.39999999999999}]"#;

        assert_eq!(heatmap_data, expected_data);
    }

    #[test]
    fn test_deserialize_heat_data() {
        let html = include_str!("./fixtures/dashboard_minimal.html");
        let document = Html::parse_document(html);
        let heatmap_js_selector = Selector::parse("script[type=\"text/javascript\"]").unwrap();
        let elements: Vec<_> = document.select(&heatmap_js_selector).collect();

        let heatmap_data = extract_heat_data_from_js(elements.first().unwrap());

        let satori_heat_data = deserialize_heat_data(&heatmap_data);

        assert!(satori_heat_data.is_ok());

        let satori_heat_data = satori_heat_data.unwrap();
        assert_eq!(satori_heat_data.len(), 1);

        let first_heat_data = satori_heat_data.first().unwrap();
        assert_eq!(first_heat_data.date, "2023-04-18");
        assert_eq!(first_heat_data.score, 9.39999999999999);
    }

    #[test]
    fn test_date_for_heatmap() {
        let early_date = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2023, 2, 12, 0, 0, 0)
            .unwrap()
            .with_timezone(&Utc);

        let early_date_string = date_for_heatmap(Some(early_date));
        assert_eq!(early_date_string, "2023-02-12");

        let late_date = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2023, 10, 1, 0, 0, 0)
            .unwrap()
            .with_timezone(&Utc);

        let late_date_string = date_for_heatmap(Some(late_date));
        assert_eq!(late_date_string, "2023-10-01");
    }
}
