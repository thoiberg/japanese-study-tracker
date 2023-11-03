use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, Utc};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::api::{
    cacheable::{CacheKey, Cacheable},
    satori::{
        data::{SatoriHeatData, SatoriHeatLevel, SatoriStats},
        request::satori_client,
    },
};

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

        let heat_data_json = extract_heat_data_from_js(elements.first().unwrap())?;

        let todays_heat_level = todays_heat_level(heat_data_json, None);

        Ok(Self {
            heat_level: todays_heat_level,
        })
    }
}

fn date_for_heatmap(date: Option<DateTime<Utc>>) -> String {
    let date = date
        .unwrap_or(Utc::now())
        .with_timezone(&chrono_tz::Asia::Tokyo);

    date.format("%Y-%m-%d").to_string()
}

fn extract_heat_data_from_js(element: &ElementRef) -> anyhow::Result<String> {
    let re = Regex::new("var activityScores = (.+);")?;

    let javascript = element.inner_html();
    let heat_data = re
        .captures(javascript.as_str())
        .and_then(|captures| captures.get(1))
        .ok_or(anyhow!("Unable to find heat data json"))?;

    Ok(heat_data.as_str().to_string())
}

fn deserialize_heat_data(json_data: &str) -> anyhow::Result<Vec<SatoriHeatData>> {
    let heat_data: Vec<SatoriHeatData> = serde_json::from_str(json_data)?;

    Ok(heat_data)
}

fn todays_heat_level(heat_data_json: String, date: Option<DateTime<Utc>>) -> SatoriHeatLevel {
    let todays_date = date_for_heatmap(date);

    deserialize_heat_data(&heat_data_json)
        .ok()
        .and_then(|heat_data| heat_data.into_iter().find(|hd| hd.date == todays_date))
        .map(|today| today.heat_level())
        .unwrap_or(SatoriHeatLevel::Zero)
}

#[cfg(test)]
mod test_super {
    use chrono::TimeZone;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_extract_heat_data_from_js() {
        let html = include_str!("../fixtures/dashboard_minimal.html");
        let document = Html::parse_document(html);
        let heatmap_js_selector = Selector::parse("script[type=\"text/javascript\"]").unwrap();
        let elements: Vec<_> = document.select(&heatmap_js_selector).collect();

        let heatmap_data = extract_heat_data_from_js(elements.first().unwrap());
        let expected_data =
            r#"[{"userID":"[REDACTED]","date":"2023-04-18","score":9.39999999999999}]"#;

        assert!(heatmap_data.is_ok());
        assert_eq!(heatmap_data.unwrap(), expected_data);
    }

    #[test]
    fn test_todays_heat_level_with_day_defined() {
        let heat_data_json = String::from(
            r#"[{"userID":"[REDACTED]","date":"2023-04-18","score":9.39999999999999}]"#,
        );

        let date = Some(DateTime::<Utc>::from_str("2023-04-18 00:00:00+00:00").unwrap());
        let heat_level = todays_heat_level(heat_data_json, date);

        assert_eq!(heat_level, SatoriHeatLevel::Four);
    }

    #[test]
    fn test_todays_heat_level_with_day_missing() {
        let heat_data_json = String::from(
            r#"[{"userID":"[REDACTED]","date":"2023-04-18","score":9.39999999999999}]"#,
        );

        let date = Some(DateTime::<Utc>::from_str("2023-04-19 00:00:00+00:00").unwrap());
        let heat_level = todays_heat_level(heat_data_json, date);

        assert_eq!(heat_level, SatoriHeatLevel::Zero);
    }

    #[test]
    fn test_deserialize_heat_data() {
        let html = include_str!("../fixtures/dashboard_minimal.html");
        let document = Html::parse_document(html);
        let heatmap_js_selector = Selector::parse("script[type=\"text/javascript\"]").unwrap();
        let elements: Vec<_> = document.select(&heatmap_js_selector).collect();

        let heatmap_data = extract_heat_data_from_js(elements.first().unwrap());

        let satori_heat_data = deserialize_heat_data(&heatmap_data.unwrap());

        assert!(satori_heat_data.is_ok());

        let satori_heat_data = satori_heat_data.unwrap();
        assert_eq!(satori_heat_data.len(), 1);

        let first_heat_data = satori_heat_data.first().unwrap();
        assert_eq!(first_heat_data.date, "2023-04-18");
        assert_eq!(first_heat_data.score, 9.39999999999999);
    }

    #[test]
    fn test_date_for_heatmap() {
        let early_date = chrono_tz::Asia::Tokyo
            .with_ymd_and_hms(2023, 2, 12, 0, 0, 0)
            .unwrap()
            .with_timezone(&Utc);

        let early_date_string = date_for_heatmap(Some(early_date));
        assert_eq!(early_date_string, "2023-02-12");

        let late_date = chrono_tz::Asia::Tokyo
            .with_ymd_and_hms(2023, 10, 1, 0, 0, 0)
            .unwrap()
            .with_timezone(&Utc);

        let late_date_string = date_for_heatmap(Some(late_date));
        assert_eq!(late_date_string, "2023-10-01");
    }
}
