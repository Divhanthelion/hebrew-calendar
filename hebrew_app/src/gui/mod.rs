//! GUI Module
//! 
//! Tauri-based desktop GUI for the Hebrew calendar application.

use hebrew_core::{DailyData, HebrewCalendar};
use tauri::{Manager, State};

use crate::config::AppConfig;
use std::sync::Mutex;

/// Application state managed by Tauri
pub struct AppState {
    pub config: Mutex<AppConfig>,
}

/// Launch the Tauri GUI
pub fn launch(config: AppConfig) -> anyhow::Result<()> {
    let state = AppState {
        config: Mutex::new(config),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_calendar_data,
            get_zmanim,
            get_date_range,
            get_config,
            update_config,
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
        .map_err(|e| anyhow::anyhow!("Tauri error: {}", e))?;

    Ok(())
}

/// Get complete calendar data for a date
#[tauri::command]
fn get_calendar_data(
    date_str: String,
    lat: Option<f64>,
    long: Option<f64>,
    state: State<AppState>,
) -> Result<DailyData, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    
    // Parse date
    let date = HebrewCalendar::parse_date(&date_str)
        .map_err(|e| e.to_string())?;
    
    // Build location
    let location = if let (Some(lat), Some(long)) = (lat, long) {
        let mut loc = hebrew_core::zmanim::GeoLocation::new(lat, long)
            .map_err(|e| e.to_string())?;
        loc = loc.with_timezone(0); // UTC for now
        Some(loc)
    } else {
        Some(config.default_location.clone())
    };
    
    HebrewCalendar::calculate_day(date, location, config.candle_lighting_offset_minutes)
        .map_err(|e| e.to_string())
}

/// Get zmanim for a date and location
#[tauri::command]
fn get_zmanim(
    date_str: String,
    lat: f64,
    long: f64,
    elevation: Option<f64>,
) -> Result<hebrew_core::zmanim::Zmanim, String> {
    let date = HebrewCalendar::parse_date(&date_str)
        .map_err(|e| e.to_string())?;
    
    let mut loc = hebrew_core::zmanim::GeoLocation::new(lat, long)
        .map_err(|e| e.to_string())?;
    
    if let Some(elev) = elevation {
        loc = loc.with_elevation(elev);
    }
    
    let calc = hebrew_core::zmanim::ZmanimCalculator::new(loc);
    calc.calculate(date)
        .map_err(|e| e.to_string())
}

/// Get calendar data for a date range
#[tauri::command]
fn get_date_range(
    start_str: String,
    end_str: String,
    lat: Option<f64>,
    long: Option<f64>,
    state: State<AppState>,
) -> Result<Vec<DailyData>, String> {
    #[allow(unused_imports)]
    use chrono::NaiveDate;
    
    let config = state.config.lock().map_err(|e| e.to_string())?;
    
    let start = HebrewCalendar::parse_date(&start_str)
        .map_err(|e| e.to_string())?;
    let end = HebrewCalendar::parse_date(&end_str)
        .map_err(|e| e.to_string())?;
    
    if end < start {
        return Err("End date must be after start date".to_string());
    }
    
    // Limit range
    let days = (end - start).num_days();
    if days > 366 {
        return Err("Date range too large (max 366 days)".to_string());
    }
    
    let location = if let (Some(lat), Some(long)) = (lat, long) {
        let mut loc = hebrew_core::zmanim::GeoLocation::new(lat, long)
            .map_err(|e| e.to_string())?;
        loc = loc.with_timezone(0);
        Some(loc)
    } else {
        Some(config.default_location.clone())
    };
    
    let mut results = Vec::with_capacity(days as usize + 1);
    let mut current = start;
    
    while current <= end {
        let data = HebrewCalendar::calculate_day(
            current, 
            location.clone(), 
            config.candle_lighting_offset_minutes
        )
        .map_err(|e| e.to_string())?;
        results.push(data);
        current = current.succ_opt().unwrap();
    }
    
    Ok(results)
}

/// Get current configuration
#[tauri::command]
fn get_config(state: State<AppState>) -> Result<AppConfig, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    Ok(config.clone())
}

/// Update configuration
#[tauri::command]
fn update_config(
    candle_offset: Option<i64>,
    lat: Option<f64>,
    long: Option<f64>,
    elevation: Option<f64>,
    state: State<AppState>,
) -> Result<(), String> {
    let mut config = state.config.lock().map_err(|e| e.to_string())?;
    
    if let Some(offset) = candle_offset {
        config.candle_lighting_offset_minutes = offset;
    }
    
    if let (Some(lat), Some(long)) = (lat, long) {
        let mut loc = hebrew_core::zmanim::GeoLocation::new(lat, long)
            .map_err(|e| e.to_string())?;
        if let Some(elev) = elevation {
            loc = loc.with_elevation(elev);
        }
        config.default_location = loc;
    }
    
    // Save to disk
    config.save().map_err(|e| e.to_string())?;
    
    Ok(())
}
