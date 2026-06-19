use tauri::{State, Emitter, Listener};
use crate::AppState;
use crate::models::{Task, NewTask, UpdateTask, AppConfig, AiTaskResponse};
use crate::ai_client::AiClient;
use std::sync::MutexGuard;

#[tauri::command]
pub async fn get_all_tasks(state: State<'_, AppState>) -> Result<Vec<Task>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_all_tasks().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_task(task: NewTask, state: State<'_, AppState>) -> Result<Task, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.add_task(task).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_task(id: i64, updates: UpdateTask, state: State<'_, AppState>) -> Result<Task, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.update_task(id, updates).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_task(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.delete_task(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_parse_task(text: String, state: State<'_, AppState>) -> Result<AiTaskResponse, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let config = db.get_config().await.map_err(|e| e.to_string())?;

    let ai_client = AiClient::new(config.api_key);
    let result = ai_client.parse_task_text(&text).await
        .map_err(|e| format!("AI 解析失败: {}", e))?;

    let task_list: Vec<NewTask> = result["taskList"]
        .as_array()
        .ok_or("返回格式错误")?
        .iter()
        .filter_map(|item| {
            Some(NewTask {
                content: item["content"].as_str()?.to_string(),
                deadline: item["deadline"].as_str().unwrap_or("").to_string(),
                priority: item["priority"].as_str().unwrap_or("中").to_string(),
                workload: item["workload"].as_i64().unwrap_or(1) as i32,
                remark: item["remark"].as_str().map(|s| s.to_string()),
            })
        })
        .collect();

    Ok(AiTaskResponse { task_list })
}

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_config().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_config(config: AppConfig, state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.update_config(config).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn show_pet_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("pet") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    } else {
        return Err("宠物窗口未创建".to_string());
    }
    Ok(())
}

#[tauri::command]
pub async fn show_reminder_window(app: tauri::AppHandle, tasks: Vec<Task>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("reminder") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        window.emit("reminder-tasks", tasks).map_err(|e| e.to_string())?;
    } else {
        return Err("提醒窗口未创建".to_string());
    }
    Ok(())
}