use crate::web::handlers::health;
use axum::{
    routing::get,
    Router,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health))
}
