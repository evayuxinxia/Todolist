use tauri::{AppHandle, Emitter};
use tokio::time::{interval, Duration};
use std::sync::Arc;
use crate::db::Database;
use crate::models::Task;

pub struct Scheduler {
    app_handle: AppHandle,
    db: Arc<Database>,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl Scheduler {
    pub fn new(app_handle: AppHandle, db: Arc<Database>) -> Self {
        Self {
            app_handle,
            db,
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    pub async fn start(&mut self) {
        if self.running.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }

        self.running.store(true, std::sync::atomic::Ordering::Relaxed);

        let db = self.db.clone();
        let app_handle = self.app_handle.clone();
        let running = self.running.clone();

        tokio::spawn(async move {
            let mut task_check_interval = interval(Duration::from_secs(1800));
            let mut pet_check_interval = interval(Duration::from_secs(60));

            loop {
                tokio::select! {
                    _ = task_check_interval.tick() => {
                        if let Ok(tasks) = db.get_upcoming_tasks(3).await {
                            if !tasks.is_empty() {
                                let _ = app_handle.emit("upcoming-tasks", &tasks);
                            }
                        }
                    }
                    _ = pet_check_interval.tick() => {
                        if let Ok(config) = db.get_config().await {
                            if config.pet_enabled {
                                let _ = app_handle.emit("show-pet", ());
                            }
                        }
                    }
                }

                if !running.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, std::sync::atomic::Ordering::Relaxed);
    }
}