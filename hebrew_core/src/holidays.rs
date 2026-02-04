//! Holiday Calculation Module
//! 
//! Implements identification of Jewish holidays based on Hebrew calendar dates.

use serde::{Deserialize, Serialize};

use crate::calendar::{DateConverter, HebrewDate, HebrewMonth};
use crate::CalendarError;

/// Jewish holiday
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Holiday {
    // Rosh Hashanah
    RoshHashanahDay1,
    RoshHashanahDay2,
    
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
    
    // Counting the Omer
    OmerDay1, OmerDay2, OmerDay3, OmerDay4, OmerDay5, OmerDay6, OmerDay7,
    OmerDay8, OmerDay9, OmerDay10, OmerDay11, OmerDay12, OmerDay13, OmerDay14,
    OmerDay15, OmerDay16, OmerDay17, OmerDay18, OmerDay19, OmerDay20, OmerDay21,
    OmerDay22, OmerDay23, OmerDay24, OmerDay25, OmerDay26, OmerDay27, OmerDay28,
    OmerDay29, OmerDay30, OmerDay31, OmerDay32, OmerDay33, OmerDay34, OmerDay35,
    OmerDay36, OmerDay37, OmerDay38, OmerDay39, OmerDay40, OmerDay41, OmerDay42,
    OmerDay43, OmerDay44, OmerDay45, OmerDay46, OmerDay47, OmerDay48, OmerDay49,
    LagBaOmer,
    
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
    
    // Rosh Chodesh
    RoshChodesh,
}

impl Holiday {
    /// Get the English name of the holiday
    pub fn name(&self) -> &'static str {
        match self {
            Holiday::RoshHashanahDay1 => "Rosh Hashanah (Day 1)",
            Holiday::RoshHashanahDay2 => "Rosh Hashanah (Day 2)",
            Holiday::YomKippur => "Yom Kippur",
            Holiday::SukkotDay1 => "Sukkot (Day 1)",
            Holiday::SukkotDay2 => "Sukkot (Day 2)",
            Holiday::SukkotCholHamoedDay1 => "Sukkot (Chol HaMoed Day 1)",
            Holiday::SukkotCholHamoedDay2 => "Sukkot (Chol HaMoed Day 2)",
            Holiday::SukkotCholHamoedDay3 => "Sukkot (Chol HaMoed Day 3)",
            Holiday::SukkotCholHamoedDay4 => "Sukkot (Chol HaMoed Day 4)",
            Holiday::SukkotCholHamoedDay5 => "Sukkot (Chol HaMoed Day 5)",
            Holiday::HoshanaRabbah => "Hoshana Rabbah",
            Holiday::SheminiAtzeret => "Shemini Atzeret",
            Holiday::SimchatTorah => "Simchat Torah",
            Holiday::ChanukahDay1 => "Chanukah (Day 1 - 1 Candle)",
            Holiday::ChanukahDay2 => "Chanukah (Day 2 - 2 Candles)",
            Holiday::ChanukahDay3 => "Chanukah (Day 3 - 3 Candles)",
            Holiday::ChanukahDay4 => "Chanukah (Day 4 - 4 Candles)",
            Holiday::ChanukahDay5 => "Chanukah (Day 5 - 5 Candles)",
            Holiday::ChanukahDay6 => "Chanukah (Day 6 - 6 Candles)",
            Holiday::ChanukahDay7 => "Chanukah (Day 7 - 7 Candles)",
            Holiday::ChanukahDay8 => "Chanukah (Day 8 - 8 Candles)",
            Holiday::TuBiShevat => "Tu B'Shevat",
            Holiday::TaanitEsther => "Ta'anit Esther",
            Holiday::Purim => "Purim",
            Holiday::ShushanPurim => "Shushan Purim",
            Holiday::PesachDay1 => "Pesach (Day 1)",
            Holiday::PesachDay2 => "Pesach (Day 2)",
            Holiday::PesachCholHamoedDay1 => "Pesach (Chol HaMoed Day 1)",
            Holiday::PesachCholHamoedDay2 => "Pesach (Chol HaMoed Day 2)",
            Holiday::PesachCholHamoedDay3 => "Pesach (Chol HaMoed Day 3)",
            Holiday::PesachCholHamoedDay4 => "Pesach (Chol HaMoed Day 4)",
            Holiday::PesachDay7 => "Pesach (Day 7)",
            Holiday::PesachDay8 => "Pesach (Day 8)",
            Holiday::LagBaOmer => "Lag BaOmer",
            Holiday::YomHaShoah => "Yom HaShoah",
            Holiday::YomHaZikaron => "Yom HaZikaron",
            Holiday::YomHaAtzmaut => "Yom HaAtzmaut",
            Holiday::YomYerushalayim => "Yom Yerushalayim",
            Holiday::ShavuotDay1 => "Shavuot (Day 1)",
            Holiday::ShavuotDay2 => "Shavuot (Day 2)",
            Holiday::ShivaAsarBTammuz => "Shiva Asar B'Tammuz",
            Holiday::TishaBAv => "Tisha B'Av",
            Holiday::TuBAv => "Tu B'Av",
            Holiday::RoshChodesh => "Rosh Chodesh",
            Holiday::OmerDay1 => "Omer Day 1",
            Holiday::OmerDay2 => "Omer Day 2",
            Holiday::OmerDay3 => "Omer Day 3",
            Holiday::OmerDay4 => "Omer Day 4",
            Holiday::OmerDay5 => "Omer Day 5",
            Holiday::OmerDay6 => "Omer Day 6",
            Holiday::OmerDay7 => "Omer Day 7",
            Holiday::OmerDay8 => "Omer Day 8",
            Holiday::OmerDay9 => "Omer Day 9",
            Holiday::OmerDay10 => "Omer Day 10",
            Holiday::OmerDay11 => "Omer Day 11",
            Holiday::OmerDay12 => "Omer Day 12",
            Holiday::OmerDay13 => "Omer Day 13",
            Holiday::OmerDay14 => "Omer Day 14",
            Holiday::OmerDay15 => "Omer Day 15",
            Holiday::OmerDay16 => "Omer Day 16",
            Holiday::OmerDay17 => "Omer Day 17",
            Holiday::OmerDay18 => "Omer Day 18",
            Holiday::OmerDay19 => "Omer Day 19",
            Holiday::OmerDay20 => "Omer Day 20",
            Holiday::OmerDay21 => "Omer Day 21",
            Holiday::OmerDay22 => "Omer Day 22",
            Holiday::OmerDay23 => "Omer Day 23",
            Holiday::OmerDay24 => "Omer Day 24",
            Holiday::OmerDay25 => "Omer Day 25",
            Holiday::OmerDay26 => "Omer Day 26",
            Holiday::OmerDay27 => "Omer Day 27",
            Holiday::OmerDay28 => "Omer Day 28",
            Holiday::OmerDay29 => "Omer Day 29",
            Holiday::OmerDay30 => "Omer Day 30",
            Holiday::OmerDay31 => "Omer Day 31",
            Holiday::OmerDay32 => "Omer Day 32",
            Holiday::OmerDay33 => "Omer Day 33 (Lag BaOmer)",
            Holiday::OmerDay34 => "Omer Day 34",
            Holiday::OmerDay35 => "Omer Day 35",
            Holiday::OmerDay36 => "Omer Day 36",
            Holiday::OmerDay37 => "Omer Day 37",
            Holiday::OmerDay38 => "Omer Day 38",
            Holiday::OmerDay39 => "Omer Day 39",
            Holiday::OmerDay40 => "Omer Day 40",
            Holiday::OmerDay41 => "Omer Day 41",
            Holiday::OmerDay42 => "Omer Day 42",
            Holiday::OmerDay43 => "Omer Day 43",
            Holiday::OmerDay44 => "Omer Day 44",
            Holiday::OmerDay45 => "Omer Day 45",
            Holiday::OmerDay46 => "Omer Day 46",
            Holiday::OmerDay47 => "Omer Day 47",
            Holiday::OmerDay48 => "Omer Day 48",
            Holiday::OmerDay49 => "Omer Day 49",
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
            Holiday::TishaBAv | Holiday::ShivaAsarBTammuz
        )
    }
}

/// Holiday calculator
pub struct HolidayCalculator;

impl HolidayCalculator {
    /// Get all holidays for a specific Hebrew date
    pub fn get_holidays(date: &HebrewDate) -> Result<Vec<Holiday>, CalendarError> {
        let mut holidays = Vec::new();
        
        // Check for major holidays
        if let Some(holiday) = Self::get_major_holiday(date) {
            holidays.push(holiday);
        }
        
        // Check for Chanukah
        if let Some(chanukah) = Self::get_chanukah_day(date) {
            holidays.push(chanukah);
        }
        
        // Check for Omer
        if let Some(omer) = Self::get_omer_day(date) {
            holidays.push(omer);
        }
        
        // Check for Rosh Chodesh
        if date.day == 1 || date.day == 30 {
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
                10 => Some(Holiday::YomKippur),
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
                _ => None,
            },
            HebrewMonth::Cheshvan => None,
            HebrewMonth::Kislev => {
                // Chanukah handled separately
                None
            },
            HebrewMonth::Teves => {
                // Chanukah and 10 Tevet handled separately
                None
            },
            HebrewMonth::Shevat => {
                if date.day == 15 {
                    Some(Holiday::TuBiShevat)
                } else {
                    None
                }
            },
            HebrewMonth::Adar => {
                if date.day == 13 {
                    Some(Holiday::TaanitEsther)
                } else if date.day == 14 {
                    Some(Holiday::Purim)
                } else if date.day == 15 {
                    Some(Holiday::ShushanPurim)
                } else {
                    None
                }
            },
            HebrewMonth::AdarI => None,
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
            HebrewMonth::Iyar => {
                if date.day == 18 {
                    // Modern holidays - simplified
                    // In reality, these move based on day of week
                    None
                } else {
                    None
                }
            },
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
            HebrewMonth::Av => {
                if date.day == 9 {
                    Some(Holiday::TishaBAv)
                } else if date.day == 15 {
                    Some(Holiday::TuBAv)
                } else {
                    None
                }
            },
            HebrewMonth::Elul => None,
        }
    }
    
    /// Get Chanukah day (if applicable)
    fn get_chanukah_day(date: &HebrewDate) -> Option<Holiday> {
        // Chanukah starts on 25 Kislev
        let is_kislev_25_to_30 = match date.month {
            HebrewMonth::Kislev if date.day >= 25 => true,
            HebrewMonth::Teves if date.day <= 2 || (date.day <= 3 && Self::is_short_kislev(date.year)) => true,
            _ => false,
        };
        
        if !is_kislev_25_to_30 {
            return None;
        }
        
        // Calculate which day of Chanukah
        let day = if date.month == HebrewMonth::Kislev {
            (date.day - 24) as usize
        } else {
            // Teves
            let kislev_days = if Self::is_short_kislev(date.year) { 29 } else { 30 };
            (kislev_days - 24 + date.day) as usize
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
    
    /// Get Omer day (if applicable)
    fn get_omer_day(date: &HebrewDate) -> Option<Holiday> {
        // Omer starts on 16 Nisan and goes for 49 days
        let omer_day = match date.month {
            HebrewMonth::Nisan if date.day >= 16 => (date.day - 15) as usize,
            HebrewMonth::Iyar => (15 + date.day) as usize,
            HebrewMonth::Sivan if date.day <= 5 => (44 + date.day) as usize,
            _ => 0,
        };
        
        if omer_day == 0 || omer_day > 49 {
            return None;
        }
        
        // Map to Holiday enum
        match omer_day {
            1 => Some(Holiday::OmerDay1),
            2 => Some(Holiday::OmerDay2),
            3 => Some(Holiday::OmerDay3),
            4 => Some(Holiday::OmerDay4),
            5 => Some(Holiday::OmerDay5),
            6 => Some(Holiday::OmerDay6),
            7 => Some(Holiday::OmerDay7),
            8 => Some(Holiday::OmerDay8),
            9 => Some(Holiday::OmerDay9),
            10 => Some(Holiday::OmerDay10),
            11 => Some(Holiday::OmerDay11),
            12 => Some(Holiday::OmerDay12),
            13 => Some(Holiday::OmerDay13),
            14 => Some(Holiday::OmerDay14),
            15 => Some(Holiday::OmerDay15),
            16 => Some(Holiday::OmerDay16),
            17 => Some(Holiday::OmerDay17),
            18 => Some(Holiday::OmerDay18),
            19 => Some(Holiday::OmerDay19),
            20 => Some(Holiday::OmerDay20),
            21 => Some(Holiday::OmerDay21),
            22 => Some(Holiday::OmerDay22),
            23 => Some(Holiday::OmerDay23),
            24 => Some(Holiday::OmerDay24),
            25 => Some(Holiday::OmerDay25),
            26 => Some(Holiday::OmerDay26),
            27 => Some(Holiday::OmerDay27),
            28 => Some(Holiday::OmerDay28),
            29 => Some(Holiday::OmerDay29),
            30 => Some(Holiday::OmerDay30),
            31 => Some(Holiday::OmerDay31),
            32 => Some(Holiday::OmerDay32),
            33 => Some(Holiday::OmerDay33), // Lag BaOmer
            34 => Some(Holiday::OmerDay34),
            35 => Some(Holiday::OmerDay35),
            36 => Some(Holiday::OmerDay36),
            37 => Some(Holiday::OmerDay37),
            38 => Some(Holiday::OmerDay38),
            39 => Some(Holiday::OmerDay39),
            40 => Some(Holiday::OmerDay40),
            41 => Some(Holiday::OmerDay41),
            42 => Some(Holiday::OmerDay42),
            43 => Some(Holiday::OmerDay43),
            44 => Some(Holiday::OmerDay44),
            45 => Some(Holiday::OmerDay45),
            46 => Some(Holiday::OmerDay46),
            47 => Some(Holiday::OmerDay47),
            48 => Some(Holiday::OmerDay48),
            49 => Some(Holiday::OmerDay49),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::{DateConverter, HebrewMonth};
    use chrono::NaiveDate;
    
    #[test]
    fn test_rosh_hashanah() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 1);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::RoshHashanahDay1));
        assert!(holidays.contains(&Holiday::RoshChodesh));
    }
    
    #[test]
    fn test_yom_kippur() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 10);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::YomKippur));
    }
    
    #[test]
    fn test_pesach() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Nisan, 15);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::PesachDay1));
        // 15 Nisan is NOT Rosh Chodesh (Rosh Chodesh is day 1 or 30)
        assert!(!holidays.contains(&Holiday::RoshChodesh));
    }
    
    #[test]
    fn test_omer() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Nisan, 16);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::OmerDay1));
        
        let lag_baomer = HebrewDate::new(5784, HebrewMonth::Iyar, 18);
        let holidays = HolidayCalculator::get_holidays(&lag_baomer).unwrap();
        assert!(holidays.contains(&Holiday::OmerDay33));
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

    #[test]
    fn test_after_sukkot_no_holiday() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 24);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        let has_major = holidays.iter().any(|h| !matches!(h, Holiday::RoshChodesh | Holiday::OmerDay1 | Holiday::OmerDay2 | Holiday::OmerDay3 | Holiday::OmerDay4 | Holiday::OmerDay5 | Holiday::OmerDay6 | Holiday::OmerDay7 | Holiday::OmerDay8 | Holiday::OmerDay9 | Holiday::OmerDay10 | Holiday::OmerDay11 | Holiday::OmerDay12 | Holiday::OmerDay13 | Holiday::OmerDay14 | Holiday::OmerDay15 | Holiday::OmerDay16 | Holiday::OmerDay17 | Holiday::OmerDay18 | Holiday::OmerDay19 | Holiday::OmerDay20 | Holiday::OmerDay21 | Holiday::OmerDay22 | Holiday::OmerDay23 | Holiday::OmerDay24 | Holiday::OmerDay25 | Holiday::OmerDay26 | Holiday::OmerDay27 | Holiday::OmerDay28 | Holiday::OmerDay29 | Holiday::OmerDay30 | Holiday::OmerDay31 | Holiday::OmerDay32 | Holiday::OmerDay33 | Holiday::OmerDay34 | Holiday::OmerDay35 | Holiday::OmerDay36 | Holiday::OmerDay37 | Holiday::OmerDay38 | Holiday::OmerDay39 | Holiday::OmerDay40 | Holiday::OmerDay41 | Holiday::OmerDay42 | Holiday::OmerDay43 | Holiday::OmerDay44 | Holiday::OmerDay45 | Holiday::OmerDay46 | Holiday::OmerDay47 | Holiday::OmerDay48 | Holiday::OmerDay49));
        assert!(!has_major, "Tishrei 24 should have no major holiday");
    }

    // === Chanukah detailed ===

    #[test]
    fn test_chanukah_all_8_days_short_kislev() {
        // 5784 is a deficient leap year (Kislev has 29 days)
        // Days 1-5: Kislev 25-29, Days 6-8: Teves 1-3
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
        // Days 1-6: Kislev 25-30, Days 7-8: Teves 1-2
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
        // 5784 is a leap year; Purim is on 14 Adar (= Adar II)
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
        // In a leap year, Adar I 14 should NOT have Purim
        let hebrew = HebrewDate::new(5784, HebrewMonth::AdarI, 14);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(!holidays.contains(&Holiday::Purim),
            "Adar I 14 in a leap year should not have Purim");
        assert!(!holidays.contains(&Holiday::TaanitEsther),
            "Adar I 13 pattern should not match in Adar I");
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
    fn test_rosh_chodesh_day_30() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Tishrei, 30);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::RoshChodesh));
    }

    #[test]
    fn test_no_rosh_chodesh_mid_month() {
        let hebrew = HebrewDate::new(5784, HebrewMonth::Cheshvan, 15);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(!holidays.contains(&Holiday::RoshChodesh));
    }

    // === Omer boundaries ===

    #[test]
    fn test_omer_last_day_nisan() {
        // Nisan 30 = Omer Day 15
        let hebrew = HebrewDate::new(5784, HebrewMonth::Nisan, 30);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::OmerDay15),
            "Nisan 30 should be Omer Day 15");
    }

    #[test]
    fn test_omer_iyar_1() {
        // Iyar 1 = Omer Day 16
        let hebrew = HebrewDate::new(5784, HebrewMonth::Iyar, 1);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::OmerDay16));
    }

    #[test]
    fn test_omer_sivan_1() {
        // Sivan 1 = Omer Day 45
        let hebrew = HebrewDate::new(5784, HebrewMonth::Sivan, 1);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::OmerDay45));
    }

    #[test]
    fn test_omer_day_49() {
        // Sivan 5 = Omer Day 49
        let hebrew = HebrewDate::new(5784, HebrewMonth::Sivan, 5);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        assert!(holidays.contains(&Holiday::OmerDay49));
    }

    #[test]
    fn test_no_omer_sivan_6() {
        // Sivan 6 is Shavuot, not Omer
        let hebrew = HebrewDate::new(5784, HebrewMonth::Sivan, 6);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        let has_omer = holidays.iter().any(|h| {
            let name = h.name();
            name.starts_with("Omer")
        });
        assert!(!has_omer, "Sivan 6 (Shavuot) should not have Omer");
    }

    #[test]
    fn test_no_omer_nisan_15() {
        // Nisan 15 is Pesach, before Omer starts
        let hebrew = HebrewDate::new(5784, HebrewMonth::Nisan, 15);
        let holidays = HolidayCalculator::get_holidays(&hebrew).unwrap();
        let has_omer = holidays.iter().any(|h| {
            let name = h.name();
            name.starts_with("Omer")
        });
        assert!(!has_omer, "Nisan 15 (Pesach Day 1) should not have Omer");
    }

    // === Trait methods ===

    #[test]
    fn test_is_yom_tov() {
        assert!(Holiday::RoshHashanahDay1.is_yom_tov());
        assert!(Holiday::YomKippur.is_yom_tov());
        assert!(Holiday::SukkotDay1.is_yom_tov());
        assert!(Holiday::PesachDay1.is_yom_tov());
        assert!(Holiday::ShavuotDay1.is_yom_tov());
        // Negatives
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
        // Negatives
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
        // Negatives
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
        assert_eq!(Holiday::OmerDay33.name(), "Omer Day 33 (Lag BaOmer)");
    }
}
