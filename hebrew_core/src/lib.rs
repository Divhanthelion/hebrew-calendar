//! Hebrew Calendar Core Library
//! 
//! Pure logic for Hebrew-Gregorian calendar conversion and Zmanim calculations.
//! Supports the proleptic fixed Hebrew calendar from 0 AD (1 BCE) to 2050 AD.

pub mod calendar;
pub mod zmanim;
pub mod holidays;
pub mod parsha;

pub use calendar::{DateConverter, HebrewDate, GregorianDate};
pub use zmanim::{ZmanimCalculator, Zmanim, GeoLocation};
pub use holidays::{Holiday, HolidayCalculator};
pub use parsha::{Parsha, ParshaCalculator};

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur in the hebrew_core library
#[derive(Error, Debug, Clone, PartialEq)]
pub enum CalendarError {
    #[error("Date out of supported range (0 AD to 2050 AD): {0}")]
    DateOutOfRange(String),
    
    #[error("Invalid date format: {0}")]
    InvalidDateFormat(String),
    
    #[error("Invalid latitude: {0}. Must be between -90 and 90.")]
    InvalidLatitude(f64),
    
    #[error("Invalid longitude: {0}. Must be between -180 and 180.")]
    InvalidLongitude(f64),
    
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

/// Complete daily calendar data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DailyData {
    /// The Gregorian date
    pub gregorian: GregorianDate,
    /// The Hebrew date
    pub hebrew: HebrewDate,
    /// Parsha for this week (if Shabbat)
    pub parsha: Option<Parsha>,
    /// Holidays on this day
    pub holidays: Vec<Holiday>,
    /// Zmanim for this day (if location provided)
    pub zmanim: Option<Zmanim>,
    /// Candle lighting time (if applicable)
    pub candle_lighting: Option<String>,
    /// Whether this is a Shabbat or Yom Tov
    pub is_yom_tov: bool,
}

/// Main entry point for calendar calculations
pub struct HebrewCalendar;

impl HebrewCalendar {
    /// Calculate complete calendar data for a specific date and location
    pub fn calculate_day(
        date: NaiveDate,
        location: Option<GeoLocation>,
        candle_offset_minutes: i64,
    ) -> Result<DailyData, CalendarError> {
        // Validate date range (0 AD to 2050 AD)
        let min_date = NaiveDate::from_ymd_opt(0, 1, 1)
            .ok_or_else(|| CalendarError::DateOutOfRange("Cannot create min date".to_string()))?;
        let max_date = NaiveDate::from_ymd_opt(2050, 12, 31)
            .ok_or_else(|| CalendarError::DateOutOfRange("Cannot create max date".to_string()))?;
        
        if date < min_date || date > max_date {
            return Err(CalendarError::DateOutOfRange(
                format!("Date {} is outside supported range", date)
            ));
        }
        
        // Convert to Hebrew date
        let hebrew = DateConverter::gregorian_to_hebrew(date)?;
        
        // Get parsha
        let parsha = if hebrew.day_of_week() == 6 { // Saturday (0=Sunday, 6=Saturday)
            Some(ParshaCalculator::get_parsha(&hebrew)?)
        } else {
            None
        };
        
        // Get holidays
        let holidays = HolidayCalculator::get_holidays(&hebrew)?;
        let is_yom_tov = holidays.iter().any(|h| h.is_yom_tov()) || hebrew.day_of_week() == 6; // Shabbat
        
        // Calculate zmanim if location provided
        let (zmanim, candle_lighting) = if let Some(loc) = location {
            let calc = ZmanimCalculator::new(loc);
            let z = calc.calculate(date)?;
            
            // Calculate candle lighting
            let candle = if is_yom_tov || hebrew.day_of_week() == 5 { // Friday (day_of_week 5) or erev Yom Tov
                calc.candle_lighting(&z, candle_offset_minutes)?
            } else {
                None
            };
            
            (Some(z), candle)
        } else {
            (None, None)
        };
        
        Ok(DailyData {
            gregorian: GregorianDate::from(date),
            hebrew,
            parsha,
            holidays,
            zmanim,
            candle_lighting,
            is_yom_tov,
        })
    }
    
    /// Parse an ISO date string (supports year 0 for 1 BCE)
    pub fn parse_date(date_str: &str) -> Result<NaiveDate, CalendarError> {
        // Handle ISO-8601 extended years (e.g., +0000-01-01 or -0005-12-31)
        let date = if date_str.starts_with('+') || date_str.starts_with('-') {
            chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|e| CalendarError::InvalidDateFormat(e.to_string()))?
        } else {
            date_str.parse::<chrono::NaiveDate>()
                .map_err(|e| CalendarError::InvalidDateFormat(e.to_string()))?
        };
        
        Ok(date)
    }
    
    /// Format a date for display, handling year 0
    pub fn format_display_date(date: NaiveDate) -> String {
        let year = date.year();
        let year_display = if year <= 0 {
            format!("{} BCE", 1 - year)
        } else {
            format!("{} AD", year)
        };
        
        format!("{} {}, {}", 
            date.month(),
            date.day(),
            year_display
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_year_zero_boundary() {
        // Year 0 in ISO-8601 is 1 BCE
        let dec_31_bce = NaiveDate::from_ymd_opt(0, 12, 31).unwrap();
        let jan_1_ce = NaiveDate::from_ymd_opt(1, 1, 1).unwrap();
        
        // Verify dates are consecutive
        assert_eq!(dec_31_bce.succ_opt(), Some(jan_1_ce));
        assert_eq!(jan_1_ce.pred_opt(), Some(dec_31_bce));
        
        // Test Hebrew conversion at boundary
        let hebrew_dec_31 = DateConverter::gregorian_to_hebrew(dec_31_bce).unwrap();
        let hebrew_jan_1 = DateConverter::gregorian_to_hebrew(jan_1_ce).unwrap();
        
        // Should be consecutive Hebrew dates
        println!("Dec 31, 1 BCE: {:?}", hebrew_dec_31);
        println!("Jan 1, 1 CE: {:?}", hebrew_jan_1);
    }
    
    #[test]
    fn test_date_parsing() {
        // ISO-8601 extended year format
        let d1 = HebrewCalendar::parse_date("+0000-01-01").unwrap();
        assert_eq!(d1.year(), 0);

        let d2 = HebrewCalendar::parse_date("0001-01-01").unwrap();
        assert_eq!(d2.year(), 1);

        let d3 = HebrewCalendar::parse_date("2024-12-25").unwrap();
        assert_eq!(d3.month(), 12);
        assert_eq!(d3.day(), 25);
    }

    #[test]
    fn test_calculate_day_pesach() {
        // April 23, 2024 = 15 Nisan 5784 = Pesach Day 1
        let date = NaiveDate::from_ymd_opt(2024, 4, 23).unwrap();
        let data = HebrewCalendar::calculate_day(date, None, 18).unwrap();
        assert_eq!(data.hebrew.month, calendar::HebrewMonth::Nisan);
        assert_eq!(data.hebrew.day, 15);
        assert!(data.holidays.contains(&holidays::Holiday::PesachDay1));
    }

    #[test]
    fn test_calculate_day_with_location() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let loc = zmanim::GeoLocation::jerusalem();
        let data = HebrewCalendar::calculate_day(date, Some(loc), 18).unwrap();
        assert!(data.zmanim.is_some(), "With location, zmanim should be present");
    }

    #[test]
    fn test_calculate_day_out_of_range() {
        let date = NaiveDate::from_ymd_opt(2051, 1, 1).unwrap();
        let result = HebrewCalendar::calculate_day(date, None, 18);
        assert!(result.is_err());
        match result.unwrap_err() {
            CalendarError::DateOutOfRange(_) => {},
            other => panic!("Expected DateOutOfRange, got {:?}", other),
        }
    }

    #[test]
    fn test_calculate_day_shabbat_yom_tov() {
        // Sept 16, 2023 = Shabbat, also Rosh Hashanah 5784
        let date = NaiveDate::from_ymd_opt(2023, 9, 16).unwrap();
        let data = HebrewCalendar::calculate_day(date, None, 18).unwrap();
        assert!(data.is_yom_tov, "Shabbat Rosh Hashanah should be yom tov");
    }

    #[test]
    fn test_calculate_day_parsha_on_shabbat() {
        // Oct 14, 2023 = Shabbat = Tishrei 29, 5784 (Bereshit)
        let date = NaiveDate::from_ymd_opt(2023, 10, 14).unwrap();
        let data = HebrewCalendar::calculate_day(date, None, 18).unwrap();
        assert!(data.parsha.is_some(),
            "Shabbat should have parsha (bug fix validation)");
    }

    #[test]
    fn test_calculate_day_parsha_not_on_weekday() {
        // Oct 10, 2023 = Tuesday
        let date = NaiveDate::from_ymd_opt(2023, 10, 10).unwrap();
        let data = HebrewCalendar::calculate_day(date, None, 18).unwrap();
        assert!(data.parsha.is_none(),
            "Tuesday should not have parsha (bug fix validation)");
    }

    #[test]
    fn test_format_display_date_ce() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let display = HebrewCalendar::format_display_date(date);
        assert_eq!(display, "3 15, 2024 AD");
    }

    #[test]
    fn test_format_display_date_bce() {
        let date = NaiveDate::from_ymd_opt(0, 6, 1).unwrap();
        let display = HebrewCalendar::format_display_date(date);
        assert_eq!(display, "6 1, 1 BCE");
    }

    #[test]
    fn test_parse_date_invalid() {
        let result = HebrewCalendar::parse_date("not-a-date");
        assert!(result.is_err());
    }
}
