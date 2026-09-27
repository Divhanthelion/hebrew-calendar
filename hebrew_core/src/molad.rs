//! Molad (mean lunar conjunction) as announced on Shabbat Mevarchim.
//!
//! For every Hebrew month this module reports the instant traditionally
//! announced in the synagogue on the Shabbat before Rosh Chodesh: the day of
//! the week, the Gregorian date, and the time given as hours, minutes and
//! `chalakim` (parts), all in Jerusalem time.
//!
//! Everything here is built on the *single* molad-of-Tishrei parts
//! arithmetic that the calendar itself uses — [`DateConverter::molad_parts`],
//! hoisted out of `hebrew_calendar_elapsed_days` — so an announced molad and
//! the date of Rosh Hashanah can never drift apart. The molad of a month is
//! that Hebrew year's molad of Tishrei plus `k × 765_433` parts, where `k` is
//! the number of months elapsed from Tishrei and 765_433 = 29 d + 12 h + 793 p
//! is the mean synodic month in parts.
//!
//! # Conventions
//!
//! * The announced molad is the **raw mean conjunction**. The dehiyyot (Molad
//!   Zaken, Gatarad, Betutakfot, Lo ADU Rosh) turn a molad into the *date of
//!   Rosh Hashanah*; they never move an announced molad, so none is applied
//!   here. Tishrei 5786 is the discriminating case: the molad is announced for
//!   Monday 22 Sep 2025 while Rosh Hashanah 5786 was postponed to Tuesday
//!   23 Sep 2025.
//! * Times are Jerusalem time with no daylight-saving adjustment, exactly as
//!   printed in synagogue tables.
//! * The parts count of [`DateConverter::molad_parts`] is measured from the
//!   Hebrew epoch, whose *moment* is 6 pm at the end of the civil day before
//!   the epoch date. Equivalently, the remainder of a parts count divided by
//!   `PARTS_PER_DAY` is measured from **noon** of a civil day: 187 parts
//!   means 10 minutes and 7 chalakim after noon. [`MoladCalculator::from_parts`]
//!   is the one place that turns parts into a civil date, and it shifts the
//!   date forward by one whenever the remainder reaches noon + 12 h.
//! * `hour` is a civil clock hour, `0..=23` (midnight-first), the way the
//!   printed tables list it. The halakhic arithmetic that produces the same
//!   instant numbers hours from 6 pm of the previous civil day; that
//!   evening-first numbering is available via [`Molad::evening_hour`].

use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

use crate::calendar::{DateConverter, GregorianDate, HebrewMonth};
use crate::CalendarError;

/// Parts (chalakim) in a full day: 24 hours × 1080 parts.
pub const PARTS_PER_DAY: i64 = 25920;

/// Parts in one hour.
pub const PARTS_PER_HOUR: i64 = 1080;

/// Parts in one minute.
pub const PARTS_PER_MINUTE: i64 = 18;

/// Parts in half a day — the point from which the remainder of a parts count
/// is measured (noon), and the Molad Zaken threshold.
pub const PARTS_PER_HALF_DAY: i64 = 12 * PARTS_PER_HOUR;

/// The mean synodic month in parts: 29 d + 12 h + 793 p = 765 433.
///
/// Note that `DateConverter::PARTS_PER_LUNATION` holds only the fraction of
/// the mean month beyond its 29 whole days (13 753); this is the whole month.
pub const PARTS_PER_LUNATION: i64 = 29 * PARTS_PER_DAY + 12 * PARTS_PER_HOUR + 793;

/// The molad of a Hebrew month, as announced on Shabbat Mevarchim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Molad {
    /// Hebrew year this molad belongs to.
    pub hebrew_year: i32,
    /// Hebrew month this molad belongs to (`Adar` means `Adar II` in a leap year).
    pub hebrew_month: HebrewMonth,
    /// Weekday of the civil date on which the molad falls.
    pub weekday: Weekday,
    /// Civil date of the molad (Jerusalem).
    pub gregorian: NaiveDate,
    /// Civil clock hour, 0 = midnight ..= 23.
    pub hour: u8,
    /// Minute within the hour, 0..=59.
    pub minute: u8,
    /// Residual parts beyond the minute, 0..=17.
    pub chalakim: u8,
}

impl Molad {
    /// The molad as parts elapsed from the Hebrew epoch — the inverse of
    /// [`MoladCalculator::from_parts`].
    pub fn parts_since_epoch(&self) -> i64 {
        let days = DateConverter::gregorian_to_rd(self.gregorian) as i64
            - DateConverter::hebrew_epoch_rd() as i64;
        days * PARTS_PER_DAY
            + self.hour as i64 * PARTS_PER_HOUR
            + self.minute as i64 * PARTS_PER_MINUTE
            + self.chalakim as i64
            + PARTS_PER_DAY
            - PARTS_PER_HALF_DAY
    }

    /// Hours counted from 6 pm of the previous civil day, the traditional
    /// halakhic numbering in which a molad at hour 18 or later is "old"
    /// (Molad Zaken). A result below 6 belongs to the *next* weekday in that
    /// numbering, because it is the evening that opens it.
    pub fn evening_hour(&self) -> u8 {
        ((self.hour as i64 + 6) % 24) as u8
    }

    /// Whether the molad falls at or after noon, which is Molad Zaken and can
    /// postpone Rosh Hashanah. Announced molads are never adjusted for it.
    pub fn is_zaken(&self) -> bool {
        self.hour >= 12
    }

    /// Civil 12-hour clock hour for display, 1..=12.
    fn clock_hour(&self) -> u8 {
        let h = self.hour % 12;
        if h == 0 {
            12
        } else {
            h
        }
    }

    /// AM or PM of the civil clock.
    fn meridiem(&self) -> &'static str {
        if self.hour < 12 {
            "AM"
        } else {
            "PM"
        }
    }

    /// Part of the civil day, for the synagogue wording.
    fn day_part(&self) -> &'static str {
        match self.hour {
            0..=5 => "night",
            6..=11 => "morning",
            12..=17 => "afternoon",
            _ => "evening",
        }
    }

    /// The elapsed time past the hour, in the wording used by published
    /// announcements: singular for one minute or one chelek, and a zero
    /// component dropped.
    fn elapsed_phrase(&self) -> String {
        let minute = match self.minute {
            0 => None,
            1 => Some("1 minute".to_string()),
            m => Some(format!("{} minutes", m)),
        };
        let chelek = match self.chalakim {
            0 => None,
            1 => Some("1 chelek".to_string()),
            c => Some(format!("{} chalakim", c)),
        };
        match (minute, chelek) {
            (None, None) => String::from("exactly"),
            (Some(m), None) => m,
            (None, Some(c)) => c,
            (Some(m), Some(c)) => format!("{} and {}", m, c),
        }
    }

    /// The molad in the form it is announced on Shabbat Mevarchim, e.g.
    /// "Monday afternoon, 10 minutes and 7 chalakim after 12:00 PM".
    pub fn announcement(&self) -> String {
        format!(
            "{} {}, {} after {}:{:02} {}",
            weekday_name(self.weekday),
            self.day_part(),
            self.elapsed_phrase(),
            self.clock_hour(),
            0,
            self.meridiem()
        )
    }

    /// The civil date in the crate's serializable form.
    pub fn gregorian_date(&self) -> GregorianDate {
        GregorianDate::from(self.gregorian)
    }
}

impl std::fmt::Display for Molad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Molad {} {}: {}, {} {}, {}, {}:{:02} {} + {} chalakim",
            self.hebrew_month.name_for_year(self.hebrew_year),
            self.hebrew_year,
            weekday_name(self.weekday),
            month_name(self.gregorian.month()),
            self.gregorian.day(),
            self.gregorian.year(),
            self.clock_hour(),
            self.minute,
            self.meridiem(),
            self.chalakim
        )
    }
}

/// Gregorian month name, for the printed-table form of a molad.
fn month_name(m: u32) -> &'static str {
    match m {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        _ => "December",
    }
}

/// Weekday name used in published molad tables ("Shabbat" for Saturday).
fn weekday_name(w: Weekday) -> &'static str {
    match w {
        Weekday::Sun => "Sunday",
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Shabbat",
    }
}

/// Calculator for the announced molad of any Hebrew month.
pub struct MoladCalculator;

impl MoladCalculator {
    /// Number of months from Tishrei of `year` to `month`, in the order the
    /// months of that year actually occur: Tishrei, Cheshvan, Kislev, Teves,
    /// Shevat, (Adar I,) Adar / Adar II, Nisan, …, Elul.
    fn months_from_tishrei(year: i32, month: HebrewMonth) -> i64 {
        let is_leap = DateConverter::is_hebrew_leap_year(year);
        let n = month.to_number(is_leap) as i64;
        if n >= 7 {
            n - 7
        } else {
            DateConverter::months_in_hebrew_year(year) as i64 - 7 + n
        }
    }

    /// The molad of Tishrei of Hebrew `year`, before any dehiyyah.
    pub fn of_tishrei(year: i32) -> Result<Molad, CalendarError> {
        Self::of_month(year, HebrewMonth::Tishrei)
    }

    /// The announced molad of `month` of Hebrew `year`.
    ///
    /// `HebrewMonth::Adar` means plain Adar in a common year and Adar II in a
    /// leap year; `HebrewMonth::AdarI` exists only in leap years.
    pub fn of_month(year: i32, month: HebrewMonth) -> Result<Molad, CalendarError> {
        if year < 1 {
            return Err(CalendarError::InvalidDateFormat(format!(
                "Hebrew year must be >= 1, got {}",
                year
            )));
        }
        if month == HebrewMonth::AdarI && !DateConverter::is_hebrew_leap_year(year) {
            return Err(CalendarError::CalculationError(format!(
                "Adar I does not exist in common year {}",
                year
            )));
        }

        let k = Self::months_from_tishrei(year, month);
        Self::from_parts(
            year,
            month,
            DateConverter::molad_parts(year) + k * PARTS_PER_LUNATION,
        )
    }

    /// Every molad announced during Hebrew `year`, Tishrei first.
    pub fn of_year(year: i32) -> Result<Vec<Molad>, CalendarError> {
        let is_leap = DateConverter::is_hebrew_leap_year(year);
        let mut months = vec![
            HebrewMonth::Tishrei,
            HebrewMonth::Cheshvan,
            HebrewMonth::Kislev,
            HebrewMonth::Teves,
            HebrewMonth::Shevat,
        ];
        if is_leap {
            months.push(HebrewMonth::AdarI);
        }
        months.push(HebrewMonth::Adar);
        months.extend_from_slice(&[
            HebrewMonth::Nisan,
            HebrewMonth::Iyar,
            HebrewMonth::Sivan,
            HebrewMonth::Tammuz,
            HebrewMonth::Av,
            HebrewMonth::Elul,
        ]);

        months.sort_by_key(|m| Self::months_from_tishrei(year, *m));
        months
            .into_iter()
            .map(|m| Self::of_month(year, m))
            .collect()
    }

    /// Build a `Molad` from a parts count measured from the Hebrew epoch.
    ///
    /// The whole-day count names the civil day the count ends *in*, so the
    /// civil date is `epoch + days - 1`, advanced by one day when the parts
    /// remainder reaches noon + 12 h (i.e. when the instant falls in the
    /// second half of the noon-based remainder, past midnight).
    fn from_parts(year: i32, month: HebrewMonth, parts: i64) -> Result<Molad, CalendarError> {
        let whole_days = parts.div_euclid(PARTS_PER_DAY);
        let remainder = parts.rem_euclid(PARTS_PER_DAY);

        let noon_hours = remainder / PARTS_PER_HOUR;
        let minute = ((remainder % PARTS_PER_HOUR) / PARTS_PER_MINUTE) as u8;
        let chalakim = (remainder % PARTS_PER_MINUTE) as u8;

        let civil_hour = ((noon_hours + 12) % 24) as u8;
        let day_shift = if remainder >= PARTS_PER_HALF_DAY {
            1
        } else {
            0
        };

        let rd = DateConverter::hebrew_epoch_rd() as i64 + whole_days - 1 + day_shift;
        let gregorian = DateConverter::rd_to_gregorian(rd as i32)?;

        Ok(Molad {
            hebrew_year: year,
            hebrew_month: month,
            weekday: gregorian.weekday(),
            gregorian,
            hour: civil_hour,
            minute,
            chalakim,
        })
    }
}

#[cfg(test)]
mod tests;
