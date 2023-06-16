// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{ActivationPolicy, SystemTray, SystemTrayEvent};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let tray = SystemTray::new();
    let mut app = tauri::Builder::default()
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![greet])
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
                    .title("WHU")
                    .build()
                    .expect("cannot build window");
            }
            _ => {}
        })
        .setup(|app| {
            app.hide().expect("cannot hide app");
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(ActivationPolicy::Accessory);

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
