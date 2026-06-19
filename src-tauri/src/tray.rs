use tauri::{AppHandle, SystemTray, SystemTrayMenu, SystemTrayMenuItem, CustomMenuItem};
use tauri::menu::{Menu, MenuItem, Submenu};

pub struct TrayManager {
    app_handle: AppHandle,
}

impl TrayManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn create_tray(&self) -> tauri::Result<()> {
        let show = CustomMenuItem::new("show".to_string(), "打开主窗口");
        let hide = CustomMenuItem::new("hide".to_string(), "隐藏窗口");
        let quit = CustomMenuItem::new("quit".to_string(), "退出");

        let menu = SystemTrayMenu::new()
            .add_item(show)
            .add_item(hide)
            .add_native_item(tauri::SystemTrayMenuItem::Separator)
            .add_item(quit);

        let tray = SystemTray::new().with_menu(menu)?;

        tray.on_menu_event(move |app, event| match event.id.as_str() {
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
        });

        Ok(())
    }
}