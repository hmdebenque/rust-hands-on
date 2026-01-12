use crate::storage::{CreateTodo, Result, StorageError, Todo, TodoStorage, UpdateTodo};
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
        let todo = Todo {
            id: Uuid::new_v4(),
            title: create.title,
            completed: false,
        };

        let mut todos = self.todos.write().await;
        todos.insert(todo.id, todo.clone());

        Ok(todo)
    }

    async fn get(&self, id: Uuid) -> Result<Todo> {
        let todos = self.todos.read().await;
        todos.get(&id).cloned().ok_or(StorageError::NotFound)
    }

    async fn list(&self) -> Result<Vec<Todo>> {
        let todos = self.todos.read().await;
        Ok(todos.values().cloned().collect())
    }

    async fn update(&self, id: Uuid, update: UpdateTodo) -> Result<Todo> {
        let mut todos = self.todos.write().await;

        let todo = todos.get_mut(&id).ok_or(StorageError::NotFound)?;

        if let Some(title) = update.title {
            todo.title = title;
        }
        if let Some(completed) = update.completed {
            todo.completed = completed;
        }

        Ok(todo.clone())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut todos = self.todos.write().await;
        todos.remove(&id).ok_or(StorageError::NotFound)?;
        Ok(())
    }
}
