use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "postgres", derive(sqlx::FromRow))]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

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
    async fn list(&self) -> Result<Vec<Todo>>;
    async fn update(&self, id: Uuid, update: UpdateTodo) -> Result<Todo>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

#[cfg(feature = "in_memory")]
pub mod memory;

#[cfg(feature = "postgres")]
pub mod postgres;
