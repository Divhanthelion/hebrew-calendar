//! Hebrew Calendar Core Library
//! 
//! Pure logic for Hebrew-Gregorian calendar conversion and Zmanim calculations.
//! Supports the proleptic fixed Hebrew calendar from 0 AD (1 BCE) to 2050 AD.

pub mod calendar;
pub mod zmanim;
pub mod holidays;
pub mod parsha;
pub mod molad;
pub mod daf_yomi;

pub use calendar::{DateConverter, HebrewDate, GregorianDate};
pub use zmanim::{ZmanimCalculator, Zmanim, GeoLocation};
pub use holidays::{Holiday, HolidayCalculator};
pub use parsha::{Parsha, ParshaCalculator};
pub use molad::{Molad, MoladCalculator};
pub use daf_yomi::{DafYomi, DafYomiCalculator, Tractate};

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
        let is_shabbat = hebrew.day_of_week() == 6; // Saturday (0=Sunday)
        let parsha = if is_shabbat {
            Some(ParshaCalculator::get_parsha(&hebrew)?)
        } else {
            None
        };
        
        // Get holidays
        let holidays = HolidayCalculator::get_holidays(&hebrew)?;
        let has_yom_tov = holidays.iter().any(|h| h.is_yom_tov());
        // Field means "Shabbat or Yom Tov" for UI highlighting
        let is_yom_tov = has_yom_tov || is_shabbat;
        
        // Calculate zmanim if location provided
        let (zmanim, candle_lighting) = if let Some(loc) = location {
            let calc = ZmanimCalculator::new(loc);
            let z = calc.calculate(date)?;
            
            // Candle lighting on erev Shabbat (Friday) or erev Yom Tov
            let tomorrow_is_yom_tov = if let Some(tomorrow) = date.succ_opt() {
                let next_h = DateConverter::gregorian_to_hebrew(tomorrow)?;
                HolidayCalculator::get_holidays(&next_h)?
                    .iter()
                    .any(|h| h.is_yom_tov())
            } else {
                false
            };
            let candle = if hebrew.day_of_week() == 5 || tomorrow_is_yom_tov {
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
    
    /// The molad of a Hebrew month, as announced on Shabbat Mevarchim.
    ///
    /// Same values as [`MoladCalculator::of_month`], offered here so the molad
    /// sits beside [`HebrewCalendar::calculate_day`] and the holiday and parsha
    /// calculators in the crate's front door. `HebrewMonth::Adar` means plain
    /// Adar in a common year and Adar II in a leap year; `AdarI` in a common
    /// year is an error.
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use hebrew_core::{calendar::HebrewMonth, HebrewCalendar};
    ///
    /// let m = HebrewCalendar::molad(5786, HebrewMonth::Tishrei).unwrap();
    /// assert_eq!(m.gregorian, NaiveDate::from_ymd_opt(2025, 9, 22).unwrap());
    /// assert_eq!((m.hour, m.minute, m.chalakim), (12, 10, 7));
    /// ```
    pub fn molad(year: i32, month: calendar::HebrewMonth) -> Result<Molad, CalendarError> {
        MoladCalculator::of_month(year, month)
    }

    /// The Daf Yomi learned on a Gregorian date, from the cycle that began on
    /// 11 September 1923.
    ///
    /// Same values as [`DafYomiCalculator::for_date`]; dates before the epoch
    /// give [`CalendarError::DateOutOfRange`].
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use hebrew_core::{DafYomi, HebrewCalendar, Tractate};
    ///
    /// let daf: DafYomi =
    ///     HebrewCalendar::daf_yomi(NaiveDate::from_ymd_opt(2020, 1, 5).unwrap()).unwrap();
    /// assert_eq!(daf.tractate, Tractate::Berachos);
    /// assert_eq!(daf.daf, 2, "the first day of the 14th cycle");
    /// assert_eq!(daf.cycle_number, 14);
    /// ```
    pub fn daf_yomi(date: NaiveDate) -> Result<DafYomi, CalendarError> {
        DafYomiCalculator::for_date(date)
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
        assert_eq!(data.parsha, Some(parsha::Parsha::Bereshit));
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

    // ──────────────── public API: molad & Daf Yomi ────────────────
    //
    // The two new features must be reachable from the crate root exactly the
    // way holidays and parsha are: `hebrew_core::Molad`,
    // `hebrew_core::daf_yomi::DafYomi`, and `HebrewCalendar::<helper>`.
    // Everything below names only `hebrew_core::` / `crate::` paths, never a
    // `#[path]`-style module import, so it fails to compile if a re-export is
    // dropped again.

    #[test]
    fn public_api_molad_is_reachable_from_the_crate_root() {
        // Both the module path and the root re-export name the same type.
        let via_module: crate::molad::Molad =
            crate::molad::MoladCalculator::of_tishrei(5786).unwrap();
        let via_root: Molad = crate::MoladCalculator::of_tishrei(5786).unwrap();
        assert_eq!(via_module, via_root);

        // And the front door agrees with the calculator.
        assert_eq!(
            HebrewCalendar::molad(5786, calendar::HebrewMonth::Tishrei).unwrap(),
            via_root
        );

        // Published value: the molad announced for Tishrei 5786.
        assert_eq!(via_root.gregorian, NaiveDate::from_ymd_opt(2025, 9, 22).unwrap());
        assert_eq!((via_root.hour, via_root.minute, via_root.chalakim), (12, 10, 7));
        assert_eq!(via_root.weekday, chrono::Weekday::Mon);

        // It serializes like the other public types, so the API layer can use it.
        let json = serde_json::to_string(&via_root).unwrap();
        assert_eq!(serde_json::from_str::<Molad>(&json).unwrap(), via_root);
    }

    #[test]
    fn public_api_daf_yomi_is_reachable_from_the_crate_root() {
        let via_module: crate::daf_yomi::DafYomi = crate::daf_yomi::DafYomiCalculator::for_date(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        )
        .unwrap();
        let via_root: DafYomi =
            crate::DafYomiCalculator::for_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap();
        assert_eq!(via_module, via_root);
        assert_eq!(
            HebrewCalendar::daf_yomi(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).unwrap(),
            via_root
        );

        // The enum is public too, including its metadata helpers.
        let tractate: Tractate = via_root.tractate;
        assert_eq!(tractate, crate::daf_yomi::Tractate::BavaKama);
        assert_eq!(tractate.name(), "Bava Kama");
        assert_eq!(tractate, Tractate::all()[20]);

        assert_eq!(via_root.daf, 60);
        assert_eq!(via_root.cycle_number, 14);
        let json = serde_json::to_string(&via_root).unwrap();
        assert_eq!(serde_json::from_str::<DafYomi>(&json).unwrap(), via_root);
    }

    #[test]
    fn public_api_molad_helper_covers_every_month_of_several_years() {
        for year in [5785i32, 5786, 5787] {
            let months = crate::MoladCalculator::of_year(year).unwrap();
            assert!(!months.is_empty());
            for m in &months {
                assert_eq!(
                    HebrewCalendar::molad(year, m.hebrew_month).unwrap(),
                    *m,
                    "front door differs from the calculator for {}",
                    year
                );
            }
        }

        // Errors surface through the front door unchanged.
        let err = HebrewCalendar::molad(5786, calendar::HebrewMonth::AdarI).unwrap_err();
        assert!(matches!(err, CalendarError::CalculationError(_)), "{:?}", err);
        assert!(matches!(
            HebrewCalendar::molad(0, calendar::HebrewMonth::Tishrei).unwrap_err(),
            CalendarError::InvalidDateFormat(_)
        ));
    }

    #[test]
    fn public_api_daf_yomi_helper_spans_the_cycle_boundary() {
        // 1923 epoch, the 2702 -> 2711 seam of June 1975, and a modern date.
        let dates = [
            (NaiveDate::from_ymd_opt(1923, 9, 11).unwrap(), Tractate::Berachos, 2, 1u32),
            (NaiveDate::from_ymd_opt(1975, 6, 23).unwrap(), Tractate::Nidah, 73, 7),
            (NaiveDate::from_ymd_opt(1975, 6, 24).unwrap(), Tractate::Berachos, 2, 8),
            (NaiveDate::from_ymd_opt(2005, 3, 2).unwrap(), Tractate::Berachos, 2, 12),
            (NaiveDate::from_ymd_opt(2027, 6, 7).unwrap(), Tractate::Nidah, 73, 14),
        ];
        for (date, tractate, daf, cycle) in dates {
            let via_helper = HebrewCalendar::daf_yomi(date).unwrap();
            assert_eq!(
                (via_helper.tractate, via_helper.daf, via_helper.cycle_number),
                (tractate, daf, cycle),
                "{} via the front door",
                date
            );
            assert_eq!(via_helper, crate::DafYomiCalculator::for_date(date).unwrap());
        }

        // Before the cycle existed, the front door reports the module's error.
        let err = HebrewCalendar::daf_yomi(NaiveDate::from_ymd_opt(1923, 9, 10).unwrap())
            .unwrap_err();
        assert!(matches!(err, CalendarError::DateOutOfRange(_)), "{:?}", err);
    }

    #[test]
    fn public_api_daily_data_is_unchanged_by_the_new_modules() {
        // hebrew_app and src-tauri consume DailyData field by field; the molad
        // and Daf Yomi features are additive, so the struct still has exactly
        // the fields it had, and calculate_day still fills them.
        let date = NaiveDate::from_ymd_opt(2023, 10, 14).unwrap();
        let data = HebrewCalendar::calculate_day(date, None, 18).unwrap();
        let DailyData {
            gregorian,
            hebrew,
            parsha,
            holidays,
            zmanim,
            candle_lighting,
            is_yom_tov,
        } = data;
        assert_eq!(gregorian.iso_string, "2023-10-14");
        assert_eq!(hebrew.year, 5784);
        assert_eq!(parsha, Some(parsha::Parsha::Bereshit));
        assert!(holidays.is_empty());
        assert!(zmanim.is_none());
        assert!(candle_lighting.is_none());
        assert!(is_yom_tov);

        // The new features answer alongside it, without touching it. Both
        // values are published rows: Bava Basra 176 on 18 December 2024 is a
        // row of the 14th-cycle schedule, and the molad of Tishrei 5785 is
        // Thursday 3 October 2024, 3:21 AM + 13 chalakim.
        let daf_date = NaiveDate::from_ymd_opt(2024, 12, 18).unwrap();
        let daf = HebrewCalendar::daf_yomi(daf_date).unwrap();
        assert_eq!(
            (daf.tractate, daf.daf, daf.cycle_number),
            (crate::daf_yomi::Tractate::BavaBasra, 176, 14)
        );
        let molad = HebrewCalendar::molad(5785, calendar::HebrewMonth::Tishrei).unwrap();
        assert_eq!(molad.gregorian, NaiveDate::from_ymd_opt(2024, 10, 3).unwrap());
        assert_eq!((molad.hour, molad.minute, molad.chalakim), (3, 21, 13));
    }
}
