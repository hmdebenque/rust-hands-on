use crate::storage::{CreateTodo, Result, StorageError, Todo, TodoStorage};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct MemoryStorage {
    todos: Arc<RwLock<HashMap<Uuid, Todo>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TodoStorage for MemoryStorage {
    async fn create(&self, create: CreateTodo) -> Result<Todo> {
        todo!()
    }

    async fn get(&self, id: Uuid) -> Result<Todo> {
        todo!()
    }
}
