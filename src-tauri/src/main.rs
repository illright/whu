// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{ActivationPolicy, SystemTray};
use tokio::time::{Duration, interval, MissedTickBehavior};

fn main() {
    let tray = SystemTray::new();
    let mut app = tauri::Builder::default()
        .system_tray(tray)
        .setup(|app| {
            let app = app.handle();
            tauri::async_runtime::spawn(async move {
                let mut interval = interval(Duration::from_secs(5 * 60));
                interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
                interval.tick().await;

                loop {
                    interval.tick().await;
                    tauri::WindowBuilder::new(
                        &app,
                        "main",
                        tauri::WindowUrl::App("index.html".into()),
                    )
                    .title("WHU")
                    .build()
                    .expect("cannot build window");
                }
            });

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
