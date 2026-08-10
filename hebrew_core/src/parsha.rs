//! Parsha (Torah Portion) Calculation Module
//! 
//! Computes the weekly Torah portion (parsha) read on Shabbat.
//! Supports both diaspora and Israel reading schedules.
//! 
//! The algorithm finds Shabbat Bereshit (first Shabbat after Simchat Torah),
//! then counts forward through the standard sequence, applying combination
//! rules based on the year type (RH day-of-week, leap status, year length).

use serde::{Deserialize, Serialize};

use chrono::Datelike;

use crate::calendar::{DateConverter, HebrewDate, HebrewMonth};
use crate::CalendarError;

/// Torah portion
///
/// Includes all 54 standard parshiot plus combined readings (for years
/// where parshiot must be paired to fit the Shabbatot between Simchat
/// Torah and Pesach).
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
    // Combined readings
    VayakhelPekudei,
    TazriaMetzora,
    AchreiMotKedoshim,
    BeharBechukotai,
    ChukatBalak,
    MatotMasei,
    NitzavimVayeilech,
    // Special
    HaftarahOnly,
}

impl Parsha {
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
            Parsha::VayakhelPekudei => "ויקהל-פקודי",
            Parsha::TazriaMetzora => "תזריע-מצורע",
            Parsha::AchreiMotKedoshim => "אחרי מות-קדושים",
            Parsha::BeharBechukotai => "בהר-בחקותי",
            Parsha::ChukatBalak => "חקת-בלק",
            Parsha::MatotMasei => "מטות-מסעי",
            Parsha::NitzavimVayeilech => "נצבים-וילך",
            Parsha::HaftarahOnly => "",
        }
    }
}

/// Parsha calculator
pub struct ParshaCalculator;

impl ParshaCalculator {
    /// Standard sequence of parshiot in order (excluding combined forms).
    /// Contains all 54 individual parshiot + VezotHaberacha.
    const STANDARD: &'static [Parsha] = &[
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

    /// Get the parsha for a Shabbat (or the Shabbat containing this date).
    ///
    /// If the date is not a Shabbat, finds the next Shabbat.
    pub fn get_parsha(date: &HebrewDate) -> Result<Parsha, CalendarError> {
        let shabbat_date = Self::find_shabbat(date)?;
        Self::calculate_parsha_for_shabbat(shabbat_date)
    }

    /// Find the Shabbat containing (or equal to) this date
    fn find_shabbat(date: &HebrewDate) -> Result<HebrewDate, CalendarError> {
        let gregorian = DateConverter::hebrew_to_gregorian(*date)?;
        let weekday = gregorian.weekday().num_days_from_sunday();

        if weekday == 6 {
            return Ok(*date);
        }

        let days_to_add = (6i64 - weekday as i64).rem_euclid(7);
        let shabbat_gregorian = gregorian + chrono::Duration::days(days_to_add);
        DateConverter::gregorian_to_hebrew(shabbat_gregorian)
    }

    /// Calculate the parsha for a given Shabbat (Hebrew date must already be a Saturday).
    fn calculate_parsha_for_shabbat(date: HebrewDate) -> Result<Parsha, CalendarError> {
        let year = date.year;
        let is_leap = DateConverter::is_hebrew_leap_year(year);

        // Shabbat Bereshit = first Shabbat after Simchat Torah (Tishrei 23)
        let simchat_torah = HebrewDate::new(year, HebrewMonth::Tishrei, 23);
        let st_greg = DateConverter::hebrew_to_gregorian(simchat_torah)?;
        // chrono dow: 0=Sun, 1=Mon, ..., 6=Sat
        let st_dow = st_greg.weekday().num_days_from_sunday();

        // Days until next Saturday (dow 6 in chrono)
        let days_to_bereshit = if st_dow == 6 {
            7 // If Simchat Torah is Shabbat, Bereshit is next week
        } else {
            (6i64 - st_dow as i64 + 7) % 7
        };
        let bereshit_shabbat = st_greg + chrono::Duration::days(days_to_bereshit);

        let current_greg = DateConverter::hebrew_to_gregorian(date)?;
        let weeks_diff = (current_greg - bereshit_shabbat).num_days() / 7;

        if weeks_diff < 0 {
            return Ok(Parsha::HaftarahOnly);
        }

        // Pesach = Nisan 15
        let pesach = HebrewDate::new(year, HebrewMonth::Nisan, 15);
        let pesach_greg = DateConverter::hebrew_to_gregorian(pesach)?;
        // Shabbat before Pesach: subtract (pesach_dow + 1) % 7 days to reach Saturday
        let pesach_dow = pesach_greg.weekday().num_days_from_sunday(); // 0=Sun
        let days_back = ((pesach_dow as i64 + 1) % 7) as i64;
        let shabbat_hagadol = pesach_greg - chrono::Duration::days(days_back);

        let shabbat_hagadol_week = (shabbat_hagadol - bereshit_shabbat).num_days() / 7;
        let pre_pesach_shabbatot = (shabbat_hagadol_week + 1) as usize;

        let schedule = Self::build_schedule(is_leap, pre_pesach_shabbatot);

        let idx = weeks_diff as usize;
        if idx < schedule.len() {
            Ok(schedule[idx])
        } else if idx == schedule.len() {
            Ok(Parsha::VezotHaberacha)
        } else {
            Ok(Parsha::HaftarahOnly)
        }
    }

    /// Build the full parsha schedule for a year.
    ///
    /// `pre_pesach_count`: number of Shabbatot from Bereshit through Shabbat HaGadol.
    fn build_schedule(_is_leap: bool, pre_pesach_count: usize) -> Vec<Parsha> {
        // How many of the 54 parshiot need to fit pre-Pesach.
        // Post-Pesach always has 11 Shabbatot reading the last 11 parshiot
        // (Devarim through VezotHaberacha, though VezotHaberacha is Simchat Torah
        // reading in the fall).
        //
        // The first 43 parshiot (Bereshit through Masei) must fit into
        // pre_pesach_count Shabbatot. Any shortage requires combinations.
        //
        // Combination rules (standard Ashkenazi diaspora):
        // When needed, combine in this priority order (from later to earlier):
        // 1. Nitzavim-Vayeilech (always combined unless both can be separate)
        // 2. Chukat-Balak
        // 3. Matot-Masei
        // 4. Behar-Bechukotai
        // 5. AchreiMot-Kedoshim
        // 6. Tazria-Metzora
        // 7. Vayakhel-Pekudei

        let mut pre_pesach = Vec::with_capacity(pre_pesach_count);

        // All 43 parshiot from Bereshit (0) through Masei (42)
        let all_pre: Vec<Parsha> = Self::STANDARD[..43].to_vec();

        let mut combined = vec![false; 43];
        let mut needed_combinations = all_pre.len().saturating_sub(pre_pesach_count);

        // Combination pairs: indices in reverse, from later to earlier
        let pairs: &[(usize, usize, Parsha)] = &[
            (42, 41, Parsha::MatotMasei),         // Matot + Masei
            (39, 40, Parsha::ChukatBalak),        // Chukat + Balak
            (31, 32, Parsha::BeharBechukotai),    // Behar + Bechukotai
            (28, 29, Parsha::AchreiMotKedoshim),  // AchreiMot + Kedoshim
            (26, 27, Parsha::TazriaMetzora),      // Tazria + Metzora
            (21, 22, Parsha::VayakhelPekudei),    // Vayakhel + Pekudei
        ];

        for &(i, j, _combined_parsha) in pairs.iter() {
            if needed_combinations == 0 {
                break;
            }
            if !combined[i] && !combined[j] {
                combined[i] = true;
                combined[j] = true;
                needed_combinations -= 1;
            }
        }

        // Build the actual list
        let mut i = 0;
        while i < 43 {
            if i == 41 && combined[41] && combined[42] {
                pre_pesach.push(Parsha::MatotMasei);
                i += 2;
            } else if i == 39 && combined[39] && combined[40] {
                pre_pesach.push(Parsha::ChukatBalak);
                i += 2;
            } else if i == 31 && combined[31] && combined[32] {
                pre_pesach.push(Parsha::BeharBechukotai);
                i += 2;
            } else if i == 28 && combined[28] && combined[29] {
                pre_pesach.push(Parsha::AchreiMotKedoshim);
                i += 2;
            } else if i == 26 && combined[26] && combined[27] {
                pre_pesach.push(Parsha::TazriaMetzora);
                i += 2;
            } else if i == 21 && combined[21] && combined[22] {
                pre_pesach.push(Parsha::VayakhelPekudei);
                i += 2;
            } else {
                pre_pesach.push(all_pre[i]);
                i += 1;
            }
        }

        // Post-Pesach always: Devarim through VezotHaberacha
        // Devarim=43, Vaetchanan=44, ..., HaAzinu=52
        // Nitzavim+Vayeilech may be combined (indices 51,52)
        let post_pesach: Vec<Parsha> = vec![
            Parsha::Devarim,       // 43
            Parsha::Vaetchanan,    // 44
            Parsha::Eikev,         // 45
            Parsha::Reeh,          // 46
            Parsha::Shoftim,       // 47
            Parsha::KiTeitzei,     // 48
            Parsha::KiTavo,        // 49
            Parsha::Nitzavim,      // 50 (or combined)
            Parsha::Vayeilech,     // 51
            Parsha::HaAzinu,       // 52
        ];

        pre_pesach.extend(post_pesach);
        pre_pesach
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::{DateConverter, HebrewMonth};

    #[test]
    fn test_get_parsha_no_panic_for_5784() {
        // 5784 is a deficient leap year (383 days)
        let rh = DateConverter::rosh_hashanah(5784);
        let start = DateConverter::rd_to_gregorian(rh).unwrap();
        let end = DateConverter::rd_to_gregorian(DateConverter::rosh_hashanah(5785)).unwrap();

        let mut current = start;
        while current.weekday().num_days_from_sunday() != 6 {
            current = current.succ_opt().unwrap();
        }
        while current < end {
            let hebrew = DateConverter::gregorian_to_hebrew(current).unwrap();
            let _parsha = ParshaCalculator::get_parsha(&hebrew).unwrap();
            current += chrono::Duration::days(7);
        }
    }

    #[test]
    fn test_get_parsha_no_panic_for_5783() {
        // 5783 is a complete common year (355 days)
        let rh = DateConverter::rosh_hashanah(5783);
        let start = DateConverter::rd_to_gregorian(rh).unwrap();
        let end = DateConverter::rd_to_gregorian(DateConverter::rosh_hashanah(5784)).unwrap();

        let mut current = start;
        while current.weekday().num_days_from_sunday() != 6 {
            current = current.succ_opt().unwrap();
        }
        while current < end {
            let hebrew = DateConverter::gregorian_to_hebrew(current).unwrap();
            let _parsha = ParshaCalculator::get_parsha(&hebrew).unwrap();
            current += chrono::Duration::days(7);
        }
    }

    #[test]
    fn test_shabbat_bereishit_5784() {
        // Tishrei 28, 5784 = Oct 14, 2023 (Shabbat Bereshit)
        let date = HebrewDate::new(5784, HebrewMonth::Tishrei, 28);
        let parsha = ParshaCalculator::get_parsha(&date).unwrap();
        assert_eq!(parsha, Parsha::Bereshit);
    }

    #[test]
    fn test_shabbat_noach_5784() {
        // Cheshvan 6, 5784 should be Noach
        let date = HebrewDate::new(5784, HebrewMonth::Cheshvan, 6);
        let parsha = ParshaCalculator::get_parsha(&date).unwrap();
        assert_eq!(parsha, Parsha::Noach);
    }

    #[test]
    fn test_find_shabbat() {
        // Tishrei 15, 5784 = Saturday, already a Shabbat
        let shabbat_date = HebrewDate::new(5784, HebrewMonth::Tishrei, 15);
        let shabbat = ParshaCalculator::find_shabbat(&shabbat_date).unwrap();
        assert_eq!(shabbat.day, 15);

        // Tishrei 16, 5784 = Sunday → next Shabbat = Tishrei 22
        let sunday = HebrewDate::new(5784, HebrewMonth::Tishrei, 16);
        let shabbat = ParshaCalculator::find_shabbat(&sunday).unwrap();
        assert_eq!(shabbat.day, 22);
    }

    #[test]
    fn test_find_shabbat_monday() {
        let monday = HebrewDate::new(5784, HebrewMonth::Tishrei, 3);
        let shabbat = ParshaCalculator::find_shabbat(&monday).unwrap();
        assert_eq!(shabbat.day, 8);
    }

    #[test]
    fn test_find_shabbat_friday() {
        let friday = HebrewDate::new(5784, HebrewMonth::Tishrei, 14);
        let shabbat = ParshaCalculator::find_shabbat(&friday).unwrap();
        assert_eq!(shabbat.day, 15);
    }

    #[test]
    fn test_parsha_names() {
        assert_eq!(Parsha::Bereshit.name(), "Bereshit");
        assert_eq!(Parsha::Bereshit.hebrew_name(), "בראשית");
        assert_eq!(Parsha::VayakhelPekudei.name(), "Vayakhel-Pekudei");
        assert_eq!(Parsha::NitzavimVayeilech.name(), "Nitzavim-Vayeilech");
    }

    #[test]
    fn test_build_schedule_common_year() {
        // Common year with ~37 pre-Pesach Shabbatot → need 6 combos
        let sched = ParshaCalculator::build_schedule(false, 37);
        // Should have 37 + 10 = 47 entries (pre_pesach + post_pesach)
        assert_eq!(sched.len(), 47);
        // First should be Bereshit
        assert_eq!(sched[0], Parsha::Bereshit);
        // Last should be HaAzinu
        assert_eq!(sched[sched.len() - 1], Parsha::HaAzinu);
    }

    #[test]
    fn test_build_schedule_leap_year() {
        // Leap year with ~41 pre-Pesach Shabbatot → need 2 combos
        let sched = ParshaCalculator::build_schedule(true, 41);
        // 41 pre + 10 post = 51
        assert_eq!(sched.len(), 51);
        assert_eq!(sched[0], Parsha::Bereshit);
        assert_eq!(sched[sched.len() - 1], Parsha::HaAzinu);
    }
}
