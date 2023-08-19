// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::fmt;
use std::{fs::File, future::Future};

use crate::settings::SHORT_BREAK_PERIOD;
use futures_lite::future::race;
use rand::seq::SliceRandom;
use serde_yaml::Value;
use tauri::{AppHandle, SystemTrayEvent};
use tauri_plugin_store;
use tokio::{
    sync::{mpsc, watch},
    time::{interval, Duration, Instant, MissedTickBehavior},
};
use urlencoding::Encoded;

rust_i18n::i18n!("locales");

mod plural_rules;
mod settings;
mod system_tray;

fn main() {
    let (last_break_tx, last_break_rx) = watch::channel::<Instant>(Instant::now());
    let (force_break_tx, mut force_break_rx) = mpsc::unbounded_channel::<Instant>();
    rust_i18n::set_locale("ru");

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let app_handle = app.handle();

            tauri::async_runtime::spawn(async move {
                let short_break_period = settings::get_u64(&app_handle, SHORT_BREAK_PERIOD);
                let mut interval = interval(Duration::from_secs(short_break_period));
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

                    let (title, description) =
                        generate_break_idea(rust_i18n::locale(), &app_handle)
                            .expect("why would generating a break idea ever fail, right?");

                    let configuration = BreakConfiguration {
                        duration: 20,
                        break_type: BreakType::Short,
                        title,
                        description,
                    };
                    create_break_window(&app_handle, &configuration)
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

            Ok(())
        })
        .system_tray(system_tray::make_tray())
        .on_system_tray_event(move |app_handle, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let last_instant = *last_break_rx.borrow();
                system_tray::calculate_time_until_break(&app_handle, last_instant);
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                use system_tray::{QUIT, SETTINGS, SKIP_TO_NEXT_SHORT_BREAK};

                match id.as_str() {
                    SKIP_TO_NEXT_SHORT_BREAK => {
                        force_break_tx
                            .send(Instant::now())
                            .expect("cannot send skip to next break");
                    }
                    QUIT => {
                        app_handle.exit(0);
                    }
                    SETTINGS => {
                        settings::create_window(app_handle, rust_i18n::locale())
                            .build()
                            .expect("cannot build settings window");
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building Tauri application");

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

struct BreakConfiguration {
    duration: u32,
    break_type: BreakType,
    title: String,
    description: String,
}

impl BreakConfiguration {
    fn to_query(&self) -> String {
        format!(
            "duration={}&break_type={}&title={}&description={}&lang={}",
            self.duration,
            Encoded(self.break_type.to_string()),
            Encoded(self.title.clone()),
            Encoded(self.description.clone()),
            rust_i18n::locale(),
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
}

fn generate_break_idea(lang: String, app: &AppHandle) -> Option<(String, String)> {
    let resource_path = app
        .path_resolver()
        .resolve_resource(format!("locales/{}.yaml", lang))?;
    let file = File::open(&resource_path).ok()?;
    let en: Value = serde_yaml::from_reader(file).ok()?;
    let break_ideas = en.as_mapping()?.get("break-ideas")?.as_mapping()?;
    let long_break_ideas = Vec::from_iter(break_ideas.get("long")?.as_mapping()?.values());

    let random_long_break_idea =
        (long_break_ideas.choose(&mut rand::thread_rng())?).as_mapping()?;

    Some((
        random_long_break_idea.get("title")?.as_str()?.to_string(),
        random_long_break_idea.get("text")?.as_str()?.to_string(),
    ))
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
