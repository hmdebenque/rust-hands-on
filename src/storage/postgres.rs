use crate::{CreateTodo, Result, StorageError, Todo, TodoStorage, UpdateTodo};
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

        sqlx::query_as!(
            Todo,
            "INSERT INTO todos (id, title, completed) VALUES ($1, $2, false) RETURNING *",
            id,
            create.title
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| StorageError::Database(e.to_string()))
    }

    async fn get(&self, id: Uuid) -> Result<Todo> {
        sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or(StorageError::NotFound)
    }

    async fn list(&self) -> Result<Vec<Todo>> {
        sqlx::query_as!(Todo, "SELECT * FROM todos ORDER BY title")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| StorageError::Database(e.to_string()))
    }

    async fn update(&self, id: Uuid, update: UpdateTodo) -> Result<Todo> {
        let mut query = String::from("UPDATE todos SET ");
        let mut updates = Vec::new();
        let mut param_count = 1;

        if update.title.is_some() {
            updates.push(format!("title = ${}", param_count));
            param_count += 1;
        }
        if update.completed.is_some() {
            updates.push(format!("completed = ${}", param_count));
            param_count += 1;
        }

        if updates.is_empty() {
            return self.get(id).await;
        }

        query.push_str(&updates.join(", "));
        query.push_str(&format!(" WHERE id = ${} RETURNING *", param_count));

        let mut q = sqlx::query_as::<_, Todo>(&query);

        if let Some(title) = update.title {
            q = q.bind(title);
        }
        if let Some(completed) = update.completed {
            q = q.bind(completed);
        }
        q = q.bind(id);

        q.fetch_optional(&self.pool)
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or(StorageError::NotFound)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
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
