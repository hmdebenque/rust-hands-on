use super::{CreateTodo, Result, StorageError, Todo, TodoStorage, UpdateTodo};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresStorage {
    pool: PgPool,
}

impl PostgresStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn init_schema(&self) -> sqlx::Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS todos (
                id UUID PRIMARY KEY,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT FALSE
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
impl TodoStorage for PostgresStorage {
    async fn create(&self, create: CreateTodo) -> Result<Todo> {
        let id = Uuid::new_v4();

        sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (id, title, completed) VALUES ($1, $2, false) RETURNING *",
        )
        .bind(id)
        .bind(create.title)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| StorageError::Database(e.to_string()))
    }

    async fn get(&self, id: Uuid) -> Result<Todo> {
        sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or(StorageError::NotFound)
    }

    async fn list(&self) -> Result<Vec<Todo>> {
        sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY title")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| StorageError::Database(e.to_string()))
    }

    async fn update(&self, id: Uuid, update: UpdateTodo) -> Result<Todo> {
        // Handle different update combinations with fixed queries
        match (update.title, update.completed) {
            (None, None) => self.get(id).await,
            (Some(title), None) => {
                sqlx::query_as::<_, Todo>("UPDATE todos SET title = $1 WHERE id = $2 RETURNING *")
                    .bind(title)
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| StorageError::Database(e.to_string()))?
                    .ok_or(StorageError::NotFound)
            }
            (None, Some(completed)) => sqlx::query_as::<_, Todo>(
                "UPDATE todos SET completed = $1 WHERE id = $2 RETURNING *",
            )
            .bind(completed)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or(StorageError::NotFound),
            (Some(title), Some(completed)) => sqlx::query_as::<_, Todo>(
                "UPDATE todos SET title = $1, completed = $2 WHERE id = $3 RETURNING *",
            )
            .bind(title)
            .bind(completed)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or(StorageError::NotFound),
        }
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            Err(StorageError::NotFound)
        } else {
            Ok(())
        }
    }
}
