use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i64,
    pub content: String,
    pub deadline: String,
    pub priority: String,
    pub workload: i32,
    pub remark: Option<String>,
    pub completed: bool,
    pub is_remind: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTask {
    pub content: String,
    pub deadline: String,
    pub priority: String,
    pub workload: i32,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTask {
    pub content: Option<String>,
    pub deadline: Option<String>,
    pub priority: Option<String>,
    pub workload: Option<i32>,
    pub remark: Option<String>,
    pub completed: Option<bool>,
    pub is_remind: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub pet_enabled: bool,
    pub pet_interval: i64,
    pub theme: String,
    pub auto_start: bool,
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiTaskResponse {
    pub task_list: Vec<NewTask>,
}