//! Hebrew Calendar Conversion Module
//! 
//! Implements the fixed arithmetic Hebrew calendar (proleptic).
//! Based on the algorithms from "Calendrical Calculations" by Reingold & Dershowitz (4th Edition).
//! 
//! Reference implementation: https://docs.rs/calendrical_calculations

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::CalendarError;

/// Hebrew month enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum HebrewMonth {
    Nisan = 1,
    Iyar = 2,
    Sivan = 3,
    Tammuz = 4,
    Av = 5,
    Elul = 6,
    Tishrei = 7,
    Cheshvan = 8,
    Kislev = 9,
    Teves = 10,
    Shevat = 11,
    Adar = 12,      // Regular Adar (in common years) or Adar II (in leap years)
    AdarI = 13,     // Adar I (in leap years only)
}

impl HebrewMonth {
    pub fn from_number(n: u8, is_leap: bool) -> Result<Self, CalendarError> {
        match (n, is_leap) {
            (1, _) => Ok(HebrewMonth::Nisan),
            (2, _) => Ok(HebrewMonth::Iyar),
            (3, _) => Ok(HebrewMonth::Sivan),
            (4, _) => Ok(HebrewMonth::Tammuz),
            (5, _) => Ok(HebrewMonth::Av),
            (6, _) => Ok(HebrewMonth::Elul),
            (7, _) => Ok(HebrewMonth::Tishrei),
            (8, _) => Ok(HebrewMonth::Cheshvan),
            (9, _) => Ok(HebrewMonth::Kislev),
            (10, _) => Ok(HebrewMonth::Teves),
            (11, _) => Ok(HebrewMonth::Shevat),
            (12, false) => Ok(HebrewMonth::Adar),
            (12, true) => Ok(HebrewMonth::AdarI),   // Month 12 = Adar I in leap years
            (13, true) => Ok(HebrewMonth::Adar),    // Month 13 = Adar II in leap years
            (13, false) => Err(CalendarError::CalculationError(
                format!("Month 13 invalid in common year")
            )),
            _ => Err(CalendarError::CalculationError(
                format!("Invalid Hebrew month number: {}", n)
            )),
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            HebrewMonth::Tishrei => "Tishrei",
            HebrewMonth::Cheshvan => "Cheshvan",
            HebrewMonth::Kislev => "Kislev",
            HebrewMonth::Teves => "Teves",
            HebrewMonth::Shevat => "Shevat",
            HebrewMonth::Adar => "Adar",
            HebrewMonth::AdarI => "Adar I",
            HebrewMonth::Nisan => "Nisan",
            HebrewMonth::Iyar => "Iyar",
            HebrewMonth::Sivan => "Sivan",
            HebrewMonth::Tammuz => "Tammuz",
            HebrewMonth::Av => "Av",
            HebrewMonth::Elul => "Elul",
        }
    }
    
    pub fn to_number(&self, is_leap: bool) -> u8 {
        match (self, is_leap) {
            (HebrewMonth::Nisan, _) => 1,
            (HebrewMonth::Iyar, _) => 2,
            (HebrewMonth::Sivan, _) => 3,
            (HebrewMonth::Tammuz, _) => 4,
            (HebrewMonth::Av, _) => 5,
            (HebrewMonth::Elul, _) => 6,
            (HebrewMonth::Tishrei, _) => 7,
            (HebrewMonth::Cheshvan, _) => 8,
            (HebrewMonth::Kislev, _) => 9,
            (HebrewMonth::Teves, _) => 10,
            (HebrewMonth::Shevat, _) => 11,
            (HebrewMonth::Adar, false) => 12,
            (HebrewMonth::Adar, true) => 13,   // Adar = Adar II in leap years
            (HebrewMonth::AdarI, true) => 12,  // Adar I = month 12 in leap years
            (HebrewMonth::AdarI, false) => 12, // Should not happen, but return 12
        }
    }
}

/// Represents a Hebrew date
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HebrewDate {
    pub year: i32,        // Hebrew year (e.g., 5784)
    pub month: HebrewMonth,
    pub day: u8,
}

impl HebrewDate {
    pub fn new(year: i32, month: HebrewMonth, day: u8) -> Self {
        Self { year, month, day }
    }
    
    /// Format as a human-readable string
    pub fn format(&self) -> String {
        format!("{} {} {}", self.day, self.month.name(), self.year)
    }
    
    /// Get day of week (0 = Sunday, 1 = Monday, ..., 6 = Saturday)
    /// 
    /// Note: R.D. (Rata Die) day 0 = Saturday, December 30, year 0 (1 BCE)
    /// So R.D. % 7 gives: 0=Saturday, 1=Sunday, 2=Monday, ..., 6=Friday
    /// We convert to standard convention: 0=Sunday, 1=Monday, ..., 6=Saturday
    pub fn day_of_week(&self) -> u8 {
        if let Ok(rd) = DateConverter::hebrew_to_rd(*self) {
            // rd % 7: 0=Sat, 1=Sun, 2=Mon, 3=Tue, 4=Wed, 5=Thu, 6=Fri
            // Target:      6     0     1     2     3     4     5
            // Formula: (rd % 7 + 6) % 7 works:
            // - RD 0 (Sat): (0 + 6) % 7 = 6 -> Saturday ✓
            // - RD 1 (Sun): (1 + 6) % 7 = 0 -> Sunday ✓
            // - RD 2 (Mon): (2 + 6) % 7 = 1 -> Monday ✓
            ((rd.rem_euclid(7) + 6).rem_euclid(7)) as u8
        } else {
            0
        }
    }
    
    /// Get the Julian Day Number for this Hebrew date
    pub fn to_julian_day(&self) -> Result<i32, CalendarError> {
        let rd = DateConverter::hebrew_to_rd(*self)?;
        Ok(DateConverter::rd_to_julian_day(rd))
    }
}

/// Represents a Gregorian date for serialization
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GregorianDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub iso_string: String,
    pub display: String,
}

impl From<NaiveDate> for GregorianDate {
    fn from(date: NaiveDate) -> Self {
        let year = date.year();
        let display = if year <= 0 {
            format!("{} {}, {} BCE", date.month(), date.day(), 1 - year)
        } else {
            format!("{} {}, {} AD", date.month(), date.day(), year)
        };
        
        Self {
            year,
            month: date.month() as u8,
            day: date.day() as u8,
            iso_string: date.to_string(),
            display,
        }
    }
}

/// Calendar conversion algorithms
pub struct DateConverter;

impl DateConverter {
    /// Hebrew epoch in R.D. (Rata Die)
    /// From "Calendrical Calculations": The epoch is Monday, October 7, -3761 (Julian)
    /// which corresponds to September 7, -3760 (Gregorian)
    /// RD = -1373426
    const HEBREW_EPOCH_RD: i32 = -1373426;
    
    /// Parts in a day (24 hours * 1080 parts/hour)
    const PARTS_PER_DAY: i64 = 25920;
    
    /// Parts in a lunation (29 days + 12 hours + 793 parts = 29*25920 + 12*1080 + 793 = 765433)
    /// Actually we use the simplified form: 13753 parts = 12 hours + 793 parts
    const PARTS_PER_LUNATION: i64 = 13753;
    
    /// Convert Gregorian date to Hebrew date
    pub fn gregorian_to_hebrew(date: NaiveDate) -> Result<HebrewDate, CalendarError> {
        let rd = Self::gregorian_to_rd(date);
        Self::rd_to_hebrew(rd)
    }
    
    /// Convert Hebrew date to Gregorian date
    pub fn hebrew_to_gregorian(hebrew: HebrewDate) -> Result<NaiveDate, CalendarError> {
        let rd = Self::hebrew_to_rd(hebrew)?;
        Self::rd_to_gregorian(rd)
    }
    
    /// Convert Hebrew date to Julian Day Number
    pub fn hebrew_to_julian_day(hebrew: HebrewDate) -> Result<i32, CalendarError> {
        let rd = Self::hebrew_to_rd(hebrew)?;
        Ok(Self::rd_to_julian_day(rd))
    }
    
    /// Calculate Rosh Hashanah (Hebrew New Year) for a given Hebrew year
    /// Returns the R.D. (Rata Die) date of Tishrei 1
    pub fn rosh_hashanah(year: i32) -> i32 {
        Self::hebrew_new_year(year)
    }
    
    /// Convert Gregorian date to R.D. (days since Jan 1, year 1)
    pub fn gregorian_to_rd(date: NaiveDate) -> i32 {
        let jd = Self::gregorian_to_julian_day(date);
        Self::julian_day_to_rd(jd)
    }
    
    /// Convert R.D. to Gregorian date
    pub fn rd_to_gregorian(rd: i32) -> Result<NaiveDate, CalendarError> {
        let jd = Self::rd_to_julian_day(rd);
        Self::julian_day_to_gregorian(jd)
    }
    
    /// Convert Julian Day to R.D.
    pub fn julian_day_to_rd(jd: i32) -> i32 {
        jd - 1721424
    }
    
    /// Convert R.D. to Julian Day
    pub fn rd_to_julian_day(rd: i32) -> i32 {
        rd + 1721424
    }
    
    /// Convert Gregorian date to Julian Day Number
    fn gregorian_to_julian_day(date: NaiveDate) -> i32 {
        let year = date.year() as i64;
        let month = date.month() as i64;
        let day = date.day() as i64;
        
        let a = (14 - month) / 12;
        let y = year + 4800 - a;
        let m = month + 12 * a - 3;
        
        (day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045) as i32
    }
    
    /// Convert Julian Day Number to Gregorian date
    fn julian_day_to_gregorian(jd: i32) -> Result<NaiveDate, CalendarError> {
        let jd = jd as i64;
        let l = jd + 68569;
        let n = (4 * l) / 146097;
        let l = l - (146097 * n + 3) / 4;
        let i = (4000 * (l + 1)) / 1461001;
        let l = l - (1461 * i) / 4 + 31;
        let j = (80 * l) / 2447;
        let day = (l - (2447 * j) / 80) as i32;
        let l = j / 11;
        let month = (j + 2 - 12 * l) as i32;
        let year = (100 * (n - 49) + i + l) as i32;
        
        NaiveDate::from_ymd_opt(year, month as u32, day as u32)
            .ok_or_else(|| CalendarError::CalculationError(
                format!("Invalid date from JD {}", jd)
            ))
    }
    
    /// Check if a Hebrew year is a leap year
    /// A year is leap if (7*y + 1) mod 19 < 7
    pub fn is_hebrew_leap_year(year: i32) -> bool {
        (7 * year + 1).rem_euclid(19) < 7
    }
    
    /// Get the number of months in a Hebrew year (12 or 13)
    pub fn months_in_hebrew_year(year: i32) -> u8 {
        if Self::is_hebrew_leap_year(year) { 13 } else { 12 }
    }
    
    /// Get the number of days in a Hebrew year
    pub fn days_in_hebrew_year(year: i32) -> u16 {
        let rosh_next = Self::hebrew_new_year(year + 1);
        let rosh_this = Self::hebrew_new_year(year);
        (rosh_next - rosh_this) as u16
    }
    
    /// Determine the year type (deficient, regular, or complete)
    pub fn hebrew_year_type(year: i32) -> YearType {
        let days = Self::days_in_hebrew_year(year);
        let is_leap = Self::is_hebrew_leap_year(year);
        match (days, is_leap) {
            (353, false) => YearType::DeficientCommon,
            (354, false) => YearType::RegularCommon,
            (355, false) => YearType::CompleteCommon,
            (383, true) => YearType::DeficientLeap,
            (384, true) => YearType::RegularLeap,
            (385, true) => YearType::CompleteLeap,
            _ => YearType::RegularCommon,
        }
    }
    
    /// Calculate the number of days elapsed from the epoch to the molad of Tishrei
    /// for the given Hebrew year, with initial postponement adjustment.
    /// Based on the algorithm from "Calendrical Calculations" 4th ed.
    fn hebrew_calendar_elapsed_days(year: i32) -> i64 {
        // Months elapsed from year 1 to year (year-1)
        // = floor((235 * year - 234) / 19)
        let months_elapsed = ((235i64 * year as i64 - 234) / 19) as i64;
        
        // Parts elapsed: the molad of Tishrei year 1 was at 5 hours 204 parts
        // which is 5604 parts after the epoch. The constant 12084 includes
        // this offset plus adjustments for the epoch calculation.
        let parts_elapsed: i64 = 12084 + Self::PARTS_PER_LUNATION * months_elapsed;
        
        // Days elapsed: 29 days per month plus parts converted to days
        let days: i64 = 29 * months_elapsed + parts_elapsed / Self::PARTS_PER_DAY;
        
        // Initial postponement: if the molad falls on Sun, Wed, or Fri,
        // Rosh Hashanah is delayed by 1 day. This is checked by:
        // (3 * (days + 1)) % 7 < 3
        // The day of week is calculated from the molad position.
        if (3 * (days + 1)).rem_euclid(7) < 3 {
            days + 1
        } else {
            days
        }
    }
    
    /// Calculate the year length correction to prevent invalid year lengths
    /// Returns additional days to delay Rosh Hashanah (0, 1, or 2)
    fn hebrew_year_length_correction(year: i32) -> i64 {
        let ny0 = Self::hebrew_calendar_elapsed_days(year - 1);
        let ny1 = Self::hebrew_calendar_elapsed_days(year);
        let ny2 = Self::hebrew_calendar_elapsed_days(year + 1);
        
        if ny2 - ny1 == 356 {
            // Would be a 356-day year (invalid), delay by 2 days
            2
        } else if ny1 - ny0 == 382 {
            // Would follow a 382-day year (invalid), delay by 1 day
            1
        } else {
            0
        }
    }
    
    /// Calculate R.D. of Rosh Hashanah for a given Hebrew year
    fn hebrew_new_year(year: i32) -> i32 {
        let elapsed = Self::hebrew_calendar_elapsed_days(year);
        let correction = Self::hebrew_year_length_correction(year);
        
        (Self::HEBREW_EPOCH_RD as i64 + elapsed + correction) as i32
    }
    
    /// Convert Hebrew date to R.D.
    fn hebrew_to_rd(hebrew: HebrewDate) -> Result<i32, CalendarError> {
        let is_leap = Self::is_hebrew_leap_year(hebrew.year);
        let month_num = hebrew.month.to_number(is_leap);
        
        // Start at Rosh Hashanah of the target year
        let mut rd = Self::hebrew_new_year(hebrew.year) as i64;
        
        // Add days for each month from Tishrei (month 7) to target month
        if month_num >= 7 {
            // We're in the first part of the year (Tishrei through Adar/Adar II)
            for m in 7..month_num {
                rd += Self::days_in_hebrew_month(hebrew.year, m) as i64;
            }
        } else {
            // We're in the second part (Nisan through Elul)
            // First, add all months from Tishrei to end of year
            let months_in_year = Self::months_in_hebrew_year(hebrew.year);
            for m in 7..=months_in_year {
                rd += Self::days_in_hebrew_month(hebrew.year, m) as i64;
            }
            // Then add months from Nisan to target
            for m in 1..month_num {
                rd += Self::days_in_hebrew_month(hebrew.year, m) as i64;
            }
        }
        
        // Add days (day 1 is the first day, so subtract 1)
        rd += (hebrew.day - 1) as i64;
        
        Ok(rd as i32)
    }
    
    /// Convert R.D. to Hebrew date
    fn rd_to_hebrew(rd: i32) -> Result<HebrewDate, CalendarError> {
        let rd_i64 = rd as i64;
        
        // Approximate year
        let mut year = ((rd_i64 - Self::HEBREW_EPOCH_RD as i64) as f64 / 365.25) as i32 + 1;
        year = year.max(1);
        
        // Adjust to correct year
        while rd < Self::hebrew_new_year(year) {
            year -= 1;
        }
        while rd >= Self::hebrew_new_year(year + 1) {
            year += 1;
        }
        
        let is_leap = Self::is_hebrew_leap_year(year);
        let start_of_year = Self::hebrew_new_year(year) as i64;
        let mut days_into_year = rd_i64 - start_of_year;
        
        // Find the month
        let months_in_year = Self::months_in_hebrew_year(year);
        
        // Calculate days in first part of year (Tishrei = month 7 through end)
        let mut days_in_first_part: i64 = 0;
        for m in 7..=months_in_year {
            days_in_first_part += Self::days_in_hebrew_month(year, m) as i64;
        }
        
        let month: u8;
        if days_into_year < days_in_first_part {
            // We're in the first part (Tishrei through Adar/Adar II)
            let mut m = 7u8;
            while days_into_year >= Self::days_in_hebrew_month(year, m) as i64 {
                days_into_year -= Self::days_in_hebrew_month(year, m) as i64;
                m += 1;
            }
            month = m;
        } else {
            // We're in the second part (Nisan through Elul)
            days_into_year -= days_in_first_part;
            let mut m = 1u8;
            while days_into_year >= Self::days_in_hebrew_month(year, m) as i64 {
                days_into_year -= Self::days_in_hebrew_month(year, m) as i64;
                m += 1;
            }
            month = m;
        }
        
        let day = (days_into_year + 1) as u8;
        
        Ok(HebrewDate::new(
            year,
            HebrewMonth::from_number(month, is_leap)?,
            day
        ))
    }
    
    /// Get the number of days in a Hebrew month
    fn days_in_hebrew_month(year: i32, month: u8) -> u8 {
        let year_type = Self::hebrew_year_type(year);
        let is_leap = Self::is_hebrew_leap_year(year);
        
        match month {
            1 => 30,  // Nisan
            2 => 29,  // Iyar
            3 => 30,  // Sivan
            4 => 29,  // Tammuz
            5 => 30,  // Av
            6 => 29,  // Elul
            7 => 30,  // Tishrei
            8 => match year_type {  // Cheshvan
                YearType::CompleteCommon | YearType::CompleteLeap => 30,
                _ => 29,
            },
            9 => match year_type {  // Kislev
                YearType::DeficientCommon | YearType::DeficientLeap => 29,
                _ => 30,
            },
            10 => 29,  // Teves
            11 => 30,  // Shevat
            12 => if is_leap { 30 } else { 29 }, // Adar I (leap) or regular Adar
            13 => 29,  // Adar II (leap years only)
            _ => 30,
        }
    }
}

/// Hebrew year type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YearType {
    DeficientCommon,  // 353 days
    RegularCommon,    // 354 days
    CompleteCommon,   // 355 days
    DeficientLeap,    // 383 days
    RegularLeap,      // 384 days
    CompleteLeap,     // 385 days
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    
    #[test]
    fn test_leap_year_calculation() {
        // Year 5784 is a leap year
        assert!(DateConverter::is_hebrew_leap_year(5784), "5784 should be leap");
        // Year 5783 is not a leap year
        assert!(!DateConverter::is_hebrew_leap_year(5783), "5783 should not be leap");
        // Year 5785 is not a leap year
        assert!(!DateConverter::is_hebrew_leap_year(5785), "5785 should not be leap");
        // Year 5786 is NOT a leap year
        assert!(!DateConverter::is_hebrew_leap_year(5786), "5786 should not be leap");
        // Year 5787 is a leap year
        assert!(DateConverter::is_hebrew_leap_year(5787), "5787 should be leap");
        
        // Verify known leap years in the 19-year cycle
        assert!(DateConverter::is_hebrew_leap_year(3), "Year 3 should be leap");
        assert!(DateConverter::is_hebrew_leap_year(6), "Year 6 should be leap");
        assert!(DateConverter::is_hebrew_leap_year(8), "Year 8 should be leap");
        assert!(DateConverter::is_hebrew_leap_year(19), "Year 19 should be leap");
        assert!(!DateConverter::is_hebrew_leap_year(1), "Year 1 should not be leap");
        assert!(!DateConverter::is_hebrew_leap_year(2), "Year 2 should not be leap");
    }
    
    #[test]
    fn test_rosh_hashanah_5784() {
        // Rosh Hashanah 5784 should be September 16, 2023
        let rd = DateConverter::rosh_hashanah(5784);
        let greg = DateConverter::rd_to_gregorian(rd).unwrap();
        
        assert_eq!(greg.year(), 2023, "Year should be 2023, got {}", greg.year());
        assert_eq!(greg.month(), 9, "Month should be September, got {}", greg.month());
        assert_eq!(greg.day(), 16, "Day should be 16, got {}", greg.day());
    }
    
    #[test]
    fn test_rosh_hashanah_5785() {
        // Rosh Hashanah 5785 should be October 3, 2024
        let rd = DateConverter::rosh_hashanah(5785);
        let greg = DateConverter::rd_to_gregorian(rd).unwrap();
        
        assert_eq!(greg.year(), 2024, "Year should be 2024, got {}", greg.year());
        assert_eq!(greg.month(), 10, "Month should be October, got {}", greg.month());
        assert_eq!(greg.day(), 3, "Day should be 3, got {}", greg.day());
    }
    
    #[test]
    fn test_rosh_hashanah_multiple_years() {
        // Test multiple years to verify the algorithm is correct
        let test_cases = vec![
            (5783, 2022, 9, 26),  // Rosh Hashanah 5783
            (5784, 2023, 9, 16),  // Rosh Hashanah 5784
            (5785, 2024, 10, 3),  // Rosh Hashanah 5785
            (5786, 2025, 9, 23),  // Rosh Hashanah 5786
            (5787, 2026, 9, 12),  // Rosh Hashanah 5787
        ];
        
        for (hebrew_year, exp_year, exp_month, exp_day) in test_cases {
            let rd = DateConverter::rosh_hashanah(hebrew_year);
            let greg = DateConverter::rd_to_gregorian(rd).unwrap();
            
            assert_eq!(greg.year(), exp_year, 
                "Year {}: expected {}-{:02}-{:02}, got {}-{:02}-{:02}",
                hebrew_year, exp_year, exp_month, exp_day, 
                greg.year(), greg.month(), greg.day());
            assert_eq!(greg.month(), exp_month,
                "Year {}: month mismatch", hebrew_year);
            assert_eq!(greg.day(), exp_day,
                "Year {}: day mismatch", hebrew_year);
        }
    }
    
    #[test]
    fn test_gregorian_to_hebrew() {
        // Test: Sept 16, 2023 should be Tishrei 1, 5784
        let sept_16_2023 = NaiveDate::from_ymd_opt(2023, 9, 16).unwrap();
        let hebrew = DateConverter::gregorian_to_hebrew(sept_16_2023).unwrap();
        
        assert_eq!(hebrew.year, 5784, "Year should be 5784, got {}", hebrew.year);
        assert_eq!(hebrew.month, HebrewMonth::Tishrei, "Month should be Tishrei, got {:?}", hebrew.month);
        assert_eq!(hebrew.day, 1, "Day should be 1, got {}", hebrew.day);
        
        // Test: Jan 1, 2024 should be Tevet 20, 5784
        let jan_1_2024 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let hebrew = DateConverter::gregorian_to_hebrew(jan_1_2024).unwrap();
        
        assert_eq!(hebrew.year, 5784, "Year should be 5784");
        assert_eq!(hebrew.month, HebrewMonth::Teves, "Month should be Teves");
        assert_eq!(hebrew.day, 20, "Day should be 20");
    }
    
    #[test]
    fn test_hebrew_to_gregorian() {
        // Tishrei 1, 5784 should be Sept 16, 2023
        let tishrei_1_5784 = HebrewDate::new(5784, HebrewMonth::Tishrei, 1);
        let greg = DateConverter::hebrew_to_gregorian(tishrei_1_5784).unwrap();
        
        assert_eq!(greg.year(), 2023);
        assert_eq!(greg.month(), 9);
        assert_eq!(greg.day(), 16);
    }
    
    #[test]
    fn test_roundtrip_conversion() {
        // Test various dates
        let test_dates = vec![
            (2023, 9, 16),
            (2024, 1, 1),
            (2024, 6, 15),
            (2020, 2, 29), // Leap year
            (2000, 1, 1),
            (1999, 12, 31),
        ];
        
        for (y, m, d) in test_dates {
            let original = NaiveDate::from_ymd_opt(y, m, d).unwrap();
            let hebrew = DateConverter::gregorian_to_hebrew(original).unwrap();
            let back = DateConverter::hebrew_to_gregorian(hebrew).unwrap();
            
            assert_eq!(original, back, 
                "Roundtrip failed for {}-{:02}-{:02}: got {}-{:02}-{:02}",
                y, m, d, back.year(), back.month(), back.day());
        }
    }
    
    #[test]
    fn test_year_types() {
        // 5784 is a leap year with 383 days
        assert!(DateConverter::is_hebrew_leap_year(5784));
        let days_5784 = DateConverter::days_in_hebrew_year(5784);
        assert_eq!(days_5784, 383, "Year 5784 should have 383 days");
        
        // 5783 is a common year
        assert!(!DateConverter::is_hebrew_leap_year(5783));
        let days_5783 = DateConverter::days_in_hebrew_year(5783);
        assert_eq!(days_5783, 355, "Year 5783 should have 355 days");
    }
    
    #[test]
    fn test_leap_year_months() {
        // 5784 is a leap year, should have 13 months
        let months = DateConverter::months_in_hebrew_year(5784);
        assert_eq!(months, 13, "Leap year 5784 should have 13 months");
        
        // In leap year, Adar I is month 12, Adar II is month 13
        let adar1 = DateConverter::days_in_hebrew_month(5784, 12);
        let adar2 = DateConverter::days_in_hebrew_month(5784, 13);
        assert_eq!(adar1, 30, "Adar I should have 30 days");
        assert_eq!(adar2, 29, "Adar II should have 29 days");
    }
    
    #[test]
    fn test_common_year_months() {
        // 5783 is common, should have 12 months
        let months = DateConverter::months_in_hebrew_year(5783);
        assert_eq!(months, 12, "Common year 5783 should have 12 months");
        
        // Regular Adar has 29 days in common year
        let adar = DateConverter::days_in_hebrew_month(5783, 12);
        assert_eq!(adar, 29, "Regular Adar should have 29 days");
    }
    
    #[test]
    fn test_historical_dates() {
        // Test known historical dates
        // Yom Kippur War started October 6, 1973
        // That was Tishrei 10, 5734
        let oct_6_1973 = NaiveDate::from_ymd_opt(1973, 10, 6).unwrap();
        let hebrew = DateConverter::gregorian_to_hebrew(oct_6_1973).unwrap();
        
        assert_eq!(hebrew.year, 5734);
        assert_eq!(hebrew.month, HebrewMonth::Tishrei);
        assert_eq!(hebrew.day, 10);
    }
    
    #[test]
    fn test_day_of_week() {
        // September 16, 2023 was a Saturday (day 6 in our 0=Sunday convention)
        let tishrei_1_5784 = HebrewDate::new(5784, HebrewMonth::Tishrei, 1);
        let dow = tishrei_1_5784.day_of_week();
        assert_eq!(dow, 6, "Rosh Hashanah 5784 should be Saturday (6)");
        
        // Verify by converting to Gregorian
        let greg = DateConverter::hebrew_to_gregorian(tishrei_1_5784).unwrap();
        // In chrono, weekday() returns Mon=0, Tue=1, ..., Sun=6
        // So Saturday should be 5
        assert_eq!(greg.weekday().num_days_from_monday(), 5, "Should be Saturday");
    }
}
