//! A Hebrew calendar: dates, holidays, Torah readings, zmanim, the molad
//! and Daf Yomi.
//!
//! The calendar is the fixed arithmetic one, covering Hebrew years 1 to 9999
//! (3761 BCE to 6239 CE). Holidays and Torah readings follow either the
//! diaspora or the Israeli custom; zmanim are shown in the location's own
//! time zone, daylight saving included.
//!
//! ```
//! use chrono::NaiveDate;
//! use hebrew_core::{GeoLocation, HebrewCalendar, Settings};
//!
//! let settings = Settings { location: Some(GeoLocation::new_york()), ..Settings::default() };
//! let day = HebrewCalendar::day(NaiveDate::from_ymd_opt(2024, 4, 22).unwrap(), &settings).unwrap();
//! assert_eq!(day.hebrew_display, "14 Nisan 5784");
//! assert!(day.holidays.iter().any(|h| h.name == "Erev Pesach"));
//! assert!(day.candle_lighting.is_some());
//! ```

pub mod calendar;
pub mod daf_yomi;
pub mod holidays;
pub mod molad;
pub mod parsha;
pub mod zmanim;

pub use calendar::{DateConverter, GregorianDate, HebrewDate, HebrewMonth};
pub use daf_yomi::{DafYomi, DafYomiCalculator, Tractate};
pub use holidays::{Holiday, HolidayCalculator, HolidayCategory};
pub use molad::{Molad, MoladCalculator};
pub use parsha::{Parsha, ParshaCalculator};
pub use zmanim::{GeoLocation, Zmanim, ZmanimCalculator};

use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors from the calendar.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum CalendarError {
    #[error("Date out of range: {0}")]
    DateOutOfRange(String),

    #[error("Invalid date: {0}")]
    InvalidDate(String),

    #[error("Invalid date format: {0}")]
    InvalidDateFormat(String),

    #[error("Invalid latitude: {0}. Must be between -90 and 90.")]
    InvalidLatitude(f64),

    #[error("Invalid longitude: {0}. Must be between -180 and 180.")]
    InvalidLongitude(f64),

    #[error("Unknown time zone: {0}. Use an IANA name such as America/New_York.")]
    InvalidTimezone(String),

    #[error("Calculation error: {0}")]
    CalculationError(String),
}

/// Whose customs to follow: festivals outside Israel keep a second day,
/// which also shifts some weeks' Torah readings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Observance {
    #[default]
    Diaspora,
    Israel,
}

/// How to calculate a day.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    /// Where to calculate zmanim and candle lighting for; none gives neither.
    pub location: Option<GeoLocation>,
    pub observance: Observance,
    /// Minutes before sunset to light candles: 18 in most places, 40 in
    /// Jerusalem.
    pub candle_lighting_minutes: i64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            location: None,
            observance: Observance::Diaspora,
            candle_lighting_minutes: 18,
        }
    }
}

/// A holiday with its names, for display.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HolidayInfo {
    pub holiday: Holiday,
    pub name: String,
    pub hebrew: String,
    pub category: HolidayCategory,
}

impl From<Holiday> for HolidayInfo {
    fn from(holiday: Holiday) -> Self {
        Self {
            name: holiday.name(),
            hebrew: holiday.hebrew_name(),
            category: holiday.category(),
            holiday,
        }
    }
}

/// A Torah reading with its names.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reading {
    pub parsha: Parsha,
    pub name: String,
    pub hebrew: String,
}

impl From<Parsha> for Reading {
    fn from(parsha: Parsha) -> Self {
        Self {
            parsha,
            name: parsha.name().to_string(),
            hebrew: parsha.hebrew_name().to_string(),
        }
    }
}

/// The day's page of Talmud with its names.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DafYomiInfo {
    pub daf: DafYomi,
    /// "Berachos 2"
    pub name: String,
    /// "ברכות ב׳"
    pub hebrew: String,
}

/// The molad announced on Shabbat Mevarchim, for the coming month.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mevarchim {
    /// The month being blessed, e.g. "Cheshvan".
    pub month: String,
    pub month_hebrew: String,
    pub molad: Molad,
    /// The announcement's wording (Jerusalem mean time).
    pub announcement: String,
}

/// Everything about one day.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DailyData {
    pub gregorian: GregorianDate,
    /// "Sunday" … "Saturday".
    pub weekday: String,
    pub hebrew: HebrewDate,
    /// "14 Nisan 5784"
    pub hebrew_display: String,
    /// "י״ד ניסן תשפ״ד"
    pub hebrew_display_he: String,
    pub holidays: Vec<HolidayInfo>,
    /// On Shabbat, the weekly portion; `None` when a festival reading replaces it.
    pub parsha: Option<Reading>,
    /// The portion of the coming Shabbat, on any day of the week.
    pub week_parsha: Option<Reading>,
    /// The Omer day this date completes, 1..=49.
    pub omer: Option<u8>,
    /// Chanukah candles lit this evening.
    pub chanukah_candles: Option<u8>,
    /// From 11 September 1923 on.
    pub daf_yomi: Option<DafYomiInfo>,
    /// On Shabbat Mevarchim, the coming month and its molad.
    pub mevarchim: Option<Mevarchim>,
    /// With a location.
    pub zmanim: Option<Zmanim>,
    /// Friday or the eve of a festival. On a second festival night, or a
    /// festival that begins as Shabbat ends, candles are lit after nightfall
    /// and this is that time.
    pub candle_lighting: Option<String>,
    /// True when `candle_lighting` is "after nightfall", not before sunset.
    pub candle_lighting_after_nightfall: bool,
    /// When Shabbat or a festival ends.
    pub havdalah: Option<String>,
    pub is_shabbat: bool,
    /// A festival day on which work is forbidden (not Shabbat itself).
    pub is_yom_tov: bool,
    pub is_fast: bool,
}

/// Main entry point.
pub struct HebrewCalendar;

impl HebrewCalendar {
    /// Everything about `date`.
    pub fn day(date: NaiveDate, settings: &Settings) -> Result<DailyData, CalendarError> {
        let hebrew = DateConverter::gregorian_to_hebrew(date)?;
        let observance = settings.observance;
        let holidays = HolidayCalculator::holidays_for(&hebrew, observance)?;
        let is_shabbat = date.weekday() == Weekday::Sat;
        let is_yom_tov = holidays.iter().any(Holiday::is_yom_tov);
        let is_fast = holidays.iter().any(Holiday::is_fast_day);

        let week_parsha = ParshaCalculator::get_parsha_for(&hebrew, observance)?;
        let parsha = if is_shabbat { week_parsha } else { None };

        let daf_yomi = if date >= daf_yomi::CYCLE_EPOCH {
            let daf = DafYomiCalculator::for_date(date)?;
            Some(DafYomiInfo {
                name: daf.to_string(),
                hebrew: format!(
                    "{} {}",
                    daf.tractate.hebrew_name(),
                    calendar::hebrew_numeral(daf.daf)
                ),
                daf,
            })
        } else {
            None
        };

        let mevarchim = if is_shabbat {
            Self::mevarchim(date)?
        } else {
            None
        };

        let mut zmanim = None;
        let mut candle_lighting = None;
        let mut after_nightfall = false;
        let mut havdalah = None;
        if let Some(location) = &settings.location {
            let calc = ZmanimCalculator::new(location.clone());
            zmanim = Some(calc.calculate(date)?);
            let tomorrow = date.succ_opt().ok_or_else(|| {
                CalendarError::DateOutOfRange(format!("{date} is the last supported day"))
            })?;
            let tomorrow_hebrew = DateConverter::gregorian_to_hebrew(tomorrow)?;
            let tomorrow_yom_tov = HolidayCalculator::holidays_for(&tomorrow_hebrew, observance)?
                .iter()
                .any(Holiday::is_yom_tov);
            let is_friday = date.weekday() == Weekday::Fri;
            if is_friday {
                candle_lighting = calc.candle_lighting(date, settings.candle_lighting_minutes)?;
            } else if tomorrow_yom_tov {
                if is_yom_tov || is_shabbat {
                    // No kindling on Shabbat or a festival itself: after nightfall.
                    candle_lighting = calc.nightfall(date)?;
                    after_nightfall = true;
                } else {
                    candle_lighting =
                        calc.candle_lighting(date, settings.candle_lighting_minutes)?;
                }
            }
            if (is_shabbat || is_yom_tov) && !(tomorrow_yom_tov || is_friday) {
                havdalah = calc.nightfall(date)?;
            }
        }

        Ok(DailyData {
            gregorian: GregorianDate::from(date),
            weekday: date.format("%A").to_string(),
            hebrew_display: hebrew.format(),
            hebrew_display_he: hebrew.format_hebrew(),
            holidays: holidays.into_iter().map(HolidayInfo::from).collect(),
            parsha: parsha.map(Reading::from),
            week_parsha: week_parsha.map(Reading::from),
            omer: HolidayCalculator::omer_day(&hebrew),
            chanukah_candles: HolidayCalculator::chanukah_candles_tonight(&hebrew)?,
            daf_yomi,
            mevarchim,
            zmanim,
            candle_lighting,
            candle_lighting_after_nightfall: after_nightfall,
            havdalah,
            is_shabbat,
            is_yom_tov,
            is_fast,
            hebrew,
        })
    }

    /// Every day from `start` to `end` inclusive (at most 400 days).
    pub fn range(
        start: NaiveDate,
        end: NaiveDate,
        settings: &Settings,
    ) -> Result<Vec<DailyData>, CalendarError> {
        let days = (end - start).num_days();
        if !(0..400).contains(&days) {
            return Err(CalendarError::DateOutOfRange(format!(
                "a range must run forwards and cover at most 400 days, not {}",
                days + 1
            )));
        }
        start
            .iter_days()
            .take(days as usize + 1)
            .map(|d| Self::day(d, settings))
            .collect()
    }

    /// If `shabbat` is Shabbat Mevarchim, the month it blesses and its molad.
    ///
    /// Shabbat Mevarchim is the last Shabbat before Rosh Chodesh begins, when
    /// Rosh Chodesh begins within the next seven days; Tishrei is not
    /// announced.
    pub fn mevarchim(shabbat: NaiveDate) -> Result<Option<Mevarchim>, CalendarError> {
        let hebrew = DateConverter::gregorian_to_hebrew(shabbat)?;
        if hebrew.month == HebrewMonth::Elul {
            return Ok(None);
        }
        let leap = DateConverter::is_hebrew_leap_year(hebrew.year);
        let length = DateConverter::days_in_hebrew_month(hebrew.year, hebrew.month.to_number(leap));
        // Rosh Chodesh begins on the 30th of a full month, else on the 1st.
        let rosh_chodesh = if length == 30 { 30 } else { length + 1 };
        let days_until = rosh_chodesh as i32 - hebrew.day as i32;
        if !(1..=7).contains(&days_until) {
            return Ok(None);
        }
        let first = shabbat + chrono::Duration::days((length - hebrew.day + 1) as i64);
        let next = DateConverter::gregorian_to_hebrew(first)?;
        let molad = MoladCalculator::of_month(next.year, next.month)?;
        let next_leap = DateConverter::is_hebrew_leap_year(next.year);
        Ok(Some(Mevarchim {
            month: next.month.name_for_year(next.year).to_string(),
            month_hebrew: next.month.hebrew_name(next_leap).to_string(),
            announcement: molad.announcement(),
            molad,
        }))
    }

    /// Hebrew date to Gregorian, validating the day and month.
    pub fn hebrew_to_gregorian(
        year: i32,
        month: HebrewMonth,
        day: u8,
    ) -> Result<NaiveDate, CalendarError> {
        DateConverter::hebrew_to_gregorian(HebrewDate::new(year, month, day))
    }

    /// Holidays from `start` to `end` inclusive, with their dates.
    pub fn holidays_between(
        start: NaiveDate,
        end: NaiveDate,
        observance: Observance,
    ) -> Result<Vec<(NaiveDate, HolidayInfo)>, CalendarError> {
        Ok(holidays::holidays_between(start, end, observance)?
            .into_iter()
            .map(|(d, h)| (d, HolidayInfo::from(h)))
            .collect())
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
            date_str
                .parse::<chrono::NaiveDate>()
                .map_err(|e| CalendarError::InvalidDateFormat(e.to_string()))?
        };

        Ok(date)
    }

    /// "15 March 2024"; years before 1 CE as "... 1 BCE".
    pub fn format_display_date(date: NaiveDate) -> String {
        calendar::format_gregorian(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    fn in_new_york() -> Settings {
        Settings {
            location: Some(GeoLocation::new_york()),
            ..Settings::default()
        }
    }

    #[test]
    fn the_calendar_is_continuous_across_1_bce_to_1_ce() {
        let before = DateConverter::gregorian_to_hebrew(date(0, 12, 31)).unwrap();
        let after = DateConverter::gregorian_to_hebrew(date(1, 1, 1)).unwrap();
        assert_eq!(
            DateConverter::hebrew_to_rd(after).unwrap()
                - DateConverter::hebrew_to_rd(before).unwrap(),
            1
        );
        assert_eq!(before.year, after.year);
        assert_eq!(after.day, before.day + 1);
    }

    #[test]
    fn the_range_is_hebrew_years_1_to_9999() {
        let first = DateConverter::rd_to_gregorian(DateConverter::rosh_hashanah(1)).unwrap();
        assert_eq!(DateConverter::gregorian_to_hebrew(first).unwrap().year, 1);
        assert!(matches!(
            DateConverter::gregorian_to_hebrew(first.pred_opt().unwrap()),
            Err(CalendarError::DateOutOfRange(_))
        ));
        // Well past the old 2050 limit.
        assert!(HebrewCalendar::day(date(2150, 1, 1), &Settings::default()).is_ok());
        assert!(HebrewCalendar::day(date(6300, 1, 1), &Settings::default()).is_err());
    }

    #[test]
    fn date_parsing() {
        assert_eq!(HebrewCalendar::parse_date("+0000-01-01").unwrap().year(), 0);
        assert_eq!(
            HebrewCalendar::parse_date("2024-12-25").unwrap(),
            date(2024, 12, 25)
        );
        assert!(HebrewCalendar::parse_date("not-a-date").is_err());
    }

    #[test]
    fn display_dates() {
        assert_eq!(
            HebrewCalendar::format_display_date(date(2024, 3, 15)),
            "15 March 2024"
        );
        assert_eq!(
            HebrewCalendar::format_display_date(date(0, 6, 1)),
            "1 June 1 BCE"
        );
        let d = HebrewCalendar::day(date(2025, 12, 15), &Settings::default()).unwrap();
        assert_eq!(d.hebrew_display, "25 Kislev 5786");
        assert_eq!(d.hebrew_display_he, "כ״ה כסלו תשפ״ו");
        assert_eq!(d.weekday, "Monday");
    }

    #[test]
    fn hebrew_numerals() {
        use calendar::hebrew_numeral as n;
        assert_eq!(n(1), "א׳");
        assert_eq!(n(15), "ט״ו");
        assert_eq!(n(16), "ט״ז");
        assert_eq!(n(30), "ל׳");
        assert_eq!(n(786), "תשפ״ו");
        assert_eq!(n(5784), "תשפ״ד");
    }

    #[test]
    fn erev_pesach_in_new_york() {
        let d = HebrewCalendar::day(date(2024, 4, 22), &in_new_york()).unwrap();
        assert!(d.holidays.iter().any(|h| h.holiday == Holiday::ErevPesach));
        assert!(!d.candle_lighting_after_nightfall);
        let second_night = HebrewCalendar::day(date(2024, 4, 23), &in_new_york()).unwrap();
        assert!(second_night.is_yom_tov);
        assert!(
            second_night.candle_lighting_after_nightfall,
            "second night: after nightfall"
        );
        let second_day = HebrewCalendar::day(date(2024, 4, 24), &in_new_york()).unwrap();
        assert!(
            second_day.havdalah.is_some(),
            "Pesach II ends with havdalah"
        );
    }

    #[test]
    fn shabbat_has_a_reading_and_havdalah() {
        let d = HebrewCalendar::day(date(2023, 10, 14), &in_new_york()).unwrap();
        assert!(d.is_shabbat);
        assert_eq!(d.parsha.as_ref().unwrap().parsha, Parsha::Bereshit);
        assert!(d.havdalah.is_some());
        let friday = HebrewCalendar::day(date(2023, 10, 13), &in_new_york()).unwrap();
        assert!(friday.candle_lighting.is_some());
        assert!(friday.parsha.is_none());
        assert_eq!(friday.week_parsha.unwrap().parsha, Parsha::Bereshit);
    }

    #[test]
    fn no_location_means_no_times() {
        let d = HebrewCalendar::day(date(2023, 10, 13), &Settings::default()).unwrap();
        assert!(d.zmanim.is_none() && d.candle_lighting.is_none());
    }

    #[test]
    fn shabbat_mevarchim_announces_the_molad() {
        // Shabbat 18 October 2025 blessed Cheshvan 5786.
        let d = HebrewCalendar::day(date(2025, 10, 18), &Settings::default()).unwrap();
        let m = d.mevarchim.expect("Shabbat Mevarchim");
        assert_eq!(m.month, "Cheshvan");
        assert_eq!(m.molad.weekday, chrono::Weekday::Wed); // 12:54 AM + 8 chalakim
                                                           // The Shabbat after Rosh Chodesh is not.
        let d = HebrewCalendar::day(date(2025, 10, 25), &Settings::default()).unwrap();
        assert!(d.mevarchim.is_none());
    }

    #[test]
    fn daf_yomi_and_omer_on_the_day() {
        let d = HebrewCalendar::day(date(2024, 5, 26), &Settings::default()).unwrap();
        assert_eq!(d.omer, Some(33));
        assert!(d.daf_yomi.is_some());
        let old = HebrewCalendar::day(date(1900, 1, 1), &Settings::default()).unwrap();
        assert!(old.daf_yomi.is_none());
    }

    #[test]
    fn ranges_are_bounded() {
        let s = Settings::default();
        assert_eq!(
            HebrewCalendar::range(date(2024, 1, 1), date(2024, 1, 31), &s)
                .unwrap()
                .len(),
            31
        );
        assert!(HebrewCalendar::range(date(2024, 1, 2), date(2024, 1, 1), &s).is_err());
        assert!(HebrewCalendar::range(date(2024, 1, 1), date(2026, 1, 1), &s).is_err());
    }

    #[test]
    fn hebrew_dates_are_validated() {
        assert_eq!(
            HebrewCalendar::hebrew_to_gregorian(5786, HebrewMonth::Tishrei, 1).unwrap(),
            date(2025, 9, 23)
        );
        assert!(HebrewCalendar::hebrew_to_gregorian(5786, HebrewMonth::AdarI, 1).is_err());
        assert!(HebrewCalendar::hebrew_to_gregorian(5786, HebrewMonth::Iyar, 30).is_err());
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
        assert_eq!(
            via_root.gregorian,
            NaiveDate::from_ymd_opt(2025, 9, 22).unwrap()
        );
        assert_eq!(
            (via_root.hour, via_root.minute, via_root.chalakim),
            (12, 10, 7)
        );
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
            crate::DafYomiCalculator::for_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
                .unwrap();
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
        assert!(
            matches!(err, CalendarError::CalculationError(_)),
            "{:?}",
            err
        );
        assert!(matches!(
            HebrewCalendar::molad(0, calendar::HebrewMonth::Tishrei).unwrap_err(),
            CalendarError::InvalidDateFormat(_)
        ));
    }

    #[test]
    fn public_api_daf_yomi_helper_spans_the_cycle_boundary() {
        // 1923 epoch, the 2702 -> 2711 seam of June 1975, and a modern date.
        let dates = [
            (
                NaiveDate::from_ymd_opt(1923, 9, 11).unwrap(),
                Tractate::Berachos,
                2,
                1u32,
            ),
            (
                NaiveDate::from_ymd_opt(1975, 6, 23).unwrap(),
                Tractate::Nidah,
                73,
                7,
            ),
            (
                NaiveDate::from_ymd_opt(1975, 6, 24).unwrap(),
                Tractate::Berachos,
                2,
                8,
            ),
            (
                NaiveDate::from_ymd_opt(2005, 3, 2).unwrap(),
                Tractate::Berachos,
                2,
                12,
            ),
            (
                NaiveDate::from_ymd_opt(2027, 6, 7).unwrap(),
                Tractate::Nidah,
                73,
                14,
            ),
        ];
        for (date, tractate, daf, cycle) in dates {
            let via_helper = HebrewCalendar::daf_yomi(date).unwrap();
            assert_eq!(
                (via_helper.tractate, via_helper.daf, via_helper.cycle_number),
                (tractate, daf, cycle),
                "{} via the front door",
                date
            );
            assert_eq!(
                via_helper,
                crate::DafYomiCalculator::for_date(date).unwrap()
            );
        }

        // Before the cycle existed, the front door reports the module's error.
        let err =
            HebrewCalendar::daf_yomi(NaiveDate::from_ymd_opt(1923, 9, 10).unwrap()).unwrap_err();
        assert!(matches!(err, CalendarError::DateOutOfRange(_)), "{:?}", err);
    }

    #[test]
    fn published_daf_and_molad_values() {
        // Both values are published rows: Bava Basra 176 on 18 December 2024 is a
        // row of the 14th-cycle schedule, and the molad of Tishrei 5785 is
        // Thursday 3 October 2024, 3:21 AM + 13 chalakim.
        let daf_date = NaiveDate::from_ymd_opt(2024, 12, 18).unwrap();
        let daf = HebrewCalendar::daf_yomi(daf_date).unwrap();
        assert_eq!(
            (daf.tractate, daf.daf, daf.cycle_number),
            (crate::daf_yomi::Tractate::BavaBasra, 176, 14)
        );
        let molad = HebrewCalendar::molad(5785, calendar::HebrewMonth::Tishrei).unwrap();
        assert_eq!(
            molad.gregorian,
            NaiveDate::from_ymd_opt(2024, 10, 3).unwrap()
        );
        assert_eq!((molad.hour, molad.minute, molad.chalakim), (3, 21, 13));
    }
}
