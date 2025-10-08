use axum::{
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::Html,
};
use chrono::{DateTime, Utc};

pub mod anki;
pub mod bunpro;
mod cacheable;
pub mod satori;
pub mod wanikani;

pub type HtmlErrorResponse = (StatusCode, Html<String>);

pub fn internal_error_html<E>(err: E) -> HtmlErrorResponse
where
    E: Into<anyhow::Error>,
{
    let err = err.into();
    tracing::error!("Error: {}", err.to_string());

    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        Html("Something went wrong".to_string()),
    )
}

pub fn add_expiry_header(
    header_map: HeaderMap,
    expiry_times: &[Option<DateTime<Utc>>],
) -> HeaderMap {
    let expires_at = expiry_times.iter().flatten().min();

    let mut header_map = header_map.clone();

    if let Some(expires_at) = expires_at {
        let expiry_header = generate_expiry_header(expires_at);
        header_map.insert(expiry_header.0, expiry_header.1);
    }

    header_map
}

fn generate_expiry_header(expires_at: &DateTime<Utc>) -> (HeaderName, HeaderValue) {
    (
        axum::http::header::EXPIRES,
        axum::http::HeaderValue::from_str(&expires_at.to_rfc2822()).unwrap(),
    )
}

#[cfg(test)]
mod test_super {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_generate_expiry_header() {
        let expires_at = Utc.with_ymd_and_hms(2024, 6, 21, 23, 12, 00).unwrap();
        let (header_name, header_value) = generate_expiry_header(&expires_at);

        assert_eq!(header_name, "expires");
        assert_eq!(header_value, "Fri, 21 Jun 2024 23:12:00 +0000");
    }

    #[test]
    fn test_add_expiry_header_uses_nearest_expiry() {
        let header_map = HeaderMap::new();
        let oldest_year = Utc.with_ymd_and_hms(2023, 6, 21, 23, 12, 00).unwrap();
        let recent_year = Utc.with_ymd_and_hms(2024, 6, 21, 23, 12, 00).unwrap();

        let expiry_times = [Some(recent_year), Some(oldest_year)];

        let header_map = add_expiry_header(header_map, &expiry_times);

        assert_eq!(
            header_map
                .get(axum::http::header::EXPIRES)
                .unwrap()
                .to_str()
                .unwrap(),
            oldest_year.to_rfc2822()
        );
    }

    #[test]
    fn test_add_expiry_header_handles_none() {
        let header_map = HeaderMap::new();
        let recent_year = Utc.with_ymd_and_hms(2024, 6, 21, 23, 12, 00).unwrap();

        let expiry_times = [Some(recent_year), None];

        let header_map = add_expiry_header(header_map, &expiry_times);

        assert_eq!(
            header_map
                .get(axum::http::header::EXPIRES)
                .unwrap()
                .to_str()
                .unwrap(),
            recent_year.to_rfc2822()
        );
    }

    #[test]
    fn test_add_expiry_header_skips_on_empty() {
        let header_map = HeaderMap::new();

        let expiry_times = [None, None];

        let header_map = add_expiry_header(header_map, &expiry_times);

        assert_eq!(header_map.get(axum::http::header::EXPIRES), None);

        let header_map = add_expiry_header(header_map, &[]);

        assert_eq!(header_map.get(axum::http::header::EXPIRES), None);
    }
}
