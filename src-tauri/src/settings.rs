use std::path::PathBuf;

use phf::phf_map;
use rust_i18n::t;
use tauri::{AppHandle, Manager};
use urlencoding::Encoded;

const SETTINGS_PATH: &str = "settings.json";
pub const SHORT_BREAK_PERIOD: &str = "short_break_period";

static DEFAULTS_U64: phf::Map<&'static str, u64> = phf_map! {
    "short_break_period" => 5 * 60,
};

pub fn get_u64(app_handle: &AppHandle, key: &str) -> u64 {
    let default = *DEFAULTS_U64.get(key).unwrap_or(&0);

    tauri_plugin_store::with_store(
        app_handle.clone(),
        app_handle.state(),
        PathBuf::from(SETTINGS_PATH),
        |store| {
            store.get(key).and_then(|json| json.as_u64()).ok_or(
                tauri_plugin_store::Error::NotFound(PathBuf::from(SETTINGS_PATH)),
            )
        },
    )
    .unwrap_or(default)
}

pub fn create_window<'a>(
    app_handle: &'a tauri::AppHandle,
    lang: String,
) -> tauri::WindowBuilder<'a> {
    tauri::WindowBuilder::new(
        app_handle,
        "settings",
        tauri::WindowUrl::App(
            format!(
                "settings.html?lang={}&path={}",
                lang,
                Encoded(SETTINGS_PATH)
            )
            .into(),
        ),
    )
    .title(format!("{} — WHU", t!("settings.title")))
}
