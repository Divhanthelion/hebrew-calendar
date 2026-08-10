//! Hebrew Calendar Conversion Module
//! 
//! Implements the fixed arithmetic Hebrew calendar (proleptic).
//! Based on the algorithms from "Calendrical Calculations" by Reingold & Dershowitz (4th Edition).
//! Reference implementation: https://github.com/unicode-org/icu4x

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
///
/// Implements the fixed arithmetic Hebrew calendar from
/// "Calendrical Calculations" by Reingold & Dershowitz, 4th Edition.
///
/// The postponement rules (dehiyyot) are encapsulated in two functions:
/// 1. `hebrew_calendar_elapsed_days` — applies the Lo ADU Rosh rule
/// 2. `hebrew_year_length_correction` — applies Molad Zaken, Gatarad,
///    and Betutakfot implicitly via year-length constraints
pub struct DateConverter;

impl DateConverter {
    /// Hebrew epoch in R.D. (Rata Die).
    /// From "Calendrical Calculations" (4th ed, p. 119):
    /// Monday, October 7, -3761 (Julian) = September 7, -3760 (Gregorian).
    /// RD = -1373426.
    const HEBREW_EPOCH_RD: i32 = -1373426;
    
    /// Parts in a day (24 hours × 1080 parts/hour)
    const PARTS_PER_DAY: i64 = 25920;
    
    /// Fractional parts in a mean synodic month beyond 29 full days.
    /// A mean synodic month = 29d + 12h + 793p = 765,433 parts total.
    /// This constant stores only the fractional portion: 12×1080 + 793 = 13,753.
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
    
    /// Convert Gregorian date to R.D.
    pub fn gregorian_to_rd(date: NaiveDate) -> i32 {
        Self::julian_day_to_rd(Self::gregorian_to_julian_day(date))
    }
    
    /// Convert R.D. to Gregorian date
    pub fn rd_to_gregorian(rd: i32) -> Result<NaiveDate, CalendarError> {
        Self::julian_day_to_gregorian(Self::rd_to_julian_day(rd))
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
    
    /// Check if a Hebrew year is a leap year.
    /// A year is leap if (7×y + 1) mod 19 < 7.
    pub fn is_hebrew_leap_year(year: i32) -> bool {
        (7 * year + 1).rem_euclid(19) < 7
    }
    
    /// Get the number of months in a Hebrew year (12 or 13)
    pub fn months_in_hebrew_year(year: i32) -> u8 {
        if Self::is_hebrew_leap_year(year) { 13 } else { 12 }
    }
    
    /// Get the number of days in a Hebrew year
    pub fn days_in_hebrew_year(year: i32) -> u16 {
        (Self::hebrew_new_year(year + 1) - Self::hebrew_new_year(year)) as u16
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
    
    // ──────────────── Hebrew New Year (Rosh Hashanah) ────────────────
    
    /// Number of days from the epoch to the molad of Tishrei for `year`,
    /// with the Lo ADU Rosh postponement applied.
    ///
    /// "Lo ADU Rosh": Rosh Hashanah cannot fall on Sunday, Wednesday, or Friday.
    /// The check `(3×(days+1)) mod 7 < 3` detects these days in the internal
    /// day-of-week convention where day 0 at the epoch is Monday.
    ///
    /// Lisp code reference:
    /// <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2261>
    fn hebrew_calendar_elapsed_days(year: i32) -> i32 {
        // Months elapsed from year 1 to `year`:
        // floor((235 × year − 234) / 19)
        let months_elapsed = ((235i64 * year as i64 - 234) / 19) as i64;
        
        // Parts elapsed: the molad of Tishrei year 1 was at 5h 204p
        // after the epoch (Monday 5:11:20 AM in parts).
        // 12084 = 5×1080 + 204 + epoch alignment constant.
        let parts_elapsed: i64 = 12084 + Self::PARTS_PER_LUNATION * months_elapsed;
        
        // Days: 29 full days per lunation plus whole days from the parts
        let days: i64 = 29 * months_elapsed + parts_elapsed / Self::PARTS_PER_DAY;
        
        // Dehiyyah: Lo ADU Rosh
        // Postpone by 1 day if Rosh Hashanah would fall on Sun, Wed, or Fri
        if (3 * (days + 1)).rem_euclid(7) < 3 {
            days as i32 + 1
        } else {
            days as i32
        }
    }
    
    /// Year-length correction.
    ///
    /// The remaining three dehiyyot (Molad Zaken, Gatarad, Betutakfot) are
    /// handled implicitly by checking for invalid year lengths:
    ///
    /// - If the year would be 356 days, postpone Rosh Hashanah by 2 days.
    ///   (356 > 355 max for common years, > 385 for leap — always invalid.)
    /// - If the previous year was 382 days, postpone by 1 day.
    ///   (382 is between common max 355 and leap min 383 — always invalid.)
    ///
    /// Lisp code reference:
    /// <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2301>
    fn hebrew_year_length_correction(year: i32) -> u8 {
        let ny0 = Self::hebrew_calendar_elapsed_days(year - 1);
        let ny1 = Self::hebrew_calendar_elapsed_days(year);
        let ny2 = Self::hebrew_calendar_elapsed_days(year + 1);
        
        if (ny2 - ny1) == 356 {
            2
        } else if (ny1 - ny0) == 382 {
            1
        } else {
            0
        }
    }
    
    /// Calculate R.D. of Rosh Hashanah (Tishrei 1) for a Hebrew year.
    ///
    /// Rosh Hashanah = epoch + elapsed_days(ADU applied) + year_length_correction
    fn hebrew_new_year(year: i32) -> i32 {
        (Self::HEBREW_EPOCH_RD as i64
            + Self::hebrew_calendar_elapsed_days(year) as i64
            + Self::hebrew_year_length_correction(year) as i64) as i32
    }
    
    /// Convert Hebrew date to R.D.
    fn hebrew_to_rd(hebrew: HebrewDate) -> Result<i32, CalendarError> {
        let is_leap = Self::is_hebrew_leap_year(hebrew.year);
        let month_num = hebrew.month.to_number(is_leap);
        
        // Start at Rosh Hashanah of the target year
        let mut rd = Self::hebrew_new_year(hebrew.year) as i64;
        
        // Add days for each month from Tishrei (month 7) to target month
        if month_num >= 7 {
            for m in 7..month_num {
                rd += Self::days_in_hebrew_month(hebrew.year, m) as i64;
            }
        } else {
            let months_in_year = Self::months_in_hebrew_year(hebrew.year);
            for m in 7..=months_in_year {
                rd += Self::days_in_hebrew_month(hebrew.year, m) as i64;
            }
            for m in 1..month_num {
                rd += Self::days_in_hebrew_month(hebrew.year, m) as i64;
            }
        }
        
        rd += (hebrew.day - 1) as i64;
        
        Ok(rd as i32)
    }
    
    /// Convert R.D. to Hebrew date
    fn rd_to_hebrew(rd: i32) -> Result<HebrewDate, CalendarError> {
        let rd_i64 = rd as i64;
        
        // Approximate year
        let mut year =
            ((rd_i64 - Self::HEBREW_EPOCH_RD as i64) as f64 / 365.25) as i32 + 1;
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
        
        let months_in_year = Self::months_in_hebrew_year(year);
        
        // Calculate days in first part of year (Tishrei through end)
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
            day,
        ))
    }
    
    /// Get the number of days in a Hebrew month
    pub fn days_in_hebrew_month(year: i32, month: u8) -> u8 {
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
            13 => 29,  // Adar II (leap only)
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
        assert!(DateConverter::is_hebrew_leap_year(5784), "5784 should be leap");
        assert!(!DateConverter::is_hebrew_leap_year(5783), "5783 should not be leap");
        assert!(!DateConverter::is_hebrew_leap_year(5785), "5785 should not be leap");
        assert!(!DateConverter::is_hebrew_leap_year(5786), "5786 should not be leap");
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
        // Rosh Hashanah 5784 = September 16, 2023
        let rd = DateConverter::rosh_hashanah(5784);
        let greg = DateConverter::rd_to_gregorian(rd).unwrap();
        assert_eq!(greg.year(), 2023);
        assert_eq!(greg.month(), 9);
        assert_eq!(greg.day(), 16);
    }
    
    #[test]
    fn test_rosh_hashanah_5785() {
        // Rosh Hashanah 5785 = October 3, 2024
        let rd = DateConverter::rosh_hashanah(5785);
        let greg = DateConverter::rd_to_gregorian(rd).unwrap();
        assert_eq!(greg.year(), 2024);
        assert_eq!(greg.month(), 10);
        assert_eq!(greg.day(), 3);
    }
    
    #[test]
    fn test_rosh_hashanah_multiple_years() {
        let test_cases = vec![
            (5783, 2022, 9, 26),
            (5784, 2023, 9, 16),
            (5785, 2024, 10, 3),
            (5786, 2025, 9, 23),
            (5787, 2026, 9, 12),
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
    fn test_rosh_hashanah_wide_range() {
        // Verified against hebcal for years 5750–5800
        let known = vec![
            (5750, 1989, 9, 30),
            (5751, 1990, 9, 20),
            (5752, 1991, 9,  9),
            (5753, 1992, 9, 28),
            (5754, 1993, 9, 16),
            (5755, 1994, 9,  6),
            (5756, 1995, 9, 25),
            (5757, 1996, 9, 14),
            (5758, 1997, 10, 2),
            (5759, 1998, 9, 21),
            (5760, 1999, 9, 11),
            (5761, 2000, 9, 30),
            (5762, 2001, 9, 18),
            (5763, 2002, 9,  7),
            (5764, 2003, 9, 27),
            (5765, 2004, 9, 16),
            (5766, 2005, 10, 4),
            (5767, 2006, 9, 23),
            (5768, 2007, 9, 13),
            (5769, 2008, 9, 30),
            (5770, 2009, 9, 19),
            (5771, 2010, 9,  9),
            (5772, 2011, 9, 29),
            (5773, 2012, 9, 17),
            (5774, 2013, 9,  5),
            (5775, 2014, 9, 25),
            (5776, 2015, 9, 14),
            (5777, 2016, 10, 3),
            (5778, 2017, 9, 21),
            (5779, 2018, 9, 10),
            (5780, 2019, 9, 30),
            (5781, 2020, 9, 19),
            (5782, 2021, 9,  7),
            (5783, 2022, 9, 26),
            (5784, 2023, 9, 16),
            (5785, 2024, 10, 3),
            (5786, 2025, 9, 23),
            (5787, 2026, 9, 12),
            (5788, 2027, 10, 2),
            (5789, 2028, 9, 21),
            (5790, 2029, 9, 10),
            (5791, 2030, 9, 28),
            (5792, 2031, 9, 18),
            (5793, 2032, 9,  6),
            (5794, 2033, 9, 24),
            (5795, 2034, 9, 14),
            (5796, 2035, 10, 4),
            (5797, 2036, 9, 22),
            (5798, 2037, 9, 10),
            (5799, 2038, 9, 30),
            (5800, 2039, 9, 19),
        ];
        for (hy, gy, gm, gd) in known {
            let rd = DateConverter::rosh_hashanah(hy);
            let g = DateConverter::rd_to_gregorian(rd).unwrap();
            assert_eq!(
                (g.year(), g.month(), g.day()),
                (gy, gm, gd),
                "Year {}: expected {}-{:02}-{:02}, got {}-{:02}-{:02}",
                hy, gy, gm, gd, g.year(), g.month(), g.day()
            );
        }
    }

    #[test]
    fn test_rosh_hashanah_day_of_week() {
        // Rosh Hashanah must never fall on Sunday, Wednesday, or Friday
        for year in 5750..5850 {
            let rd = DateConverter::rosh_hashanah(year);
            // rd % 7: 0=Sat, 1=Sun, 2=Mon, 3=Tue, 4=Wed, 5=Thu, 6=Fri
            let dow = rd.rem_euclid(7);
            assert!(
                dow != 1 && dow != 4 && dow != 6,
                "Year {}: Rosh Hashanah RD={} falls on dow={} (Sun=1,Wed=4,Fri=6)",
                year, rd, dow
            );
        }
    }

    #[test]
    fn test_year_length_validity() {
        // Year lengths must be in {353,354,355} for common or {383,384,385} for leap
        for year in 5750..5850 {
            let days = DateConverter::days_in_hebrew_year(year);
            let is_leap = DateConverter::is_hebrew_leap_year(year);
            let valid = if is_leap {
                days == 383 || days == 384 || days == 385
            } else {
                days == 353 || days == 354 || days == 355
            };
            assert!(
                valid,
                "Year {} (leap={}): got {} days, which is invalid",
                year, is_leap, days
            );
        }
    }
    
    #[test]
    fn test_gregorian_to_hebrew() {
        // Sept 16, 2023 = Tishrei 1, 5784
        let sept_16_2023 = NaiveDate::from_ymd_opt(2023, 9, 16).unwrap();
        let hebrew = DateConverter::gregorian_to_hebrew(sept_16_2023).unwrap();
        assert_eq!(hebrew.year, 5784);
        assert_eq!(hebrew.month, HebrewMonth::Tishrei);
        assert_eq!(hebrew.day, 1);
        
        // Jan 1, 2024 = Tevet 20, 5784
        let jan_1_2024 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let hebrew = DateConverter::gregorian_to_hebrew(jan_1_2024).unwrap();
        assert_eq!(hebrew.year, 5784);
        assert_eq!(hebrew.month, HebrewMonth::Teves);
        assert_eq!(hebrew.day, 20);
    }
    
    #[test]
    fn test_hebrew_to_gregorian() {
        // Tishrei 1, 5784 = Sept 16, 2023
        let tishrei_1_5784 = HebrewDate::new(5784, HebrewMonth::Tishrei, 1);
        let greg = DateConverter::hebrew_to_gregorian(tishrei_1_5784).unwrap();
        assert_eq!(greg.year(), 2023);
        assert_eq!(greg.month(), 9);
        assert_eq!(greg.day(), 16);
    }
    
    #[test]
    fn test_roundtrip_conversion() {
        let test_dates = vec![
            (2023, 9, 16),
            (2024, 1, 1),
            (2024, 6, 15),
            (2020, 2, 29),
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
        assert!(DateConverter::is_hebrew_leap_year(5784));
        let days_5784 = DateConverter::days_in_hebrew_year(5784);
        assert_eq!(days_5784, 383, "Year 5784 should have 383 days");
        
        assert!(!DateConverter::is_hebrew_leap_year(5783));
        let days_5783 = DateConverter::days_in_hebrew_year(5783);
        assert_eq!(days_5783, 355, "Year 5783 should have 355 days");
    }
    
    #[test]
    fn test_leap_year_months() {
        let months = DateConverter::months_in_hebrew_year(5784);
        assert_eq!(months, 13, "Leap year 5784 should have 13 months");
        let adar1 = DateConverter::days_in_hebrew_month(5784, 12);
        let adar2 = DateConverter::days_in_hebrew_month(5784, 13);
        assert_eq!(adar1, 30, "Adar I should have 30 days");
        assert_eq!(adar2, 29, "Adar II should have 29 days");
    }
    
    #[test]
    fn test_common_year_months() {
        let months = DateConverter::months_in_hebrew_year(5783);
        assert_eq!(months, 12, "Common year 5783 should have 12 months");
        let adar = DateConverter::days_in_hebrew_month(5783, 12);
        assert_eq!(adar, 29, "Regular Adar should have 29 days");
    }
    
    #[test]
    fn test_historical_dates() {
        // Yom Kippur War: October 6, 1973 = Tishrei 10, 5734
        let oct_6_1973 = NaiveDate::from_ymd_opt(1973, 10, 6).unwrap();
        let hebrew = DateConverter::gregorian_to_hebrew(oct_6_1973).unwrap();
        assert_eq!(hebrew.year, 5734);
        assert_eq!(hebrew.month, HebrewMonth::Tishrei);
        assert_eq!(hebrew.day, 10);
    }
    
    #[test]
    fn test_day_of_week() {
        // Rosh Hashanah 5784 (Sept 16, 2023) was a Saturday
        let tishrei_1_5784 = HebrewDate::new(5784, HebrewMonth::Tishrei, 1);
        let dow = tishrei_1_5784.day_of_week();
        assert_eq!(dow, 6, "Rosh Hashanah 5784 should be Saturday (6)");
        
        let greg = DateConverter::hebrew_to_gregorian(tishrei_1_5784).unwrap();
        assert_eq!(greg.weekday().num_days_from_monday(), 5, "Should be Saturday");
    }
}
