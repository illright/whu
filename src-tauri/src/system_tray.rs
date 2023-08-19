use std::time::Duration;

use crate::plural_rules;
use crate::settings::{self, SHORT_BREAK_PERIOD};
use rust_i18n::t;
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};
use tokio::time::Instant;

pub const SKIP_TO_NEXT_SHORT_BREAK: &str = "skip_to_next_short_break";
pub const TIME_UNTIL_SHORT_BREAK: &str = "time_until_short_break";
pub const SETTINGS: &str = "settings";
pub const QUIT: &str = "quit";

pub fn make_tray() -> SystemTray {
    let skip_to_next_break = CustomMenuItem::new(
        SKIP_TO_NEXT_SHORT_BREAK.to_string(),
        t!("tray.skip_to_next_break"),
    );
    let quit = CustomMenuItem::new(QUIT.to_string(), t!("tray.quit"));
    let time_until_break = CustomMenuItem::new(
        TIME_UNTIL_SHORT_BREAK.to_string(),
        t!("tray.time_until_break"),
    )
    .disabled();
    let settings = CustomMenuItem::new(SETTINGS.to_string(), t!("tray.settings"));
    let tray_menu = SystemTrayMenu::new()
        .add_item(time_until_break)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(skip_to_next_break)
        .add_item(settings)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

pub fn calculate_time_until_break(app_handle: &tauri::AppHandle, last_break_instant: Instant) {
    let item_handle = app_handle.tray_handle().get_item(TIME_UNTIL_SHORT_BREAK);

    let short_break_period = settings::get_u64(app_handle, SHORT_BREAK_PERIOD);
    let next_break_instant =
        last_break_instant.checked_add(Duration::from_secs(short_break_period));

    if let Some(next_break_instant) = next_break_instant {
        let time_to_next_break = next_break_instant - Instant::now();
        item_handle
            .set_title(format!(
                "{} {} {}",
                t!("tray.next_break"),
                t!("tray.in_about"),
                approximate_duration(time_to_next_break)
            ))
            .expect("cannot set remaining time until next break in tray");
    }
}

fn approximate_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();

    if seconds < 60 {
        String::from(t!("durations.a_minute"))
    } else {
        let ceil_minutes =
            i32::try_from(seconds / 60 + 1).expect("minutes will always fit in an i32");
        format!(
            "{} {}",
            ceil_minutes,
            t!(format!(
                "durations.minutes.{}",
                plural_rules::select("ru", ceil_minutes)
            )
            .as_str())
        )
    }
}
