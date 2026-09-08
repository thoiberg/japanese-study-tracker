use askama::Template;
use axum::{extract::State, http::HeaderMap, response::Html};
use tokio::try_join;

use crate::api::{
    add_expiry_header, bunpro::data::BunproActivityStats, cacheable::Cacheable, internal_error,
    HtmlErrorResponse,
};

use super::data::{BunproData, BunproDueStats};

mod activity;
mod client;
mod due;

pub async fn bunpro_handler(
    State(redis_client): State<Option<redis::Client>>,
) -> Result<(HeaderMap, Html<String>), HtmlErrorResponse> {
    let client = client::bunpro_client().await.map_err(internal_error)?;

    let ((due_data, study_queue_expiry), (stats_data, stats_expiry)) = try_join!(
        BunproDueStats::get(&redis_client, Some(&client)),
        BunproActivityStats::get(&redis_client, Some(&client))
    )
    .map_err(internal_error)?;

    let bunpro_data = BunproData::new(due_data, stats_data);

    let headers = add_expiry_header(HeaderMap::new(), &[study_queue_expiry, stats_expiry]);
    let html_string = bunpro_data.render().map_err(internal_error)?;

    Ok((headers, Html(html_string)))
}
