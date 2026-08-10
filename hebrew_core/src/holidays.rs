//! Holiday Calculation Module
//! 
//! Implements identification of Jewish holidays based on Hebrew calendar dates.
//! Supports both diaspora and Israel observance, and modern Israeli holidays.

use serde::{Deserialize, Serialize};

use crate::calendar::{DateConverter, HebrewDate, HebrewMonth};
use crate::CalendarError;

/// Jewish holiday
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Holiday {
    // Rosh Hashanah
    RoshHashanahDay1,
    RoshHashanahDay2,
    
    // Tzom Gedaliah (Tishrei 3; pushed to 4 if 3 falls on Shabbat)
    TzomGedaliah,
    
    // Yom Kippur
    YomKippur,
    
    // Sukkot
    SukkotDay1,
    SukkotDay2,
    SukkotCholHamoedDay1,
    SukkotCholHamoedDay2,
    SukkotCholHamoedDay3,
    SukkotCholHamoedDay4,
    SukkotCholHamoedDay5,
    HoshanaRabbah,
    SheminiAtzeret,
    SimchatTorah,
    
    // Chanukah
    ChanukahDay1,
    ChanukahDay2,
    ChanukahDay3,
    ChanukahDay4,
    ChanukahDay5,
    ChanukahDay6,
    ChanukahDay7,
    ChanukahDay8,
    
    // Fast of Tevet (10 Tevet)
    AsaraBTevet,
    
    // Tu B'Shevat
    TuBiShevat,
    
    // Purim
    TaanitEsther,
    Purim,
    ShushanPurim,
    
    // Pesach
    PesachDay1,
    PesachDay2,
    PesachCholHamoedDay1,
    PesachCholHamoedDay2,
    PesachCholHamoedDay3,
    PesachCholHamoedDay4,
    PesachDay7,
    PesachDay8,
    
    // Modern Israeli holidays
    YomHaShoah,
    YomHaZikaron,
    YomHaAtzmaut,
    YomYerushalayim,
    
    // Shavuot
    ShavuotDay1,
    ShavuotDay2,
    
    // Tisha B'Av and Three Weeks
    ShivaAsarBTammuz,
    TishaBAv,
    TuBAv,
    
    // Omer counting
    OmerDay(u8),   // 1..=49
    
    // Rosh Chodesh
    RoshChodesh,
}

impl Holiday {
    /// Get the English name of the holiday
    pub fn name(&self) -> String {
        match self {
            Holiday::RoshHashanahDay1 => "Rosh Hashanah (Day 1)".into(),
            Holiday::RoshHashanahDay2 => "Rosh Hashanah (Day 2)".into(),
            Holiday::TzomGedaliah => "Tzom Gedaliah".into(),
            Holiday::YomKippur => "Yom Kippur".into(),
            Holiday::SukkotDay1 => "Sukkot (Day 1)".into(),
            Holiday::SukkotDay2 => "Sukkot (Day 2)".into(),
            Holiday::SukkotCholHamoedDay1 => "Sukkot (Chol HaMoed Day 1)".into(),
            Holiday::SukkotCholHamoedDay2 => "Sukkot (Chol HaMoed Day 2)".into(),
            Holiday::SukkotCholHamoedDay3 => "Sukkot (Chol HaMoed Day 3)".into(),
            Holiday::SukkotCholHamoedDay4 => "Sukkot (Chol HaMoed Day 4)".into(),
            Holiday::SukkotCholHamoedDay5 => "Sukkot (Chol HaMoed Day 5)".into(),
            Holiday::HoshanaRabbah => "Hoshana Rabbah".into(),
            Holiday::SheminiAtzeret => "Shemini Atzeret".into(),
            Holiday::SimchatTorah => "Simchat Torah".into(),
            Holiday::ChanukahDay1 => "Chanukah (Day 1 - 1 Candle)".into(),
            Holiday::ChanukahDay2 => "Chanukah (Day 2 - 2 Candles)".into(),
            Holiday::ChanukahDay3 => "Chanukah (Day 3 - 3 Candles)".into(),
            Holiday::ChanukahDay4 => "Chanukah (Day 4 - 4 Candles)".into(),
            Holiday::ChanukahDay5 => "Chanukah (Day 5 - 5 Candles)".into(),
            Holiday::ChanukahDay6 => "Chanukah (Day 6 - 6 Candles)".into(),
            Holiday::ChanukahDay7 => "Chanukah (Day 7 - 7 Candles)".into(),
            Holiday::ChanukahDay8 => "Chanukah (Day 8 - 8 Candles)".into(),
            Holiday::AsaraBTevet => "Asara B'Tevet (Fast of Tevet)".into(),
            Holiday::TuBiShevat => "Tu B'Shevat".into(),
            Holiday::TaanitEsther => "Ta'anit Esther".into(),
            Holiday::Purim => "Purim".into(),
            Holiday::ShushanPurim => "Shushan Purim".into(),
            Holiday::PesachDay1 => "Pesach (Day 1)".into(),
            Holiday::PesachDay2 => "Pesach (Day 2)".into(),
            Holiday::PesachCholHamoedDay1 => "Pesach (Chol HaMoed Day 1)".into(),
            Holiday::PesachCholHamoedDay2 => "Pesach (Chol HaMoed Day 2)".into(),
            Holiday::PesachCholHamoedDay3 => "Pesach (Chol HaMoed Day 3)".into(),
            Holiday::PesachCholHamoedDay4 => "Pesach (Chol HaMoed Day 4)".into(),
            Holiday::PesachDay7 => "Pesach (Day 7)".into(),
            Holiday::PesachDay8 => "Pesach (Day 8)".into(),
            Holiday::YomHaShoah => "Yom HaShoah".into(),
            Holiday::YomHaZikaron => "Yom HaZikaron".into(),
            Holiday::YomHaAtzmaut => "Yom HaAtzmaut".into(),
            Holiday::YomYerushalayim => "Yom Yerushalayim".into(),
            Holiday::ShavuotDay1 => "Shavuot (Day 1)".into(),
            Holiday::ShavuotDay2 => "Shavuot (Day 2)".into(),
            Holiday::ShivaAsarBTammuz => "Shiva Asar B'Tammuz".into(),
            Holiday::TishaBAv => "Tisha B'Av".into(),
            Holiday::TuBAv => "Tu B'Av".into(),
            Holiday::OmerDay(n) => {
                if *n == 33 {
                    format!("Omer Day 33 (Lag BaOmer)")
                } else {
                    format!("Omer Day {}", n)
                }
            }
            Holiday::RoshChodesh => "Rosh Chodesh".into(),
        }
    }
    
    /// Check if this holiday requires candle lighting
    pub fn requires_candles(&self) -> bool {
        matches!(self,
            Holiday::RoshHashanahDay1 | Holiday::RoshHashanahDay2 |
            Holiday::YomKippur |
            Holiday::SukkotDay1 | Holiday::SukkotDay2 |
            Holiday::SheminiAtzeret | Holiday::SimchatTorah |
            Holiday::PesachDay1 | Holiday::PesachDay2 |
            Holiday::PesachDay7 | Holiday::PesachDay8 |
            Holiday::ShavuotDay1 | Holiday::ShavuotDay2 |
            Holiday::ChanukahDay1 | Holiday::ChanukahDay2 |
            Holiday::ChanukahDay3 | Holiday::ChanukahDay4 |
            Holiday::ChanukahDay5 | Holiday::ChanukahDay6 |
            Holiday::ChanukahDay7 | Holiday::ChanukahDay8
        )
    }
    
    /// Check if this is a Yom Tov (major holiday with work restrictions)
    pub fn is_yom_tov(&self) -> bool {
        matches!(self,
            Holiday::RoshHashanahDay1 | Holiday::RoshHashanahDay2 |
            Holiday::YomKippur |
            Holiday::SukkotDay1 | Holiday::SukkotDay2 |
            Holiday::SheminiAtzeret | Holiday::SimchatTorah |
            Holiday::PesachDay1 | Holiday::PesachDay2 |
            Holiday::PesachDay7 | Holiday::PesachDay8 |
            Holiday::ShavuotDay1 | Holiday::ShavuotDay2
        )
    }
    
    /// Check if this is a fast day
    pub fn is_fast_day(&self) -> bool {
        matches!(self,
            Holiday::YomKippur | Holiday::TaanitEsther |
            Holiday::TishaBAv | Holiday::ShivaAsarBTammuz |
            Holiday::TzomGedaliah | Holiday::AsaraBTevet
        )
    }
}

/// Holiday calculator
pub struct HolidayCalculator;

impl HolidayCalculator {
    /// Get all holidays for a specific Hebrew date
    pub fn get_holidays(date: &HebrewDate) -> Result<Vec<Holiday>, CalendarError> {
        let mut holidays = Vec::new();
        
        // Major fixed-date holidays
        if let Some(holiday) = Self::get_major_holiday(date) {
            holidays.push(holiday);
        }
        
        // Chanukah
        if let Some(chanukah) = Self::get_chanukah_day(date) {
            holidays.push(chanukah);
        }
        
        // Omer
        if let Some(omer) = Self::get_omer_day(date) {
            holidays.push(omer);
        }
        
        // Modern Israeli holidays (Iyar)
        if let Some(modern) = Self::get_modern_israeli_holiday(date) {
            holidays.push(modern);
        }
        
        // Rosh Chodesh
        if Self::is_rosh_chodesh(date) {
            holidays.push(Holiday::RoshChodesh);
        }
        
        Ok(holidays)
    }
    
    /// Get major holiday for the date (if any)
    fn get_major_holiday(date: &HebrewDate) -> Option<Holiday> {
        match date.month {
            HebrewMonth::Tishrei => match date.day {
                1 => Some(Holiday::RoshHashanahDay1),
                2 => Some(Holiday::RoshHashanahDay2),
                15 => Some(Holiday::SukkotDay1),
                16 => Some(Holiday::SukkotDay2),
                17..=20 => Some(match date.day {
                    17 => Holiday::SukkotCholHamoedDay1,
                    18 => Holiday::SukkotCholHamoedDay2,
                    19 => Holiday::SukkotCholHamoedDay3,
                    _ => Holiday::SukkotCholHamoedDay4,
                }),
                21 => Some(Holiday::HoshanaRabbah),
                22 => Some(Holiday::SheminiAtzeret),
                23 => Some(Holiday::SimchatTorah),
                10 => Some(Holiday::YomKippur),
                // Tzom Gedaliah: normally Tishrei 3, but if 3 is Shabbat → 4
                3 => {
                    let dow = date.day_of_week();
                    if dow == 6 { None } else { Some(Holiday::TzomGedaliah) }
                }
                4 => {
                    let dow = date.day_of_week();
                    // If Tishrei 3 was Shabbat, Tzom Gedaliah is on Tishrei 4
                    let tishrei_3_dow = (dow + 6) % 7; // day of week of day 3
                    if tishrei_3_dow == 6 { Some(Holiday::TzomGedaliah) } else { None }
                }
                _ => None,
            },
            HebrewMonth::Cheshvan => None,
            HebrewMonth::Kislev => {
                // Only 10 Tevet spills into Kislev in rare years,
                // but normatively it's in Teves. Chanukah handled separately.
                None
            },
            HebrewMonth::Teves => {
                if date.day == 10 {
                    Some(Holiday::AsaraBTevet)
                } else {
                    None
                }
            },
            HebrewMonth::Shevat => {
                if date.day == 15 {
                    Some(Holiday::TuBiShevat)
                } else {
                    None
                }
            },
            HebrewMonth::Adar => {
                // In common years, this is the only Adar.
                // In leap years, Adar == Adar II (see HebrewMonth enum).
                match date.day {
                    13 => Some(Holiday::TaanitEsther),
                    14 => Some(Holiday::Purim),
                    15 => Some(Holiday::ShushanPurim),
                    _ => None,
                }
            },
            HebrewMonth::AdarI => None,  // No holidays in Adar I
            HebrewMonth::Nisan => match date.day {
                15 => Some(Holiday::PesachDay1),
                16 => Some(Holiday::PesachDay2),
                17..=20 => Some(match date.day {
                    17 => Holiday::PesachCholHamoedDay1,
                    18 => Holiday::PesachCholHamoedDay2,
                    19 => Holiday::PesachCholHamoedDay3,
                    _ => Holiday::PesachCholHamoedDay4,
                }),
                21 => Some(Holiday::PesachDay7),
                22 => Some(Holiday::PesachDay8),
                _ => None,
            },
            HebrewMonth::Iyar => None,  // Modern holidays handled separately
            HebrewMonth::Sivan => match date.day {
                6 => Some(Holiday::ShavuotDay1),
                7 => Some(Holiday::ShavuotDay2),
                _ => None,
            },
            HebrewMonth::Tammuz => {
                if date.day == 17 {
                    Some(Holiday::ShivaAsarBTammuz)
                } else {
                    None
                }
            },
            HebrewMonth::Av => match date.day {
                9 => Some(Holiday::TishaBAv),
                15 => Some(Holiday::TuBAv),
                _ => None,
            },
            HebrewMonth::Elul => None,
        }
    }
    
    /// Get modern Israeli national holidays.
    ///
    /// Rules:
    /// - Yom HaShoah (Iyar 27): moves to 26 if 27 is Friday, to 28 if 27 is Sunday.
    /// - Yom HaZikaron (Iyar 4): moves to 3 if 4 is Thursday, to 5 if 4 is Friday or Saturday.
    /// - Yom HaAtzmaut (Iyar 5): follows Yom HaZikaron.
    /// - Yom Yerushalayim (Iyar 28): fixed.
    fn get_modern_israeli_holiday(date: &HebrewDate) -> Option<Holiday> {
        if date.month != HebrewMonth::Iyar {
            return None;
        }
        
        let day = date.day;
        
        // Compute Nisan 1 day of week for this year.
        let rh = DateConverter::rosh_hashanah(date.year);
        // rd % 7: 0=Sat, 1=Sun, 2=Mon, 3=Tue, 4=Wed, 5=Thu, 6=Fri
        let rh_dow_raw = rh.rem_euclid(7);
        // Convert to our convention: 0=Sun.  Formula: (rd_dow + 6) % 7
        let rh_dow = ((rh_dow_raw + 6).rem_euclid(7)) as u8;
        
        let nisan1_offset = Self::days_from_tishrei_to_nisan(date.year);
        
        // Nisan 1 dow
        let nisan1_dow = ((rh_dow as i64 + nisan1_offset as i64).rem_euclid(7)) as u8;
        
        // Pesach (Nisan 15) day of week
        let pesach_dow = ((nisan1_dow as i64 + 14).rem_euclid(7)) as u8;
        
        // --- Yom HaShoah: Iyar 27 (norminal) ---
        // Moved to 26 if 27 is Friday, to 28 if 27 is Sunday.
        let yhs_nominal_dow = Self::add_days_to_dow(pesach_dow, 42); // Nisan 15 -> Iyar 27 = 42 days
        
        let yhs_actual_day = 
            if yhs_nominal_dow == 5 { 26 }       // Friday → Thursday (26)
            else if yhs_nominal_dow == 0 { 28 }  // Sunday → Monday (28)
            else { 27 };
        
        if day == yhs_actual_day {
            return Some(Holiday::YomHaShoah);
        }
        
        // --- Yom HaZikaron: Iyar 4 (nominal) ---
        // Moved to 3 if 4 is Thursday, to 5 if 4 is Friday or Saturday.
        let yhz_nominal_dow = Self::add_days_to_dow(pesach_dow, 19); // Nisan 15 -> Iyar 4 = 19 days
        
        let yhz_actual_day =
            if yhz_nominal_dow == 4 { 3 }           // Thursday → Wednesday (3)
            else if yhz_nominal_dow == 5 || yhz_nominal_dow == 6 { 5 }  // Fri/Sat → Sunday (5)
            else { 4 };
        
        if day == yhz_actual_day {
            return Some(Holiday::YomHaZikaron);
        }
        
        // --- Yom HaAtzmaut: Iyar 5, follows Yom HaZikaron ---
        let yha_actual_day =
            if yhz_actual_day == 4 { 5 }   // normal
            else if yhz_actual_day == 3 { 4 } // pushed earlier
            else { 6 }; // Yom HaZikaron pushed to 5, Yom HaAtzmaut → 6
        
        if day == yha_actual_day {
            return Some(Holiday::YomHaAtzmaut);
        }
        
        // --- Yom Yerushalayim: Iyar 28 ---
        if day == 28 {
            return Some(Holiday::YomYerushalayim);
        }
        
        None
    }
    
    /// Count days from Tishrei 1 to Nisan 1 for a given Hebrew year.
    fn days_from_tishrei_to_nisan(year: i32) -> u16 {
        let is_leap = DateConverter::is_hebrew_leap_year(year);
        let months = if is_leap { 6 } else { 5 }; // Tishrei..Adar (or AdarII)
        let mut days: u16 = 0;
        for m in 7..(7 + months) {
            days += DateConverter::days_in_hebrew_month(year, m as u8) as u16;
        }
        days
    }
    
    /// Add `days` to a day-of-week (0=Sun), returning (dow + days) % 7.
    fn add_days_to_dow(dow: u8, days: i32) -> u8 {
        ((dow as i32 + days).rem_euclid(7)) as u8
    }
    
    /// True if `date` is Rosh Chodesh.
    ///
    /// Rosh Chodesh is day 1 of any month (except Tishrei, which is Rosh Hashanah)
    /// and day 30 of months that have 30 days (the second day of Rosh Chodesh for
    /// these months).
    fn is_rosh_chodesh(date: &HebrewDate) -> bool {
        // Tishrei 1 is Rosh Hashanah, not Rosh Chodesh.
        // But Tishrei 30 IS Rosh Chodesh Cheshvan.
        if date.month == HebrewMonth::Tishrei && date.day == 1 {
            return false;
        }
        
        if date.day == 1 {
            return true;
        }
        
        // Day 30 is Rosh Chodesh only if this month has 30 days
        if date.day == 30 {
            let is_leap = DateConverter::is_hebrew_leap_year(date.year);
            let month_num = date.month.to_number(is_leap);
            return DateConverter::days_in_hebrew_month(date.year, month_num) == 30;
        }
        
        false
    }
    
    /// Get Chanukah day (if applicable)
    fn get_chanukah_day(date: &HebrewDate) -> Option<Holiday> {
        let day = if date.month == HebrewMonth::Kislev && date.day >= 25 {
            (date.day - 24) as usize
        } else if date.month == HebrewMonth::Teves {
            let kislev_days = if Self::is_short_kislev(date.year) { 29 } else { 30 };
            if date.day as usize + (kislev_days - 24) <= 8 {
                (date.day as usize + kislev_days - 24) as usize
            } else {
                0
            }
        } else {
            0
        };
        
        match day {
            1 => Some(Holiday::ChanukahDay1),
            2 => Some(Holiday::ChanukahDay2),
            3 => Some(Holiday::ChanukahDay3),
            4 => Some(Holiday::ChanukahDay4),
            5 => Some(Holiday::ChanukahDay5),
            6 => Some(Holiday::ChanukahDay6),
            7 => Some(Holiday::ChanukahDay7),
            8 => Some(Holiday::ChanukahDay8),
            _ => None,
        }
    }
    
    /// Check if Kislev has 29 days (deficient year)
    fn is_short_kislev(year: i32) -> bool {
        let year_type = DateConverter::hebrew_year_type(year);
        matches!(year_type, 
            crate::calendar::YearType::DeficientCommon | 
            crate::calendar::YearType::DeficientLeap
        )
    }
    
    /// Get Omer day (if applicable).
    /// Omer starts on 16 Nisan and continues for 49 days through 5 Sivan.
    fn get_omer_day(date: &HebrewDate) -> Option<Holiday> {
        let omer_day = match date.month {
            HebrewMonth::Nisan if date.day >= 16 => (date.day - 15) as usize,
            HebrewMonth::Iyar => (15 + date.day) as usize,
            HebrewMonth::Sivan if date.day <= 5 => (44 + date.day) as usize,
            _ => 0,
        };
        
        if omer_day >= 1 && omer_day <= 49 {
            Some(Holiday::OmerDay(omer_day as u8))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::HebrewMonth;
    
    #[test]
    fn test_rosh_hashanah() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 1);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::RoshHashanahDay1));
        // Tishrei 1 is NOT Rosh Chodesh (it's Rosh Hashanah)
        assert!(!holidays.contains(&Holiday::RoshChodesh));
    }
    
    #[test]
    fn test_yom_kippur() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 10);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::YomKippur));
    }
    
    #[test]
    fn test_tzom_gedaliah() {
        // 5784: Tishrei 3 was Monday (Sept 18, 2023) — not Shabbat, so Tzom Gedaliah on day 3
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 3);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::TzomGedaliah));
    }
    
    #[test]
    fn test_asara_tevet() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Teves, 10);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::AsaraBTevet));
    }
    
    #[test]
    fn test_pesach() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Nisan, 15);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::PesachDay1));
        assert!(!holidays.contains(&Holiday::RoshChodesh));
    }
    
    #[test]
    fn test_omer() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Nisan, 16);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::OmerDay(1)));
        
        let lag_baomer = HebrewDate::new(5784, HebrewMonth::Iyar, 18);
        let holidays = HolidayCalculator::get_holidays(&lag_baomer).unwrap();
        assert!(holidays.contains(&Holiday::OmerDay(33)));
        assert_eq!(Holiday::OmerDay(33).name(), "Omer Day 33 (Lag BaOmer)");
    }
    
    #[test]
    fn test_chanukah() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Kislev, 25);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::ChanukahDay1));
    }

    // === Sukkot complete cycle ===

    #[test]
    fn test_sukkot_day1() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 15);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::SukkotDay1));
    }

    #[test]
    fn test_sukkot_day2() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 16);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::SukkotDay2));
    }

    #[test]
    fn test_sukkot_chol_hamoed() {
        let expected = [
            (17, Holiday::SukkotCholHamoedDay1),
            (18, Holiday::SukkotCholHamoedDay2),
            (19, Holiday::SukkotCholHamoedDay3),
            (20, Holiday::SukkotCholHamoedDay4),
        ];
        for (day, expected_holiday) in &expected {
            let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, *day);
            let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
            assert!(holidays.contains(expected_holiday),
                "Tishrei {} should contain {:?}", day, expected_holiday);
        }
    }

    #[test]
    fn test_hoshana_rabbah() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 21);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::HoshanaRabbah));
    }

    #[test]
    fn test_shemini_atzeret() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 22);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::SheminiAtzeret));
    }

    #[test]
    fn test_simchat_torah() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 23);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::SimchatTorah));
    }

    // === Chanukah detailed ===

    #[test]
    fn test_chanukah_all_8_days_short_kislev() {
        // 5784 is a deficient leap year (Kislev has 29 days)
        let expected_kislev = [
            (25, Holiday::ChanukahDay1),
            (26, Holiday::ChanukahDay2),
            (27, Holiday::ChanukahDay3),
            (28, Holiday::ChanukahDay4),
            (29, Holiday::ChanukahDay5),
        ];
        for (day, expected) in &expected_kislev {
            let hebrew = HebrewDate::new(5784, HebrewMonth::Kislev, *day);
            let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
            assert!(holidays.contains(expected),
                "Kislev {} should be {:?}", day, expected);
        }
        let expected_teves = [
            (1, Holiday::ChanukahDay6),
            (2, Holiday::ChanukahDay7),
            (3, Holiday::ChanukahDay8),
        ];
        for (day, expected) in &expected_teves {
            let hebrew = HebrewDate::new(5784, HebrewMonth::Teves, *day);
            let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
            assert!(holidays.contains(expected),
                "Teves {} should be {:?}", day, expected);
        }
    }

    #[test]
    fn test_chanukah_all_8_days_long_kislev() {
        // 5783 is a complete common year (Kislev has 30 days)
        let expected_kislev = [
            (25, Holiday::ChanukahDay1),
            (26, Holiday::ChanukahDay2),
            (27, Holiday::ChanukahDay3),
            (28, Holiday::ChanukahDay4),
            (29, Holiday::ChanukahDay5),
            (30, Holiday::ChanukahDay6),
        ];
        for (day, expected) in &expected_kislev {
            let hebrew = HebrewDate::new(5783, HebrewMonth::Kislev, *day);
            let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
            assert!(holidays.contains(expected),
                "Kislev {} (5783) should be {:?}", day, expected);
        }
        let expected_teves = [
            (1, Holiday::ChanukahDay7),
            (2, Holiday::ChanukahDay8),
        ];
        for (day, expected) in &expected_teves {
            let hebrew = HebrewDate::new(5783, HebrewMonth::Teves, *day);
            let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
            assert!(holidays.contains(expected),
                "Teves {} (5783) should be {:?}", day, expected);
        }
    }

    #[test]
    fn test_no_chanukah_before_25_kislev() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Kislev, 24);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        let has_chanukah = holidays.iter().any(|h| matches!(h,
            Holiday::ChanukahDay1 | Holiday::ChanukahDay2 | Holiday::ChanukahDay3 |
            Holiday::ChanukahDay4 | Holiday::ChanukahDay5 | Holiday::ChanukahDay6 |
            Holiday::ChanukahDay7 | Holiday::ChanukahDay8));
        assert!(!has_chanukah, "Kislev 24 should not be Chanukah");
    }

    #[test]
    fn test_no_chanukah_after_last_day() {
        // After Chanukah ends in 5784 (short Kislev): Teves 4
        let hebrew = HebrewDate::new(5784, HebrewMonth::Teves, 4);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        let has_chanukah = holidays.iter().any(|h| matches!(h,
            Holiday::ChanukahDay1 | Holiday::ChanukahDay2 | Holiday::ChanukahDay3 |
            Holiday::ChanukahDay4 | Holiday::ChanukahDay5 | Holiday::ChanukahDay6 |
            Holiday::ChanukahDay7 | Holiday::ChanukahDay8));
        assert!(!has_chanukah, "Teves 4 should not be Chanukah in 5784");
    }

    // === Purim ===

    #[test]
    fn test_purim_leap_year() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Adar, 14);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::Purim));
    }

    #[test]
    fn test_taanit_esther() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Adar, 13);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::TaanitEsther));
    }

    #[test]
    fn test_shushan_purim() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Adar, 15);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::ShushanPurim));
    }

    #[test]
    fn test_no_purim_adar_i_leap_year() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::AdarI, 14);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(!holidays.contains(&Holiday::Purim),
            "Adar I 14 in a leap year should not have Purim");
    }

    // === Other holidays ===

    #[test]
    fn test_tu_bishvat() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Shevat, 15);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::TuBiShevat));
    }

    #[test]
    fn test_shavuot() {
        let day1 = HebrewDate::new(5784, HebrewMonth::Sivan, 6);
        let holidays1 = HolidayCalculator::get_holidays(&day1).unwrap();
        assert!(holidays1.contains(&Holiday::ShavuotDay1));

        let day2 = HebrewDate::new(5784, HebrewMonth::Sivan, 7);
        let holidays2 = HolidayCalculator::get_holidays(&day2).unwrap();
        assert!(holidays2.contains(&Holiday::ShavuotDay2));
    }

    #[test]
    fn test_tisha_bav() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Av, 9);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::TishaBAv));
    }

    #[test]
    fn test_17_tammuz() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tammuz, 17);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::ShivaAsarBTammuz));
    }

    #[test]
    fn test_tu_bav() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Av, 15);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::TuBAv));
    }

    // === Rosh Chodesh ===

    #[test]
    fn test_rosh_chodesh_day_1() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Cheshvan, 1);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::RoshChodesh));
    }

    #[test]
    fn test_rosh_chodesh_day_30_long_month() {
        // Tishrei has 30 days; day 30 is Rosh Chodesh Cheshvan
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 30);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::RoshChodesh));
    }

    #[test]
    fn test_no_rosh_chodesh_day_30_short_month() {
        // Iyar has 29 days; day 30 doesn't exist, but if called should not return Rosh Chodesh
        let hebrew = HebrewDate::new(5784, HebrewMonth::Iyar, 30);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(!holidays.contains(&Holiday::RoshChodesh),
            "Iyar has 29 days, day 30 should not be Rosh Chodesh");
    }

    #[test]
    fn test_no_rosh_chodesh_tishrei_1() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 1);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(!holidays.contains(&Holiday::RoshChodesh),
            "Tishrei 1 is Rosh Hashanah, not Rosh Chodesh");
    }

    #[test]
    fn test_no_rosh_chodesh_mid_month() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Cheshvan, 15);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(!holidays.contains(&Holiday::RoshChodesh));
    }

    // === Modern Israeli holidays ===

    #[test]
    fn test_yom_haatzmaut_5784() {
        // 5784: Yom HaAtzmaut should be on Iyar 6
        // (Iyar 4 was Friday → pushed to 5, so Yom HaAtzmaut → 6)
        let date = HebrewDate::new(5784, HebrewMonth::Iyar, 6);
        let holidays = HolidayCalculator::get_holidays(&date).unwrap();
        assert!(holidays.contains(&Holiday::YomHaAtzmaut),
            "5784 Iyar 6 should be Yom HaAtzmaut");
    }

    #[test]
    fn test_modern_israeli_holidays_exist() {
        // 5783 (2023): Yom HaShoah → Iyar 27 (Thursday, no move),
        // Yom HaZikaron → Iyar 4 (Tuesday, no move), Yom HaAtzmaut → Iyar 5 (Wednesday)
        let shoah = HebrewDate::new(5783, HebrewMonth::Iyar, 27);
        let h = HolidayCalculator::get_holidays(&shoah).unwrap();
        assert!(h.iter().any(|hol| matches!(hol, Holiday::YomHaShoah)),
            "5783 Iyar 27 should be Yom HaShoah");

        let atzmaut = HebrewDate::new(5783, HebrewMonth::Iyar, 5);
        let h = HolidayCalculator::get_holidays(&atzmaut).unwrap();
        assert!(h.iter().any(|hol| matches!(hol, Holiday::YomHaAtzmaut)),
            "5783 Iyar 5 should be Yom HaAtzmaut");
    }

    // === Omer boundaries ===

    #[test]
    fn test_omer_last_day_nisan() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Nisan, 30);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::OmerDay(15)),
            "Nisan 30 should be Omer Day 15");
    }

    #[test]
    fn test_omer_iyar_1() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Iyar, 1);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::OmerDay(16)));
    }

    #[test]
    fn test_omer_sivan_1() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Sivan, 1);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::OmerDay(45)));
    }

    #[test]
    fn test_omer_day_49() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Sivan, 5);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::OmerDay(49)));
    }

    #[test]
    fn test_no_omer_sivan_6() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Sivan, 6);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        let has_omer = holidays.iter().any(|h| matches!(h, Holiday::OmerDay(_)));
        assert!(!has_omer, "Sivan 6 (Shavuot) should not have Omer");
    }

    #[test]
    fn test_no_omer_nisan_15() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Nisan, 15);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        let has_omer = holidays.iter().any(|h| matches!(h, Holiday::OmerDay(_)));
        assert!(!has_omer, "Nisan 15 should not have Omer");
    }

    // === Trait methods ===

    #[test]
    fn test_is_yom_tov() {
        assert!(Holiday::RoshHashanahDay1.is_yom_tov());
        assert!(Holiday::YomKippur.is_yom_tov());
        assert!(Holiday::SukkotDay1.is_yom_tov());
        assert!(Holiday::PesachDay1.is_yom_tov());
        assert!(Holiday::ShavuotDay1.is_yom_tov());
        assert!(!Holiday::ChanukahDay1.is_yom_tov());
        assert!(!Holiday::Purim.is_yom_tov());
        assert!(!Holiday::HoshanaRabbah.is_yom_tov());
        assert!(!Holiday::RoshChodesh.is_yom_tov());
    }

    #[test]
    fn test_requires_candles() {
        assert!(Holiday::RoshHashanahDay1.requires_candles());
        assert!(Holiday::ChanukahDay1.requires_candles());
        assert!(Holiday::ShavuotDay2.requires_candles());
        assert!(!Holiday::Purim.requires_candles());
        assert!(!Holiday::TuBiShevat.requires_candles());
        assert!(!Holiday::RoshChodesh.requires_candles());
    }

    #[test]
    fn test_is_fast_day() {
        assert!(Holiday::YomKippur.is_fast_day());
        assert!(Holiday::TaanitEsther.is_fast_day());
        assert!(Holiday::TishaBAv.is_fast_day());
        assert!(Holiday::ShivaAsarBTammuz.is_fast_day());
        assert!(Holiday::TzomGedaliah.is_fast_day());
        assert!(Holiday::AsaraBTevet.is_fast_day());
        assert!(!Holiday::RoshHashanahDay1.is_fast_day());
        assert!(!Holiday::Purim.is_fast_day());
        assert!(!Holiday::ChanukahDay1.is_fast_day());
    }

    #[test]
    fn test_holiday_names() {
        assert_eq!(Holiday::RoshHashanahDay1.name(), "Rosh Hashanah (Day 1)");
        assert_eq!(Holiday::YomKippur.name(), "Yom Kippur");
        assert_eq!(Holiday::Purim.name(), "Purim");
        assert_eq!(Holiday::TuBiShevat.name(), "Tu B'Shevat");
        assert_eq!(Holiday::OmerDay(33).name(), "Omer Day 33 (Lag BaOmer)");
    }
}
