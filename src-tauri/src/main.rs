// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures_lite::future::race;
use std::future::Future;
use tauri::{
    ActivationPolicy, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
use tokio::{
    sync::{mpsc, watch},
    time::{interval, Duration, Instant, MissedTickBehavior},
};

fn main() {
    let (tx, rx) = watch::channel::<Instant>(Instant::now());
    let (tx2, mut rx2) = mpsc::unbounded_channel::<Option<Instant>>();
    let skip_to_next_break =
        CustomMenuItem::new("skip_to_next_break".to_string(), "Skip to next break");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let time_until_break =
        CustomMenuItem::new("time_until_break".to_string(), "Time until break").disabled();
    let tray_menu = SystemTrayMenu::new()
        .add_item(time_until_break)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(skip_to_next_break)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);
    let mut app = tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(move |app_handle, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let last_instant = *rx.borrow();
                match last_instant.checked_add(Duration::from_secs(5 * 60)) {
                    Some(next_break_instant) => {
                        let time_to_next_break = next_break_instant - Instant::now();
                        let item_handle = app_handle.tray_handle().get_item("time_until_break");
                        item_handle
                            .set_title(format!(
                                "Time until break: {} seconds",
                                time_to_next_break.as_secs()
                            ))
                            .expect("cannot set remaining time until next break in tray");
                    }
                    None => {}
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "skip_to_next_break" => {
                    tx2.send(None).expect("cannot send skip to next break");
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            let app = app.handle();

            tauri::async_runtime::spawn(async move {
                let mut interval = interval(Duration::from_secs(5 * 60));
                interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
                interval.tick().await;

                loop {
                    match race(wrap_with_options(interval.tick()), rx2.recv())
                        .await
                        .unwrap()
                    {
                        Some(tick) => {
                            tx.send(tick).expect("cannot send last tick");
                        }
                        None => {
                            tx.send(Instant::now()).expect("cannot send last tick");
                            interval.reset();
                        }
                    }
                    tauri::WindowBuilder::new(
                        &app,
                        "main",
                        tauri::WindowUrl::App("index.html".into()),
                    )
                    .title("WHU")
                    .fullscreen(true)
                    .always_on_top(true)
                    .closable(false)
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

async fn wrap_with_options<InnerType>(
    future: impl Future<Output = InnerType>,
) -> Option<Option<InnerType>> {
    Some(Some(future.await))
}
