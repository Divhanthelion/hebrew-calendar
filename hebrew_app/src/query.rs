//! The requests both front ends answer: the desktop window (Tauri commands)
//! and the web server (HTTP query strings). Each takes the same fields.

use chrono::{Datelike, NaiveDate};
use hebrew_core::{
    CalendarError, DailyData, GeoLocation, HebrewCalendar, HebrewMonth, HolidayInfo, Observance,
    Settings,
};
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;

/// Where and how to calculate. Without `lat` and `lng`, the configured
/// default location is used; `tz` is an IANA zone name.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Place {
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub tz: Option<String>,
    pub elevation: Option<f64>,
    pub name: Option<String>,
    /// Follow Israel's calendar rather than the diaspora's.
    pub israel: Option<bool>,
    /// Minutes before sunset to light candles.
    pub candles: Option<i64>,
    /// No location at all: no zmanim or candle lighting.
    pub nowhere: Option<bool>,
}

impl Place {
    pub fn settings(&self, config: &AppConfig) -> Result<Settings, CalendarError> {
        let location = if self.nowhere.unwrap_or(false) {
            None
        } else if let (Some(lat), Some(lng)) = (self.lat, self.lng) {
            let mut location = GeoLocation::new(lat, lng)?
                .with_timezone(self.tz.as_deref().unwrap_or("UTC"))?
                .with_elevation(self.elevation.unwrap_or(0.0));
            if let Some(name) = &self.name {
                location = location.with_name(name.clone());
            }
            Some(location)
        } else {
            Some(config.location.clone())
        };
        // A request that names its own place gets that place's defaults (18
        // minutes; Israel's calendar in Israel's time zone), not the
        // configured place's.
        let own_place = self.lat.is_some() && self.lng.is_some();
        let observance = match self.israel {
            Some(true) => Observance::Israel,
            Some(false) => Observance::Diaspora,
            None if own_place && self.tz.as_deref() == Some("Asia/Jerusalem") => Observance::Israel,
            None if own_place => Observance::Diaspora,
            None => config.observance,
        };
        let default_candles = if own_place {
            18
        } else {
            config.candle_lighting_minutes
        };
        Ok(Settings {
            location,
            observance,
            candle_lighting_minutes: self.candles.unwrap_or(default_candles).clamp(0, 120),
        })
    }
}

/// Days from `start` to `end`, inclusive.
#[derive(Debug, Clone, Deserialize)]
pub struct DaysQuery {
    pub start: String,
    pub end: String,
    #[serde(default, flatten)]
    pub place: Place,
}

/// One day.
#[derive(Debug, Clone, Deserialize)]
pub struct DayQuery {
    pub date: String,
    #[serde(default, flatten)]
    pub place: Place,
}

/// Holidays from `start` to `end`, inclusive.
#[derive(Debug, Clone, Deserialize)]
pub struct HolidaysQuery {
    pub start: String,
    pub end: String,
    pub israel: Option<bool>,
}

/// A Hebrew date to convert.
#[derive(Debug, Clone, Deserialize)]
pub struct HebrewDateQuery {
    pub year: i32,
    pub month: HebrewMonth,
    pub day: u8,
}

/// A holiday on a date.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DatedHoliday {
    pub date: String,
    pub weekday: String,
    pub hebrew_date: String,
    #[serde(flatten)]
    pub holiday: HolidayInfo,
}

/// A converted date.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Converted {
    pub date: String,
}

pub fn days(q: &DaysQuery, config: &AppConfig) -> Result<Vec<DailyData>, CalendarError> {
    let settings = q.place.settings(config)?;
    HebrewCalendar::range(parse(&q.start)?, parse(&q.end)?, &settings)
}

pub fn day(q: &DayQuery, config: &AppConfig) -> Result<DailyData, CalendarError> {
    HebrewCalendar::day(parse(&q.date)?, &q.place.settings(config)?)
}

pub fn holidays(q: &HolidaysQuery, config: &AppConfig) -> Result<Vec<DatedHoliday>, CalendarError> {
    let (start, end) = (parse(&q.start)?, parse(&q.end)?);
    if !(0..=800).contains(&(end - start).num_days()) {
        return Err(CalendarError::DateOutOfRange(
            "a holiday list must run forwards and cover at most 800 days".to_string(),
        ));
    }
    let observance = match q.israel {
        Some(true) => Observance::Israel,
        Some(false) => Observance::Diaspora,
        None => config.observance,
    };
    HebrewCalendar::holidays_between(start, end, observance)?
        .into_iter()
        .map(|(date, holiday)| {
            let hebrew = hebrew_core::DateConverter::gregorian_to_hebrew(date)?;
            Ok(DatedHoliday {
                date: date.to_string(),
                weekday: date.weekday().to_string(),
                hebrew_date: hebrew.format(),
                holiday,
            })
        })
        .collect()
}

pub fn hebrew_to_gregorian(q: &HebrewDateQuery) -> Result<Converted, CalendarError> {
    let date = HebrewCalendar::hebrew_to_gregorian(q.year, q.month, q.day)?;
    Ok(Converted {
        date: date.to_string(),
    })
}

fn parse(s: &str) -> Result<NaiveDate, CalendarError> {
    HebrewCalendar::parse_date(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_place_overrides_the_default_location() {
        let config = AppConfig::default();
        let place = Place {
            lat: Some(40.7),
            lng: Some(-74.0),
            tz: Some("America/New_York".into()),
            ..Place::default()
        };
        let settings = place.settings(&config).unwrap();
        assert_eq!(settings.location.unwrap().timezone, "America/New_York");
        // Not the configured Jerusalem's customs: New York's.
        assert_eq!(settings.observance, Observance::Diaspora);
        assert_eq!(settings.candle_lighting_minutes, 18);
        let tel_aviv = Place {
            lat: Some(32.08),
            lng: Some(34.78),
            tz: Some("Asia/Jerusalem".into()),
            ..Place::default()
        };
        assert_eq!(
            tel_aviv.settings(&config).unwrap().observance,
            Observance::Israel
        );
        let default = Place::default().settings(&config).unwrap();
        assert_eq!(
            default.candle_lighting_minutes, 40,
            "the configured place keeps its minutes"
        );
    }

    #[test]
    fn nowhere_means_no_location_and_bad_zones_are_errors() {
        let config = AppConfig::default();
        let nowhere = Place {
            nowhere: Some(true),
            ..Place::default()
        };
        assert!(nowhere.settings(&config).unwrap().location.is_none());
        let bad = Place {
            lat: Some(1.0),
            lng: Some(1.0),
            tz: Some("Nowhere/Land".into()),
            ..Place::default()
        };
        assert!(matches!(
            bad.settings(&config),
            Err(CalendarError::InvalidTimezone(_))
        ));
    }

    #[test]
    fn holidays_come_with_their_dates() {
        let q = HolidaysQuery {
            start: "2025-09-20".into(),
            end: "2025-10-20".into(),
            israel: Some(false),
        };
        let list = holidays(&q, &AppConfig::default()).unwrap();
        let rh = list
            .iter()
            .find(|h| h.holiday.name == "Rosh Hashanah I")
            .unwrap();
        assert_eq!(rh.date, "2025-09-23");
        assert_eq!(rh.hebrew_date, "1 Tishrei 5786");
    }
}
