pub mod storage;
pub mod web;

use axum::Router;
use axum::routing::post;
use storage::memory::MemoryStorage;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};


pub fn app() -> Router {
    // In-memory storage
    let storage = MemoryStorage::new();

    // For Postgres, uncomment:
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let pool = sqlx::PgPool::connect(&database_url).await.expect("Failed to connect to Postgres");
    // let storage = storage::postgres::PostgresStorage::new(pool);
    // storage.init_schema().await.expect("Failed to initialize schema");

    let app = web::create_router(storage);
    app
}

pub async fn start_server() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("MYAPP_LOG"))
        .init();

    let app = app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
