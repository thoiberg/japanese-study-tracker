use std::{fs, net::SocketAddr};

use axum::{http::StatusCode, response::Html, routing::get, Router};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod api;

use api::wanikani::wanikani_handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "japanese-study-tracker=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // TODO add nest service to serve dist/assets directory
    let app = Router::new()
        .merge(Router::new().nest_service("/assets", ServeDir::new("dist/assets")))
        .route("/", get(root_handler))
        .route("/api/wanikani", get(wanikani_handler));

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::info!("listening on {}", address);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn root_handler() -> Result<Html<String>, (StatusCode, &'static str)> {
    let html_string = fs::read_to_string("./dist/index.html").map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "index file could not be found",
        )
    })?;

    Ok(Html(html_string))
    // Read the index file (return 500 if impossible)
    // pull from the dist dir
    // then on deploy copy the dist dir from Vue into the rust app
    // Router::new().
    // ServeFile::new("assets/index.html")
    // Html(include_str!("dist/assets/index.html"))
}
