// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::fmt;
use std::{
    fs::File,
    future::Future,
    sync::{Arc, Mutex},
};

use crate::settings::{LONG_BREAK_PERIOD, SHORT_BREAK_PERIOD};
use futures_lite::future::race;
use rand::seq::SliceRandom;
use serde_yaml::Value;
use tauri::{AppHandle, Manager, SystemTrayEvent};
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
    let short_break_counter = Arc::new(Mutex::new(0));
    let thread_short_break_counter = Arc::clone(&short_break_counter);
    let (last_short_break_tx, last_short_break_rx) = watch::channel::<Instant>(Instant::now());
    let (force_break_tx, mut force_break_rx) =
        mpsc::unbounded_channel::<(Instant, Option<BreakType>)>();
    rust_i18n::set_locale("ru");

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let app_handle = app.handle();

            tauri::async_runtime::spawn(async move {
                let short_break_period = settings::get_u64(&app_handle, SHORT_BREAK_PERIOD);
                let long_break_period = settings::get_u64(&app_handle, LONG_BREAK_PERIOD);
                let mut interval = interval(Duration::from_secs(short_break_period));
                interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
                interval.tick().await;

                loop {
                    let channel_answer = race(
                        map(interval.tick(), |instant| Some((instant, None))),
                        force_break_rx.recv(),
                    )
                    .await;

                    let Some((instant, possible_break_type)) = channel_answer else {
                        break;
                    };

                    let mut short_breaks_elapsed = thread_short_break_counter.lock().unwrap();

                    let break_type = possible_break_type.unwrap_or({
                        if *short_breaks_elapsed < long_break_period {
                            BreakType::Short
                        } else {
                            BreakType::Long
                        }
                    });

                    if break_type == BreakType::Short {
                        *short_breaks_elapsed += 1;
                        interval.reset();
                        last_short_break_tx
                            .send(instant)
                            .expect("cannot send time of last short break");
                    } else {
                        *short_breaks_elapsed = 0;
                    }

                    let (title, description) =
                        generate_break_idea(&break_type, rust_i18n::locale(), &app_handle);

                    let configuration = BreakConfiguration {
                        duration: 20,
                        break_type,
                        title,
                        description,
                    };
                    if app_handle.get_window("main") == None {
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
                }
            });

            Ok(())
        })
        .system_tray(system_tray::make_tray())
        .on_system_tray_event(move |app_handle, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let last_instant = *last_short_break_rx.borrow();
                system_tray::calculate_time_until_break(&app_handle, last_instant);
                let short_breaks_elapsed = short_break_counter.lock().unwrap();
                system_tray::calculate_short_breaks_until_long_break(
                    &app_handle,
                    *short_breaks_elapsed,
                    settings::get_u64(&app_handle, LONG_BREAK_PERIOD),
                );
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                use system_tray::{
                    QUIT, SETTINGS, SKIP_TO_NEXT_LONG_BREAK, SKIP_TO_NEXT_SHORT_BREAK,
                };

                match id.as_str() {
                    SKIP_TO_NEXT_SHORT_BREAK => {
                        force_break_tx
                            .send((Instant::now(), Some(BreakType::Short)))
                            .expect("cannot send skip to next short break");
                    }
                    SKIP_TO_NEXT_LONG_BREAK => {
                        force_break_tx
                            .send((Instant::now(), Some(BreakType::Long)))
                            .expect("cannot send skip to next long break");
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

#[derive(Debug, PartialEq)]
enum BreakType {
    Short,
    Long,
}

impl fmt::Display for BreakType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BreakType::Short => write!(f, "short"),
            BreakType::Long => write!(f, "long"),
        }
    }
}

struct BreakConfiguration {
    duration: u32,
    break_type: BreakType,
    title: Option<String>,
    description: String,
}

impl BreakConfiguration {
    fn to_query(&self) -> String {
        let mut params = vec![
            format!("duration={}", self.duration),
            format!("break_type={}", self.break_type),
            format!("description={}", Encoded(self.description.clone())),
            format!("lang={}", rust_i18n::locale()),
        ];

        if let Some(title) = &self.title {
            params.push(format!("title={}", Encoded(title.clone())));
        }

        params.join("&")
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

fn generate_break_idea(
    break_type: &BreakType,
    lang: String,
    app: &AppHandle,
) -> (Option<String>, String) {
    let locale = get_translations(&lang, app)
        .expect(format!("cannot parse locale file for language {:?}", lang).as_str());

    let break_idea = pick_random_idea(&locale, break_type)
        .expect("locale files should have keys `break-ideas.short` and `break-ideas.long` with non-empty maps as values");

    if *break_type == BreakType::Short {
        (
            None,
            break_idea
                .as_str()
                .expect("short break ideas must be strings")
                .to_string(),
        )
    } else {
        parse_long_break_idea(break_idea)
            .expect("long break ideas must have keys `title` and `text`")
    }
}

fn get_translations(lang: &String, app: &AppHandle) -> Option<Value> {
    let resource_path = app
        .path_resolver()
        .resolve_resource(format!("locales/{}.yaml", lang))?;
    let file = File::open(&resource_path).ok()?;
    let locale: Value = serde_yaml::from_reader(file).ok()?;

    Some(locale)
}

fn pick_random_idea<'a>(locale: &'a Value, break_type: &BreakType) -> Option<&'a Value> {
    let break_ideas = locale.as_mapping()?.get("break-ideas")?.as_mapping()?;

    Some(
        Vec::from_iter(
            break_ideas
                .get(if *break_type == BreakType::Short {
                    "short"
                } else {
                    "long"
                })?
                .as_mapping()?
                .values(),
        )
        .choose(&mut rand::thread_rng())?,
    )
}

fn parse_long_break_idea(idea: &Value) -> Option<(Option<String>, String)> {
    let title = idea.get("title")?.as_str()?.to_string();
    let text = idea.get("text")?.as_str()?.to_string();

    Some((Some(title), text))
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
