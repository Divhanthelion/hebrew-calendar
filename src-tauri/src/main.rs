#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use hebrew_core::{DailyData, HebrewCalendar};
use std::sync::Mutex;
use tauri::{Manager, State};

/// Application state managed by Tauri
struct AppState {
    default_lat: Mutex<f64>,
    default_long: Mutex<f64>,
    default_tz: Mutex<i32>,
    default_elevation: Mutex<f64>,
    candle_offset: Mutex<i64>,
}

/// Get complete calendar data for a date
#[tauri::command]
fn get_calendar_data(
    date_str: String,
    lat: Option<f64>,
    long: Option<f64>,
    state: State<AppState>,
) -> Result<DailyData, String> {
    let date = HebrewCalendar::parse_date(&date_str)
        .map_err(|e| e.to_string())?;

    let offset = *state.candle_offset.lock().map_err(|e| e.to_string())?;

    let location = if let (Some(lat), Some(long)) = (lat, long) {
        Some(
            hebrew_core::zmanim::GeoLocation::new(lat, long)
                .map_err(|e| e.to_string())?
                .with_timezone(*state.default_tz.lock().map_err(|e| e.to_string())?)
                .with_elevation(*state.default_elevation.lock().map_err(|e| e.to_string())?),
        )
    } else {
        let dlat = *state.default_lat.lock().map_err(|e| e.to_string())?;
        let dlong = *state.default_long.lock().map_err(|e| e.to_string())?;
        let dtz = *state.default_tz.lock().map_err(|e| e.to_string())?;
        let delev = *state.default_elevation.lock().map_err(|e| e.to_string())?;
        Some(
            hebrew_core::zmanim::GeoLocation::new(dlat, dlong)
                .map_err(|e| e.to_string())?
                .with_timezone(dtz)
                .with_elevation(delev),
        )
    };

    HebrewCalendar::calculate_day(date, location, offset)
        .map_err(|e| e.to_string())
}

/// Get zmanim for a date and location
#[tauri::command]
fn get_zmanim(
    date_str: String,
    lat: f64,
    long: f64,
    elevation: Option<f64>,
    state: State<AppState>,
) -> Result<hebrew_core::zmanim::Zmanim, String> {
    let date = HebrewCalendar::parse_date(&date_str)
        .map_err(|e| e.to_string())?;

    let tz = *state.default_tz.lock().map_err(|e| e.to_string())?;
    let mut loc = hebrew_core::zmanim::GeoLocation::new(lat, long)
        .map_err(|e| e.to_string())?
        .with_timezone(tz);

    if let Some(elev) = elevation {
        loc = loc.with_elevation(elev);
    }

    let calc = hebrew_core::zmanim::ZmanimCalculator::new(loc);
    calc.calculate(date)
        .map_err(|e| e.to_string())
}

fn main() {
    let jerusalem = hebrew_core::zmanim::GeoLocation::jerusalem();

    let state = AppState {
        default_lat: Mutex::new(jerusalem.latitude),
        default_long: Mutex::new(jerusalem.longitude),
        default_tz: Mutex::new(jerusalem.timezone_offset_minutes),
        default_elevation: Mutex::new(jerusalem.elevation_meters),
        candle_offset: Mutex::new(18),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_calendar_data,
            get_zmanim,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
