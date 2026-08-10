//! Parsha (Torah Portion) Calculation Module
//!
//! Computes the weekly Torah portion (parsha) read on Shabbat using the
//! diaspora keviyah schedule tables from hebcal (sedra.ts).
//!
//! Key = `{leap}{rhDay}{yearType}` where rhDay is 1=Sun..7=Sat and
//! yearType is 0=incomplete, 1=regular, 2=complete (Cheshvan/Kislev lengths).

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

/// Parsha calculator (diaspora schedule).
///
/// Port of hebcal's sedra keviyah tables:
/// https://github.com/hebcal/hebcal-es6/blob/main/src/sedra.ts
pub struct ParshaCalculator;

/// Schedule entry: non-negative = single parsha index (0=Bereshit),
/// negative = doubled pair starting at -id, CHAG = holiday reading.
#[derive(Clone, Copy)]
enum SedraEntry {
    Single(i8),
    Double(i8), // stores the positive first index; means pair (i, i+1)
    Chag,
}

use SedraEntry::*;

impl ParshaCalculator {
    /// 0-based single-parsha map (0..=52); 53 = Vezot Haberacha unused on Shabbat.
    const SINGLES: &'static [Parsha] = &[
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

    fn double_parsha(first: i8) -> Parsha {
        match first {
            21 => Parsha::VayakhelPekudei,
            26 => Parsha::TazriaMetzora,
            28 => Parsha::AchreiMotKedoshim,
            31 => Parsha::BeharBechukotai,
            38 => Parsha::ChukatBalak,
            41 => Parsha::MatotMasei,
            50 => Parsha::NitzavimVayeilech,
            _ => Parsha::HaftarahOnly,
        }
    }

    fn entry_to_parsha(e: SedraEntry) -> Parsha {
        match e {
            Single(i) => Self::SINGLES[i as usize],
            Double(i) => Self::double_parsha(i),
            Chag => Parsha::HaftarahOnly,
        }
    }

    /// Get the parsha for a Shabbat (or the Shabbat containing this date).
    pub fn get_parsha(date: &HebrewDate) -> Result<Parsha, CalendarError> {
        let shabbat_date = Self::find_shabbat(date)?;
        Self::calculate_parsha_for_shabbat(shabbat_date)
    }

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

    fn calculate_parsha_for_shabbat(date: HebrewDate) -> Result<Parsha, CalendarError> {
        let year = date.year;
        let (first_sat_rd, schedule) = Self::schedule_for_year(year)?;
        let shabbat_rd = DateConverter::hebrew_to_rd(date)?;
        if shabbat_rd < first_sat_rd {
            // Before this year's first Saturday — use previous year
            let (prev_first, prev_sched) = Self::schedule_for_year(year - 1)?;
            let week = ((shabbat_rd - prev_first) / 7) as usize;
            if week < prev_sched.len() {
                return Ok(Self::entry_to_parsha(prev_sched[week]));
            }
            return Ok(Parsha::HaftarahOnly);
        }
        let week = ((shabbat_rd - first_sat_rd) / 7) as usize;
        if week < schedule.len() {
            return Ok(Self::entry_to_parsha(schedule[week]));
        }
        // Past end of this year's schedule — next year
        let (next_first, next_sched) = Self::schedule_for_year(year + 1)?;
        let week = ((shabbat_rd - next_first) / 7) as usize;
        if week < next_sched.len() {
            Ok(Self::entry_to_parsha(next_sched[week]))
        } else {
            Ok(Parsha::HaftarahOnly)
        }
    }

    /// Returns (RD of first Saturday on/after RH, schedule array).
    fn schedule_for_year(year: i32) -> Result<(i32, Vec<SedraEntry>), CalendarError> {
        let rh_rd = DateConverter::rosh_hashanah(year);
        // First Saturday on or after RH:
        // RD % 7: 0=Sat, 1=Sun, ..., 6=Fri. We want RD with rem 0.
        let rem = rh_rd.rem_euclid(7);
        let first_sat = if rem == 0 { rh_rd } else { rh_rd + (7 - rem) };

        let leap = DateConverter::is_hebrew_leap_year(year);
        // RH day: 1=Sun ... 7=Sat (hebcal convention)
        let rh_date = HebrewDate::new(year, HebrewMonth::Tishrei, 1);
        let rh_day = rh_date.day_of_week() + 1; // 0=Sun → 1
        let ytype = match DateConverter::hebrew_year_type(year) {
            crate::calendar::YearType::DeficientCommon | crate::calendar::YearType::DeficientLeap => 0,
            crate::calendar::YearType::RegularCommon | crate::calendar::YearType::RegularLeap => 1,
            crate::calendar::YearType::CompleteCommon | crate::calendar::YearType::CompleteLeap => 2,
        };
        let key = format!("{}{}{}", leap as u8, rh_day, ytype);
        let schedule = Self::lookup_diaspora_schedule(&key)
            .ok_or_else(|| CalendarError::CalculationError(
                format!("Unknown sedra year type key {} for year {}", key, year)
            ))?;
        Ok((first_sat, schedule))
    }

    fn lookup_diaspora_schedule(key: &str) -> Option<Vec<SedraEntry>> {
        // hebcal first tries `{leap}{rhDay}{type}`, then appends IL flag
        // (0=diaspora, 1=Israel) when the short key is absent.
        let candidates = [
            key.to_string(),
            format!("{}0", key), // diaspora
        ];
        // Also resolve known aliases used in sedra.ts
        for cand in &candidates {
            let resolved = match cand.as_str() {
                "0221" => "020",
                "0310" => "0220",
                "0311" => "020",
                "1310" => "1220",
                "1311" => "1221",
                "1721" => "170",
                other => other,
            };
            if let Some(sched) = Self::build_type(resolved) {
                return Some(sched);
            }
        }
        None
    }

    fn build_type(key: &str) -> Option<Vec<SedraEntry>> {
        let entries: &[SedraEntry] = match key {
            "020" => &[Single(51), Single(52), Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Double(21), Single(23), Single(24), Chag, Single(25), Double(26), Double(28), Single(30), Double(31), Single(33), Single(34), Single(35), Single(36), Single(37), Single(38), Single(39), Single(40), Double(41), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Double(50)],
            "0220" => &[Single(51), Single(52), Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Double(21), Single(23), Single(24), Chag, Single(25), Double(26), Double(28), Single(30), Double(31), Single(33), Chag, Single(34), Single(35), Single(36), Single(37), Double(38), Single(40), Double(41), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Double(50)],
            "0510" => &[Single(52), Chag, Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Double(21), Single(23), Single(24), Chag, Chag, Single(25), Double(26), Double(28), Single(30), Double(31), Single(33), Single(34), Single(35), Single(36), Single(37), Single(38), Single(39), Single(40), Double(41), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Single(50)],
            "0511" => &[Single(52), Chag, Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Double(21), Single(23), Single(24), Chag, Single(25), Double(26), Double(28), Single(30), Single(31), Single(32), Single(33), Single(34), Single(35), Single(36), Single(37), Single(38), Single(39), Single(40), Double(41), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Single(50)],
            "052" => &[Single(52), Chag, Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Single(21), Single(22), Single(23), Single(24), Chag, Single(25), Double(26), Double(28), Single(30), Double(31), Single(33), Single(34), Single(35), Single(36), Single(37), Single(38), Single(39), Single(40), Double(41), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Single(50)],
            "070" => &[Chag, Single(52), Chag, Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Double(21), Single(23), Single(24), Chag, Single(25), Double(26), Double(28), Single(30), Double(31), Single(33), Single(34), Single(35), Single(36), Single(37), Single(38), Single(39), Single(40), Double(41), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Single(50)],
            "072" => &[Chag, Single(52), Chag, Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Double(21), Single(23), Single(24), Chag, Single(25), Double(26), Double(28), Single(30), Double(31), Single(33), Single(34), Single(35), Single(36), Single(37), Single(38), Single(39), Single(40), Double(41), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Double(50)],
            "1200" => &[Single(51), Single(52), Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Single(21), Single(22), Single(23), Single(24), Single(25), Single(26), Single(27), Chag, Single(28), Single(29), Single(30), Single(31), Single(32), Single(33), Chag, Single(34), Single(35), Single(36), Single(37), Double(38), Single(40), Double(41), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Double(50)],
            "1201" => &[Single(51), Single(52), Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Single(21), Single(22), Single(23), Single(24), Single(25), Single(26), Single(27), Chag, Single(28), Single(29), Single(30), Single(31), Single(32), Single(33), Single(34), Single(35), Single(36), Single(37), Single(38), Single(39), Single(40), Double(41), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Double(50)],
            "1220" => &[Single(51), Single(52), Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Single(21), Single(22), Single(23), Single(24), Single(25), Single(26), Single(27), Chag, Chag, Single(28), Single(29), Single(30), Single(31), Single(32), Single(33), Single(34), Single(35), Single(36), Single(37), Single(38), Single(39), Single(40), Double(41), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Single(50)],
            "1221" => &[Single(51), Single(52), Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Single(21), Single(22), Single(23), Single(24), Single(25), Single(26), Single(27), Chag, Single(28), Single(29), Single(30), Single(31), Single(32), Single(33), Single(34), Single(35), Single(36), Single(37), Single(38), Single(39), Single(40), Single(41), Single(42), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Single(50)],
            "150" => &[Single(52), Chag, Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Single(21), Single(22), Single(23), Single(24), Single(25), Single(26), Single(27), Single(28), Chag, Single(29), Single(30), Single(31), Single(32), Single(33), Single(34), Single(35), Single(36), Single(37), Single(38), Single(39), Single(40), Single(41), Single(42), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Single(50)],
            "152" => &[Single(52), Chag, Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Single(21), Single(22), Single(23), Single(24), Single(25), Single(26), Single(27), Single(28), Chag, Single(29), Single(30), Single(31), Single(32), Single(33), Single(34), Single(35), Single(36), Single(37), Single(38), Single(39), Single(40), Single(41), Single(42), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Double(50)],
            "170" => &[Chag, Single(52), Chag, Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Single(21), Single(22), Single(23), Single(24), Single(25), Single(26), Single(27), Chag, Single(28), Single(29), Single(30), Single(31), Single(32), Single(33), Single(34), Single(35), Single(36), Single(37), Single(38), Single(39), Single(40), Double(41), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Double(50)],
            "1720" => &[Chag, Single(52), Chag, Chag, Single(0), Single(1), Single(2), Single(3), Single(4), Single(5), Single(6), Single(7), Single(8), Single(9), Single(10), Single(11), Single(12), Single(13), Single(14), Single(15), Single(16), Single(17), Single(18), Single(19), Single(20), Single(21), Single(22), Single(23), Single(24), Single(25), Single(26), Single(27), Chag, Single(28), Single(29), Single(30), Single(31), Single(32), Single(33), Chag, Single(34), Single(35), Single(36), Single(37), Double(38), Single(40), Double(41), Single(43), Single(44), Single(45), Single(46), Single(47), Single(48), Single(49), Double(50)],
            _ => return None,
        };
        Some(entries.to_vec())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::{DateConverter, HebrewMonth};
    use chrono::NaiveDate;

    #[test]
    fn test_get_parsha_no_panic_for_5784() {
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
        // Oct 14, 2023 = Tishrei 29, 5784
        let date = HebrewDate::new(5784, HebrewMonth::Tishrei, 29);
        assert_eq!(ParshaCalculator::get_parsha(&date).unwrap(), Parsha::Bereshit);
    }

    #[test]
    fn test_shabbat_noach_5784() {
        let date = HebrewDate::new(5784, HebrewMonth::Cheshvan, 6);
        assert_eq!(ParshaCalculator::get_parsha(&date).unwrap(), Parsha::Noach);
    }

    #[test]
    fn test_find_shabbat() {
        let shabbat_date = HebrewDate::new(5784, HebrewMonth::Tishrei, 15);
        let shabbat = ParshaCalculator::find_shabbat(&shabbat_date).unwrap();
        assert_eq!(shabbat.day, 15);

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
    fn test_5784_against_hebcal() {
        let cases = [
            (2023, 10, 14, Parsha::Bereshit),
            (2023, 10, 21, Parsha::Noach),
            (2024, 3, 9, Parsha::Vayakhel),
            (2024, 3, 16, Parsha::Pekudei),
            (2024, 3, 23, Parsha::Vayikra),
            (2024, 4, 13, Parsha::Tazria),
            (2024, 4, 20, Parsha::Metzora),
            (2024, 4, 27, Parsha::HaftarahOnly), // Pesach Chol HaMoed
            (2024, 5, 4, Parsha::AchreiMot),
            (2024, 5, 11, Parsha::Kedoshim),
            (2024, 8, 3, Parsha::MatotMasei),
            (2024, 8, 10, Parsha::Devarim),
            (2024, 8, 17, Parsha::Vaetchanan),
            (2024, 8, 31, Parsha::Reeh),
            (2024, 9, 28, Parsha::NitzavimVayeilech),
            (2024, 10, 5, Parsha::HaAzinu),
        ];
        for (y, m, d, expected) in cases {
            let g = NaiveDate::from_ymd_opt(y, m, d).unwrap();
            let h = DateConverter::gregorian_to_hebrew(g).unwrap();
            let got = ParshaCalculator::get_parsha(&h).unwrap();
            assert_eq!(got, expected, "{}-{:02}-{:02} ({}): got {:?}, expected {:?}",
                y, m, d, h.format(), got, expected);
        }
    }

    #[test]
    fn test_5784_year_key() {
        // leap, RH Saturday (7), deficient (0) → "170"
        let (first_sat, sched) = ParshaCalculator::schedule_for_year(5784).unwrap();
        let rh = DateConverter::rosh_hashanah(5784);
        assert_eq!(rh.rem_euclid(7), 0, "RH 5784 should be Saturday");
        assert_eq!(first_sat, rh);
        assert!(!sched.is_empty());
    }
}

#[cfg(test)]
mod extra_checks {
    use super::*;
    use crate::calendar::{DateConverter, HebrewMonth};
    use chrono::NaiveDate;

    #[test]
    fn test_5783_against_hebcal_sample() {
        let cases = [
            (2022, 10, 22, Parsha::Bereshit),
            (2023, 3, 25, Parsha::Vayikra),
            (2023, 4, 15, Parsha::Shemini),
            (2023, 4, 22, Parsha::TazriaMetzora),
            (2023, 7, 8, Parsha::Pinchas),
            (2023, 9, 9, Parsha::NitzavimVayeilech),
        ];
        for (y, m, d, expected) in cases {
            let g = NaiveDate::from_ymd_opt(y, m, d).unwrap();
            let h = DateConverter::gregorian_to_hebrew(g).unwrap();
            let got = ParshaCalculator::get_parsha(&h).unwrap();
            assert_eq!(got, expected, "{}-{:02}-{:02} ({}): got {:?}", y, m, d, h.format(), got);
        }
    }

    #[test]
    fn test_adar_ii_format() {
        let d = HebrewDate::new(5784, HebrewMonth::Adar, 14);
        assert!(d.format().contains("Adar II"), "got {}", d.format());
    }
}
