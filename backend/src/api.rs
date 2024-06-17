use axum::{
    http::{HeaderName, HeaderValue, StatusCode},
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

pub fn generate_expiry_header(expires_at: &DateTime<Utc>) -> (HeaderName, HeaderValue) {
    (
        axum::http::header::EXPIRES,
        axum::http::HeaderValue::from_str(&expires_at.to_rfc2822()).unwrap(),
    )
}
