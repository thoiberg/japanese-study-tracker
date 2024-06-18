use axum::{
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    Json,
};
use chrono::{DateTime, Utc};

pub mod anki;
pub mod bunpro;
mod cacheable;
pub mod satori;
pub mod wanikani;

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    message: String,
}

pub fn internal_error<E>(err: E) -> (StatusCode, Json<ErrorResponse>)
where
    E: Into<anyhow::Error>,
{
    let err = err.into();
    tracing::error!("Error: {}", err.to_string());

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            message: err.to_string(),
        }),
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
