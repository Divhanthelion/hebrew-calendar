//! Parsha (Torah Portion) Calculation Module
//! 
//! Implements the calculation of weekly Torah portions based on Hebrew calendar rules.

use serde::{Deserialize, Serialize};

use crate::calendar::{DateConverter, HebrewDate, HebrewMonth};
use crate::CalendarError;
use chrono::Datelike;

/// Torah portion (Parsha)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Parsha {
    Bereshit,
    Noach,
    LechLecha,
    Vayera,
    ChayeiSara,
    Toldot,
    Vayetzei,
    Vayishlach,
    Vayeshev,
    Miketz,
    Vayigash,
    Vayechi,
    Shemot,
    Vaera,
    Bo,
    Beshalach,
    Yitro,
    Mishpatim,
    Terumah,
    Tetzaveh,
    KiTisa,
    Vayakhel,
    Pekudei,
    Vayikra,
    Tzav,
    Shemini,
    Tazria,
    Metzora,
    AchreiMot,
    Kedoshim,
    Emor,
    Behar,
    Bechukotai,
    Bamidbar,
    Nasso,
    Behaalotecha,
    Shelach,
    Korach,
    Chukat,
    Balak,
    Pinchas,
    Matot,
    Masei,
    Devarim,
    Vaetchanan,
    Eikev,
    Reeh,
    Shoftim,
    KiTeitzei,
    KiTavo,
    Nitzavim,
    Vayeilech,
    HaAzinu,
    VezotHaberacha,
    // Special readings
    VayakhelPekudei,  // Combined
    TazriaMetzora,    // Combined
    AchreiMotKedoshim, // Combined
    BeharBechukotai,  // Combined
    ChukatBalak,      // Combined (in Israel)
    MatotMasei,       // Combined
    NitzavimVayeilech, // Combined
    HaftarahOnly,     // When no regular parsha
}

impl Parsha {
    /// Get the English name
    pub fn name(&self) -> &'static str {
        match self {
            Parsha::Bereshit => "Bereshit",
            Parsha::Noach => "Noach",
            Parsha::LechLecha => "Lech Lecha",
            Parsha::Vayera => "Vayera",
            Parsha::ChayeiSara => "Chayei Sara",
            Parsha::Toldot => "Toldot",
            Parsha::Vayetzei => "Vayetzei",
            Parsha::Vayishlach => "Vayishlach",
            Parsha::Vayeshev => "Vayeshev",
            Parsha::Miketz => "Miketz",
            Parsha::Vayigash => "Vayigash",
            Parsha::Vayechi => "Vayechi",
            Parsha::Shemot => "Shemot",
            Parsha::Vaera => "Vaera",
            Parsha::Bo => "Bo",
            Parsha::Beshalach => "Beshalach",
            Parsha::Yitro => "Yitro",
            Parsha::Mishpatim => "Mishpatim",
            Parsha::Terumah => "Terumah",
            Parsha::Tetzaveh => "Tetzaveh",
            Parsha::KiTisa => "Ki Tisa",
            Parsha::Vayakhel => "Vayakhel",
            Parsha::Pekudei => "Pekudei",
            Parsha::Vayikra => "Vayikra",
            Parsha::Tzav => "Tzav",
            Parsha::Shemini => "Shemini",
            Parsha::Tazria => "Tazria",
            Parsha::Metzora => "Metzora",
            Parsha::AchreiMot => "Achrei Mot",
            Parsha::Kedoshim => "Kedoshim",
            Parsha::Emor => "Emor",
            Parsha::Behar => "Behar",
            Parsha::Bechukotai => "Bechukotai",
            Parsha::Bamidbar => "Bamidbar",
            Parsha::Nasso => "Nasso",
            Parsha::Behaalotecha => "Behaalotecha",
            Parsha::Shelach => "Shelach",
            Parsha::Korach => "Korach",
            Parsha::Chukat => "Chukat",
            Parsha::Balak => "Balak",
            Parsha::Pinchas => "Pinchas",
            Parsha::Matot => "Matot",
            Parsha::Masei => "Masei",
            Parsha::Devarim => "Devarim",
            Parsha::Vaetchanan => "Vaetchanan",
            Parsha::Eikev => "Eikev",
            Parsha::Reeh => "Reeh",
            Parsha::Shoftim => "Shoftim",
            Parsha::KiTeitzei => "Ki Teitzei",
            Parsha::KiTavo => "Ki Tavo",
            Parsha::Nitzavim => "Nitzavim",
            Parsha::Vayeilech => "Vayeilech",
            Parsha::HaAzinu => "HaAzinu",
            Parsha::VezotHaberacha => "Vezot Haberacha",
            Parsha::VayakhelPekudei => "Vayakhel-Pekudei",
            Parsha::TazriaMetzora => "Tazria-Metzora",
            Parsha::AchreiMotKedoshim => "Achrei Mot-Kedoshim",
            Parsha::BeharBechukotai => "Behar-Bechukotai",
            Parsha::ChukatBalak => "Chukat-Balak",
            Parsha::MatotMasei => "Matot-Masei",
            Parsha::NitzavimVayeilech => "Nitzavim-Vayeilech",
            Parsha::HaftarahOnly => "Haftarah Only",
        }
    }
    
    /// Get the Hebrew name
    pub fn hebrew_name(&self) -> &'static str {
        match self {
            Parsha::Bereshit => "בראשית",
            Parsha::Noach => "נח",
            Parsha::LechLecha => "לך לך",
            Parsha::Vayera => "וירא",
            Parsha::ChayeiSara => "חיי שרה",
            Parsha::Toldot => "תולדות",
            Parsha::Vayetzei => "ויצא",
            Parsha::Vayishlach => "וישלח",
            Parsha::Vayeshev => "וישב",
            Parsha::Miketz => "מקץ",
            Parsha::Vayigash => "ויגש",
            Parsha::Vayechi => "ויחי",
            Parsha::Shemot => "שמות",
            Parsha::Vaera => "וארא",
            Parsha::Bo => "בא",
            Parsha::Beshalach => "בשלח",
            Parsha::Yitro => "יתרו",
            Parsha::Mishpatim => "משפטים",
            Parsha::Terumah => "תרומה",
            Parsha::Tetzaveh => "תצוה",
            Parsha::KiTisa => "כי תשא",
            Parsha::Vayakhel => "ויקהל",
            Parsha::Pekudei => "פקודי",
            Parsha::Vayikra => "ויקרא",
            Parsha::Tzav => "צו",
            Parsha::Shemini => "שמיני",
            Parsha::Tazria => "תזריע",
            Parsha::Metzora => "מצורע",
            Parsha::AchreiMot => "אחרי מות",
            Parsha::Kedoshim => "קדושים",
            Parsha::Emor => "אמור",
            Parsha::Behar => "בהר",
            Parsha::Bechukotai => "בחקותי",
            Parsha::Bamidbar => "במדבר",
            Parsha::Nasso => "נשא",
            Parsha::Behaalotecha => "בהעלותך",
            Parsha::Shelach => "שלח",
            Parsha::Korach => "קרח",
            Parsha::Chukat => "חקת",
            Parsha::Balak => "בלק",
            Parsha::Pinchas => "פינחס",
            Parsha::Matot => "מטות",
            Parsha::Masei => "מסעי",
            Parsha::Devarim => "דברים",
            Parsha::Vaetchanan => "ואתחנן",
            Parsha::Eikev => "עקב",
            Parsha::Reeh => "ראה",
            Parsha::Shoftim => "שופטים",
            Parsha::KiTeitzei => "כי תצא",
            Parsha::KiTavo => "כי תבוא",
            Parsha::Nitzavim => "נצבים",
            Parsha::Vayeilech => "וילך",
            Parsha::HaAzinu => "האזינו",
            Parsha::VezotHaberacha => "וזאת הברכה",
            _ => "",
        }
    }
}

/// Parsha calculator
pub struct ParshaCalculator;

impl ParshaCalculator {
    /// Get the parsha for a Shabbat
    pub fn get_parsha(date: &HebrewDate) -> Result<Parsha, CalendarError> {
        // Find the Shabbat of this date
        let shabbat_date = Self::find_shabbat(date)?;
        
        // Calculate based on the Hebrew year cycle
        Self::calculate_parsha_for_shabbat(shabbat_date)
    }
    
    /// Find the Shabbat containing this date
    fn find_shabbat(date: &HebrewDate) -> Result<HebrewDate, CalendarError> {
        // Convert to Gregorian to find day of week
        let gregorian = DateConverter::hebrew_to_gregorian(*date)?;
        let weekday = gregorian.weekday().num_days_from_sunday();
        
        // Shabbat is day 6 (0-indexed from Sunday = 0)
        if weekday == 6 {
            return Ok(*date);
        }
        
        // Calculate days to add
        let days_to_add = 6 - weekday as i64;
        let shabbat_gregorian = gregorian + chrono::Duration::days(days_to_add);
        DateConverter::gregorian_to_hebrew(shabbat_gregorian)
    }
    
    /// Calculate the parsha for a Shabbat
    fn calculate_parsha_for_shabbat(date: HebrewDate) -> Result<Parsha, CalendarError> {
        let year = date.year;
        let is_leap = DateConverter::is_hebrew_leap_year(year);

        // Find Rosh Hashanah of this year
        let rosh_hashanah = HebrewDate::new(year, HebrewMonth::Tishrei, 1);
        let rosh_gregorian = DateConverter::hebrew_to_gregorian(rosh_hashanah)?;
        let rh_weekday = rosh_gregorian.weekday().num_days_from_sunday();

        // Find Simchat Torah (Tishrei 23 in diaspora)
        let simchat_torah = HebrewDate::new(year, HebrewMonth::Tishrei, 23);
        let simchat_gregorian = DateConverter::hebrew_to_gregorian(simchat_torah)?;
        let st_weekday = simchat_gregorian.weekday().num_days_from_sunday();

        // Find the first Shabbat after Simchat Torah (Shabbat Bereshit)
        let days_to_shabbat = if st_weekday == 6 {
            7 // If Simchat Torah is Shabbat, Bereshit is next week
        } else {
            (6 - st_weekday as i64 + 7) % 7
        };
        let bereshit_shabbat = simchat_gregorian + chrono::Duration::days(days_to_shabbat);

        // Count weeks from Shabbat Bereshit to current Shabbat
        let current_gregorian = DateConverter::hebrew_to_gregorian(date)?;
        let weeks_diff = (current_gregorian - bereshit_shabbat).num_days() / 7;

        if weeks_diff < 0 {
            // Before Bereshit (during Tishrei holidays)
            return Ok(Parsha::HaftarahOnly);
        }

        // Get the base parsha index based on year type
        let parsha_index = Self::get_parsha_index(weeks_diff as usize, is_leap, rh_weekday, year);

        Ok(parsha_index)
    }
    
    /// Get the parsha index based on week number and year type
    fn get_parsha_index(week: usize, is_leap: bool, rh_weekday: u32, year: i32) -> Parsha {
        // Standard sequence of parshiot
        let standard_sequence: Vec<Parsha> = vec![
            Parsha::Bereshit, Parsha::Noach, Parsha::LechLecha, Parsha::Vayera,
            Parsha::ChayeiSara, Parsha::Toldot, Parsha::Vayetzei, Parsha::Vayishlach,
            Parsha::Vayeshev, Parsha::Miketz, Parsha::Vayigash, Parsha::Vayechi,
            Parsha::Shemot, Parsha::Vaera, Parsha::Bo, Parsha::Beshalach,
            Parsha::Yitro, Parsha::Mishpatim, Parsha::Terumah, Parsha::Tetzaveh,
            Parsha::KiTisa, Parsha::Vayakhel, Parsha::Pekudei, Parsha::Vayikra,
            Parsha::Tzav, Parsha::Shemini, Parsha::Tazria, Parsha::Metzora,
            Parsha::AchreiMot, Parsha::Kedoshim, Parsha::Emor, Parsha::Behar,
            Parsha::Bechukotai, Parsha::Bamidbar, Parsha::Nasso, Parsha::Behaalotecha,
            Parsha::Shelach, Parsha::Korach, Parsha::Chukat, Parsha::Balak,
            Parsha::Pinchas, Parsha::Matot, Parsha::Masei, Parsha::Devarim,
            Parsha::Vaetchanan, Parsha::Eikev, Parsha::Reeh, Parsha::Shoftim,
            Parsha::KiTeitzei, Parsha::KiTavo, Parsha::Nitzavim, Parsha::Vayeilech,
            Parsha::HaAzinu,
        ];
        
        // For leap years or special configurations, parshiot are combined
        // This is a simplified version - full implementation would handle all edge cases
        
        let adjusted_week = Self::adjust_for_combined_parshiot(week, is_leap, rh_weekday, year);
        
        if adjusted_week < standard_sequence.len() {
            standard_sequence[adjusted_week]
        } else if adjusted_week == standard_sequence.len() {
            Parsha::VezotHaberacha
        } else {
            Parsha::HaftarahOnly
        }
    }
    
    /// Adjust week number for combined parshiot
    fn adjust_for_combined_parshiot(week: usize, is_leap: bool, rh_weekday: u32, year: i32) -> usize {
        // In leap years, fewer parshiot are combined
        // In common years starting on certain days, more combinations occur
        
        // Special handling based on year configuration
        let _year_type = DateConverter::hebrew_year_type(year);
        
        // Simplified combination rules:
        // In Israel, Chukat and Balak are often combined in common years
        // In diaspora, they are usually separate
        
        // This is a basic implementation - a full implementation would have
        // detailed tables for all year configurations
        
        match (is_leap, rh_weekday, week) {
            // Vayakhel-Pekudei combination
            (_, _, 21) if !is_leap && week > 20 => week - 1,
            
            // Tazria-Metzora combination
            (_, _, 26) if !is_leap && week > 25 => week - 1,
            
            // Achrei Mot-Kedoshim combination
            (_, _, 29) if !is_leap && week > 28 => week - 1,
            
            // Behar-Bechukotai combination
            (_, _, 32) if !is_leap && week > 31 => week - 1,
            
            // Matot-Masei combination
            (_, _, 41) if !is_leap && week > 40 => week - 1,
            
            // Nitzavim-Vayeilech combination
            (false, _, 50) => 49, // Combined
            
            _ => week,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    
    #[test]
    fn test_parsha_calculation() {
        // Test a known Shabbat
        let date = HebrewDate::new(5784, HebrewMonth::Tishrei, 21); // Shabbat Bereishit 5784
        let parsha = ParshaCalculator::get_parsha(&date).unwrap();
        println!("Parsha for Tishrei 21, 5784: {:?}", parsha);
        // This should be Bereshit or Noach depending on year configuration
    }
    
    #[test]
    fn test_find_shabbat() {
        // Tishrei 15, 5784 = Saturday Sept 30, 2023, already a Shabbat
        let shabbat_date = HebrewDate::new(5784, HebrewMonth::Tishrei, 15);
        let shabbat = ParshaCalculator::find_shabbat(&shabbat_date).unwrap();
        assert_eq!(shabbat.day, 15);

        // Tishrei 16, 5784 = Sunday Oct 1, 2023 → next Shabbat is Tishrei 22
        let sunday = HebrewDate::new(5784, HebrewMonth::Tishrei, 16);
        let shabbat = ParshaCalculator::find_shabbat(&sunday).unwrap();
        assert_eq!(shabbat.day, 22);
    }
    
    #[test]
    fn test_parsha_names() {
        assert_eq!(Parsha::Bereshit.name(), "Bereshit");
        assert_eq!(Parsha::Bereshit.hebrew_name(), "בראשית");
    }

    #[test]
    fn test_shabbat_bereishit_5784() {
        // Tishrei 28, 5784 = Oct 13, 2023 (Shabbat) = Parashat Bereshit
        let date = HebrewDate::new(5784, HebrewMonth::Tishrei, 28);
        let parsha = ParshaCalculator::get_parsha(&date).unwrap();
        assert_eq!(parsha, Parsha::Bereshit, "Tishrei 28, 5784 should be Bereshit");
    }

    #[test]
    fn test_shabbat_noach_5784() {
        // Cheshvan 6, 5784 should be Noach
        let date = HebrewDate::new(5784, HebrewMonth::Cheshvan, 6);
        let parsha = ParshaCalculator::get_parsha(&date).unwrap();
        assert_eq!(parsha, Parsha::Noach, "Cheshvan 6, 5784 should be Noach");
    }

    #[test]
    fn test_find_shabbat_monday() {
        // Tishrei 3, 5784 = Monday Sept 18, 2023 → next Shabbat = Tishrei 8
        let monday = HebrewDate::new(5784, HebrewMonth::Tishrei, 3);
        let shabbat = ParshaCalculator::find_shabbat(&monday).unwrap();
        assert_eq!(shabbat.day, 8, "Monday Tishrei 3 should find Shabbat Tishrei 8");
    }

    #[test]
    fn test_find_shabbat_friday() {
        // Tishrei 14, 5784 = Friday Sept 29, 2023 → next Shabbat = Tishrei 15
        let friday = HebrewDate::new(5784, HebrewMonth::Tishrei, 14);
        let shabbat = ParshaCalculator::find_shabbat(&friday).unwrap();
        assert_eq!(shabbat.day, 15, "Friday Tishrei 14 should find Shabbat Tishrei 15");
    }

    #[test]
    fn test_parsha_full_year_no_panic() {
        // Iterate every Shabbat in 5784, assert get_parsha doesn't panic
        use crate::calendar::DateConverter;
        let rh = DateConverter::rosh_hashanah(5784);
        let rh_next = DateConverter::rosh_hashanah(5785);
        let start = DateConverter::rd_to_gregorian(rh).unwrap();
        let end = DateConverter::rd_to_gregorian(rh_next).unwrap();

        let mut current = start;
        // Advance to first Saturday
        while current.weekday().num_days_from_sunday() != 6 {
            current = current.succ_opt().unwrap();
        }
        while current < end {
            let hebrew = DateConverter::gregorian_to_hebrew(current).unwrap();
            let _parsha = ParshaCalculator::get_parsha(&hebrew).unwrap();
            current = current + chrono::Duration::days(7);
        }
    }

    #[test]
    fn test_parsha_common_year() {
        // 5785 is a common year; verify a known Shabbat doesn't crash
        use crate::calendar::DateConverter;
        let rh = DateConverter::rosh_hashanah(5785);
        let start = DateConverter::rd_to_gregorian(rh).unwrap();
        // Find first Shabbat
        let mut current = start;
        while current.weekday().num_days_from_sunday() != 6 {
            current = current.succ_opt().unwrap();
        }
        let hebrew = DateConverter::gregorian_to_hebrew(current).unwrap();
        let _parsha = ParshaCalculator::get_parsha(&hebrew).unwrap();
    }
}
