use axum::http::StatusCode;

pub async fn wanikani_handler() -> Result<&'static str, (StatusCode, &'static str)> {
    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Errrr it's still in progress",
    ))
}
