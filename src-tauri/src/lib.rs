use std::sync::{Arc, Mutex};
use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, CustomMenuItem};
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use tokio::sync::RwLock;

mod db;
mod ai_client;
mod scheduler;
mod tray;
mod commands;
mod models;

use db::Database;
use scheduler::Scheduler;
use tray::TrayManager;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
    pub scheduler: Arc<RwLock<Scheduler>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let db = Database::new("todo.db")?;

            let db_clone = db.clone();
            tauri::async_runtime::spawn(async move {
                let _ = db_clone.init().await;
            });

            let app_handle = app.handle().clone();
            let scheduler = Scheduler::new(app_handle.clone(), db.clone());

            let state = AppState {
                db,
                scheduler: Arc::new(RwLock::new(scheduler)),
            };

            app.manage(state);

            let tray_manager = TrayManager::new(app_handle.clone());
            tray_manager.create_tray()?;

            let scheduler_handle = state.scheduler.clone();
            tauri::async_runtime::spawn(async move {
                let mut scheduler = scheduler_handle.write().await;
                scheduler.start().await;
            });

            let window = app.get_webview_window("main").unwrap();
            window.on_window_event(|event| {
                if let tauri::WindowEvent::CloseRequested { .. } = event {
                    window.hide().unwrap();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_all_tasks,
            commands::add_task,
            commands::update_task,
            commands::delete_task,
            commands::ai_parse_task,
            commands::get_config,
            commands::update_config,
            commands::show_pet_window,
            commands::show_reminder_window,
        ])
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}