pub mod storage;
pub mod web;
pub mod data_types;

pub use web::create_router;

use axum::Router;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub async fn app() -> Router {
    use crate::storage::memory::MemoryStorage;
    let storage = MemoryStorage::new();
    tracing::info!("Using in-memory storage");
    create_router(storage)
}

pub async fn start_server() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("MYAPP_LOG"))
        .init();

    let app = app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
