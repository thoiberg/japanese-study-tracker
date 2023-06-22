use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "japanese-study-tracker=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/", get(root_handler));

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::info!("listening on {}", address);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn root_handler() -> Html<&'static str> {
    Html(include_str!("templates/index.html"))
}
