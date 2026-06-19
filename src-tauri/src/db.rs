use sqlx::{SqlitePool, Row};
use anyhow::Result;
use chrono::{Utc, NaiveDate};
use crate::models::{Task, NewTask, UpdateTask, AppConfig};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Database {
    pool: Arc<Mutex<Option<SqlitePool>>>,
}

impl Database {
    pub fn new() -> Result<Self> {
        Ok(Self { pool: Arc::new(Mutex::new(None)) })
    }

    pub async fn init(&self) -> Result<()> {
        let database_url = format!("sqlite:todo.db");
        let pool = SqlitePool::connect(&database_url).await?;
        *self.pool.lock().await = Some(pool);

        let pool_ref = self.pool.lock().await.as_ref().unwrap();
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                deadline TEXT,
                priority TEXT NOT NULL DEFAULT '中',
                workload INTEGER NOT NULL DEFAULT 1,
                remark TEXT,
                completed BOOLEAN NOT NULL DEFAULT 0,
                is_remind BOOLEAN NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL
            )
            "#
        )
        .execute(pool_ref)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS app_config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )
            "#
        )
        .execute(pool_ref)
        .await?;

        Ok(())
    }

    async fn get_pool(&self) -> Result<SqlitePool> {
        self.pool.lock().await
            .as_ref()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Database not initialized"))
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>> {
        let pool = self.get_pool().await?;
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks ORDER BY created_at DESC"
        )
        .fetch_all(&pool)
        .await?;
        Ok(tasks)
    }

    pub async fn add_task(&self, task: NewTask) -> Result<Task> {
        let now = Utc::now().to_rfc3339();
        let deadline = if task.deadline.is_empty() {
            String::new()
        } else {
            task.deadline
        };

        let pool = self.get_pool().await?;
        let id = sqlx::query(
            "INSERT INTO tasks (content, deadline, priority, workload, remark, completed, is_remind, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&task.content)
        .bind(&deadline)
        .bind(&task.priority)
        .bind(task.workload)
        .bind(&task.remark)
        .bind(false)
        .bind(true)
        .bind(&now)
        .execute(&pool)
        .await?
        .last_insert_rowid();

        let new_task = Task {
            id,
            content: task.content,
            deadline,
            priority: task.priority,
            workload: task.workload,
            remark: task.remark,
            completed: false,
            is_remind: true,
            created_at: now,
        };

        Ok(new_task)
    }

    pub async fn update_task(&self, id: i64, updates: UpdateTask) -> Result<Task> {
        let mut query = String::from("UPDATE tasks SET ");
        let mut params = Vec::new();
        let mut first = true;

        if let Some(content) = updates.content {
            if !first { query.push_str(", "); }
            query.push_str("content = ?");
            params.push(content);
            first = false;
        }
        if let Some(deadline) = updates.deadline {
            if !first { query.push_str(", "); }
            query.push_str("deadline = ?");
            params.push(deadline);
            first = false;
        }
        if let Some(priority) = updates.priority {
            if !first { query.push_str(", "); }
            query.push_str("priority = ?");
            params.push(priority);
            first = false;
        }
        if let Some(workload) = updates.workload {
            if !first { query.push_str(", "); }
            query.push_str("workload = ?");
            params.push(workload.to_string());
            first = false;
        }
        if let Some(remark) = updates.remark {
            if !first { query.push_str(", "); }
            query.push_str("remark = ?");
            params.push(remark);
            first = false;
        }
        if let Some(completed) = updates.completed {
            if !first { query.push_str(", "); }
            query.push_str("completed = ?");
            params.push(completed.to_string());
            first = false;
        }
        if let Some(is_remind) = updates.is_remind {
            if !first { query.push_str(", "); }
            query.push_str("is_remind = ?");
            params.push(is_remind.to_string());
            first = false;
        }

        query.push_str(" WHERE id = ?");

        let pool = self.get_pool().await?;
        let mut query_builder = sqlx::query(&query);
        for param in params {
            query_builder = query_builder.bind(param);
        }
        query_builder = query_builder.bind(id);

        query_builder.execute(&pool).await?;

        self.get_task_by_id(id).await
    }

    pub async fn delete_task(&self, id: i64) -> Result<()> {
        let pool = self.get_pool().await?;
        sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(&pool)
            .await?;
        Ok(())
    }

    pub async fn get_task_by_id(&self, id: i64) -> Result<Task> {
        let pool = self.get_pool().await?;
        let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await?;
        Ok(task)
    }

    pub async fn get_upcoming_tasks(&self, days: i64) -> Result<Vec<Task>> {
        let today = Utc::now().date_naive();
        let future = today + chrono::Duration::days(days);

        let pool = self.get_pool().await?;
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks WHERE deadline >= ? AND deadline <= ? AND completed = 0 AND is_remind = 1"
        )
        .bind(today.to_string())
        .bind(future.to_string())
        .fetch_all(&pool)
        .await?;

        Ok(tasks)
    }

    pub async fn get_config(&self) -> Result<AppConfig> {
        let pool = self.get_pool().await?;
        let pet_enabled: bool = sqlx::query_scalar("SELECT value FROM app_config WHERE key = 'pet_enabled'")
            .fetch_optional(&pool)
            .await?
            .and_then(|s| s.parse().ok())
            .unwrap_or(true);

        let pet_interval: i64 = sqlx::query_scalar("SELECT value FROM app_config WHERE key = 'pet_interval'")
            .fetch_optional(&pool)
            .await?
            .and_then(|s| s.parse().ok())
            .unwrap_or(60);

        let theme: String = sqlx::query_scalar("SELECT value FROM app_config WHERE key = 'theme'")
            .fetch_optional(&pool)
            .await?
            .unwrap_or_else(|| "light".to_string());

        let auto_start: bool = sqlx::query_scalar("SELECT value FROM app_config WHERE key = 'auto_start'")
            .fetch_optional(&pool)
            .await?
            .and_then(|s| s.parse().ok())
            .unwrap_or(false);

        let api_key: Option<String> = sqlx::query_scalar("SELECT value FROM app_config WHERE key = 'api_key'")
            .fetch_optional(&pool)
            .await?;

        Ok(AppConfig {
            pet_enabled,
            pet_interval,
            theme,
            auto_start,
            api_key,
        })
    }

    pub async fn update_config(&self, config: AppConfig) -> Result<()> {
        let pool = self.get_pool().await?;
        sqlx::query(
            "INSERT OR REPLACE INTO app_config (key, value) VALUES ('pet_enabled', ?)"
        )
        .bind(config.pet_enabled.to_string())
        .execute(&pool)
        .await?;

        sqlx::query(
            "INSERT OR REPLACE INTO app_config (key, value) VALUES ('pet_interval', ?)"
        )
        .bind(config.pet_interval.to_string())
        .execute(&pool)
        .await?;

        sqlx::query(
            "INSERT OR REPLACE INTO app_config (key, value) VALUES ('theme', ?)"
        )
        .bind(&config.theme)
        .execute(&pool)
        .await?;

        sqlx::query(
            "INSERT OR REPLACE INTO app_config (key, value) VALUES ('auto_start', ?)"
        )
        .bind(config.auto_start.to_string())
        .execute(&pool)
        .await?;

        if let Some(api_key) = &config.api_key {
            sqlx::query(
                "INSERT OR REPLACE INTO app_config (key, value) VALUES ('api_key', ?)"
            )
            .bind(api_key)
            .execute(&pool)
            .await?;
        }

        Ok(())
    }
}
