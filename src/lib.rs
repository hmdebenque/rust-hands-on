pub mod storage;
pub mod web;
pub mod data_types;

pub use web::create_router;

use axum::Router;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub async fn app_postgres(database_url: &str) -> Router {
    use storage::postgres::PostgresStorage;

    let pool = sqlx::PgPool::connect(database_url)
        .await
        .expect("Failed to connect to Postgres");

    let storage = PostgresStorage::new(pool);
    storage
        .run_migrations()
        .await
        .expect("Failed to run migrations");

    tracing::info!("Connected to Postgres and ran migrations");
    create_router(storage)
}

pub async fn app_in_memory() -> Router {
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

    let database_url = std::env::var("DATABASE_URL");
    
    let app = if database_url.is_ok() {
        let url = database_url.unwrap();
        let url_str = url.as_str();
        tracing::info!("Starting with PostGreSQL DB at {url_str}");
        app_postgres(url_str).await
    } else {
        tracing::info!("Starting with in-memory storage.");
        app_in_memory().await
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
