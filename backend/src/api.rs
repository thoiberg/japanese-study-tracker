use axum::{http::StatusCode, Json};

pub mod bunpro;
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
