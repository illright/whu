use std::path::PathBuf;

use tauri::{AppHandle, Manager};
use urlencoding::Encoded;

const SETTINGS_PATH: &str = "settings.json";
pub const SHORT_BREAK_PERIOD: &str = "short_break_period";

pub fn get_u64(app_handle: &AppHandle, key: &str, default: u64) -> u64 {
    tauri_plugin_store::with_store(
        app_handle.clone(),
        app_handle.state(),
        PathBuf::from("settings.json"),
        |store| {
            store.get(key).and_then(|json| json.as_u64()).ok_or(
                tauri_plugin_store::Error::NotFound(PathBuf::from("settings.json")),
            )
        },
    )
    .unwrap_or(default)
}

pub fn create_window<'a>(app_handle: &'a tauri::AppHandle) -> tauri::WindowBuilder<'a> {
    tauri::WindowBuilder::new(
        app_handle,
        "settings",
        tauri::WindowUrl::App(format!("settings.html?path={}", Encoded(SETTINGS_PATH)).into()),
    )
    .title("Settings — WHU")
}
