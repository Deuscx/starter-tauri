// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;

use app::menu;
use menu::{get_menu, menu_event_handle};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let menu = get_menu();

    let mut tauri_app = tauri::Builder::default();

    #[cfg(not(target_os = "macos"))]
    {
        use menu::{get_system_tray, system_tray_handle};

        let system_tray = get_system_tray(true);
        tauri_app = tauri_app
            .system_tray(system_tray)
            .on_system_tray_event(system_tray_handle);
    }

    tauri_app = tauri_app.menu(menu).on_menu_event(menu_event_handle);

    tauri_app
        .invoke_handler(tauri::generate_handler![greet])
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
