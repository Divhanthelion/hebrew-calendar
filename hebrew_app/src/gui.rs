//! The desktop window (Tauri). The page calls these commands with the same
//! requests the web server takes as query strings.

use hebrew_core::DailyData;
use tauri::State;

use crate::config::AppConfig;
use crate::query::{
    self, Converted, DatedHoliday, DayQuery, DaysQuery, HebrewDateQuery, HolidaysQuery,
};

#[tauri::command]
fn days(query: DaysQuery, config: State<AppConfig>) -> Result<Vec<DailyData>, String> {
    query::days(&query, &config).map_err(|e| e.to_string())
}

#[tauri::command]
fn day(query: DayQuery, config: State<AppConfig>) -> Result<DailyData, String> {
    query::day(&query, &config).map_err(|e| e.to_string())
}

#[tauri::command]
fn holidays(query: HolidaysQuery, config: State<AppConfig>) -> Result<Vec<DatedHoliday>, String> {
    query::holidays(&query, &config).map_err(|e| e.to_string())
}

#[tauri::command]
fn hebrew_to_gregorian(query: HebrewDateQuery) -> Result<Converted, String> {
    query::hebrew_to_gregorian(&query).map_err(|e| e.to_string())
}

pub fn launch(config: AppConfig) -> anyhow::Result<()> {
    tauri::Builder::default()
        .manage(config)
        .invoke_handler(tauri::generate_handler![
            days,
            day,
            holidays,
            hebrew_to_gregorian
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
