use async_trait::async_trait;
use uuid::Uuid;
use crate::data_types::{CreateTodo, Todo};

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Todo not found")]
    NotFound,
    #[error("Database error: {0}")]
    Database(String),
}

pub type Result<T> = std::result::Result<T, StorageError>;

#[async_trait]
pub trait TodoStorage: Send + Sync {
    async fn create(&self, todo: CreateTodo) -> Result<Todo>;
    async fn get(&self, id: Uuid) -> Result<Todo>;
}


pub mod memory;
