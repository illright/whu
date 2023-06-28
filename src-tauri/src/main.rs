// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::fmt;
use futures_lite::future::race;
use std::future::Future;
use tauri::{ActivationPolicy, SystemTrayEvent};
use tokio::{
    sync::{mpsc, watch},
    time::{interval, Duration, Instant, MissedTickBehavior},
};
use urlencoding::Encoded;

mod system_tray {
    use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};
    pub const SKIP_TO_NEXT_SHORT_BREAK: &str = "skip_to_next_short_break";
    pub const TIME_UNTIL_SHORT_BREAK: &str = "time_until_short_break";
    pub const QUIT: &str = "quit";

    pub fn make_tray() -> SystemTray {
        let skip_to_next_break =
            CustomMenuItem::new(SKIP_TO_NEXT_SHORT_BREAK.to_string(), "Skip to next break");
        let quit = CustomMenuItem::new(QUIT.to_string(), "Quit");
        let time_until_break =
            CustomMenuItem::new(TIME_UNTIL_SHORT_BREAK.to_string(), "Time until break").disabled();
        let tray_menu = SystemTrayMenu::new()
            .add_item(time_until_break)
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(skip_to_next_break)
            .add_item(quit);
        SystemTray::new().with_menu(tray_menu)
    }
}

const BREAK_TITLE: &str = "Trust me, it works";
const BREAK_DESCRIPTION: &str = "Take 5 minutes to breathe deeply and focus on exhaling. It is known to induce a sense of calmness and help you relieve stress.";

fn main() {
    use system_tray::{QUIT, SKIP_TO_NEXT_SHORT_BREAK, TIME_UNTIL_SHORT_BREAK};

    let mut app = tauri::Builder::default()
        .setup(|app| {
            let (last_break_tx, last_break_rx) = watch::channel::<Instant>(Instant::now());
            let (force_break_tx, mut force_break_rx) = mpsc::unbounded_channel::<Instant>();
            let app_handle_1 = app.handle();
            let app_handle_2 = app.handle();

            tauri::async_runtime::spawn(async move {
                let mut interval = interval(Duration::from_secs(5 * 60));
                interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
                interval.tick().await;

                loop {
                    let channel_answer = race(
                        map(interval.tick(), |instant| Some(instant)),
                        force_break_rx.recv(),
                    )
                    .await;

                    match channel_answer {
                        None => {}
                        Some(instant) => {
                            interval.reset();
                            last_break_tx
                                .send(instant)
                                .expect("cannot send time of last break");
                        }
                    }

                    let configuration = BreakConfiguration {
                        duration: 20,
                        break_type: BreakType::Short,
                        title: BREAK_TITLE,
                        description: BREAK_DESCRIPTION,
                    };
                    create_break_window(&app_handle_1, &configuration)
                        .build()
                        .expect(
                            format!(
                                "cannot build break window, parameters: {}",
                                &configuration.to_query()
                            )
                            .as_str(),
                        );
                }
            });

            let tray = system_tray::make_tray();
            tray.on_event(move |event| match event {
                SystemTrayEvent::LeftClick { .. } => {
                    let last_instant = *last_break_rx.borrow();
                    match last_instant.checked_add(Duration::from_secs(5 * 60)) {
                        Some(next_break_instant) => {
                            let time_to_next_break = next_break_instant - Instant::now();
                            let item_handle =
                                app_handle_2.tray_handle().get_item(TIME_UNTIL_SHORT_BREAK);
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
                    SKIP_TO_NEXT_SHORT_BREAK => {
                        force_break_tx
                            .send(Instant::now())
                            .expect("cannot send skip to next break");
                    }
                    QUIT => {
                        app_handle_2.exit(0);
                    }
                    _ => {}
                },
                _ => {}
            })
            .build(app)
            .expect("cannot add tray to the application on setup");

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building Tauri application");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(ActivationPolicy::Accessory);

    app.run(|_app_handle, event| {
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        }
    });
}

enum BreakType {
    Short,
    // Long,
}

impl fmt::Display for BreakType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BreakType::Short => write!(f, "short"),
            // BreakType::Long => write!(f, "long"),
        }
    }
}

struct BreakConfiguration<'a> {
    duration: u32,
    break_type: BreakType,
    title: &'a str,
    description: &'a str,
}

impl<'a> BreakConfiguration<'a> {
    fn to_query(&self) -> String {
        format!(
            "duration={}&break_type={}&title={}&description={}",
            self.duration,
            Encoded(self.break_type.to_string()),
            Encoded(self.title),
            Encoded(self.description)
        )
    }
}

fn create_break_window<'a>(
    app_handle: &'a tauri::AppHandle,
    break_configuration: &BreakConfiguration,
) -> tauri::WindowBuilder<'a> {
    let parameters = break_configuration.to_query();

    tauri::WindowBuilder::new(
        app_handle,
        "main",
        tauri::WindowUrl::App(format!("index.html?{}", parameters.as_str()).into()),
    )
    .title("WHU")
    .fullscreen(true)
    .always_on_top(true)
    .closable(false)
}

async fn map<SourceValue, ResultValue, Mapper>(
    future: impl Future<Output = SourceValue>,
    mapper: Mapper,
) -> ResultValue
where
    Mapper: FnOnce(SourceValue) -> ResultValue,
{
    mapper(future.await)
}
