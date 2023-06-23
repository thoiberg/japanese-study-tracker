use axum::{http::StatusCode, Json};

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
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            message: err.to_string(),
        }),
    )
}
