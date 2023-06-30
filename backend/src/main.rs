use std::{fs, net::SocketAddr};

use axum::{http::StatusCode, response::Html, routing::get, Router};
use tokio::signal;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod api;

use api::wanikani::wanikani_handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "japanese_study_tracker_backend=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .merge(Router::new().nest_service("/assets", ServeDir::new("dist/assets")))
        .route("/", get(root_handler))
        .route("/api/wanikani", get(wanikani_handler));

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::info!("listening on {}", address);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
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
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}
