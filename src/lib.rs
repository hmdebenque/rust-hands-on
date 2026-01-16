pub mod storage;
pub mod web;

pub use web::create_router;

use axum::Router;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(feature = "postgres")]
pub async fn app() -> Router {
    use storage::postgres::PostgresStorage;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let storage = PostgresStorage::new(pool);
    storage
        .run_migrations()
        .await
        .expect("Failed to run migrations");

    tracing::info!("Connected to Postgres and ran migrations");
    web::create_router(storage)
}

#[cfg(not(feature = "postgres"))]
pub async fn app() -> Router {
    use storage::memory::MemoryStorage;

    let storage = MemoryStorage::new();
    tracing::info!("Using in-memory storage");
    web::create_router(storage)
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
