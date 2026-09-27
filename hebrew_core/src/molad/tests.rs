//! Published-value tests for the announced molad.
//!
//! Oracles:
//! * Chabad.org, "Molad Times — For the Jewish Years 5786-5787" — the synagogue
//!   table for 5786 and the 13-month table for 5787.
//! * TorahCalc, `GET /api/molad?year=&month=` — an independent published
//!   calculator, used here for all of 5785 and spot rows of 5786/5787.
//!
//! Both give Jerusalem time, civil clock, with chalakim carried as a remainder.

use super::*;
use chrono::Weekday::*;
use HebrewMonth::*;

/// One row of a published molad table.
type Row = (HebrewMonth, Weekday, i32, u32, u32, u8, u8, u8);

fn check(year: i32, row: &Row) {
    let (month, weekday, gyear, gmonth, gday, hour, minute, chalakim) = *row;
    let m = MoladCalculator::of_month(year, month)
        .unwrap_or_else(|e| panic!("molad of {} {}: {}", month.name(), year, e));
    let want = NaiveDate::from_ymd_opt(gyear, gmonth, gday).unwrap();
    assert_eq!(
        (m.gregorian, m.weekday, m.hour, m.minute, m.chalakim),
        (want, weekday, hour, minute, chalakim),
        "molad of {} {}",
        month.name_for_year(year),
        year
    );
    assert_eq!(m.hebrew_year, year);
    assert_eq!(m.hebrew_month, month);
}

fn check_table(year: i32, rows: &[Row]) {
    for row in rows {
        check(year, row);
    }
    // The table must be the whole year, in the order the months occur.
    let all = MoladCalculator::of_year(year).unwrap();
    assert_eq!(all.len(), rows.len(), "month count for {}", year);
    for (m, row) in all.iter().zip(rows) {
        assert_eq!(m.hebrew_month, row.0, "month order for {}", year);
    }
}

/// Every month of 5785 (common year), from TorahCalc's molad API.
const Y5785: &[Row] = &[
    (Tishrei, Thu, 2024, 10, 3, 3, 21, 13),
    (Cheshvan, Fri, 2024, 11, 1, 16, 5, 14),
    (Kislev, Sun, 2024, 12, 1, 4, 49, 15),
    (Teves, Mon, 2024, 12, 30, 17, 33, 16),
    (Shevat, Wed, 2025, 1, 29, 6, 17, 17),
    (Adar, Thu, 2025, 2, 27, 19, 2, 0),
    (Nisan, Sat, 2025, 3, 29, 7, 46, 1),
    (Iyar, Sun, 2025, 4, 27, 20, 30, 2),
    (Sivan, Tue, 2025, 5, 27, 9, 14, 3),
    (Tammuz, Wed, 2025, 6, 25, 21, 58, 4),
    (Av, Fri, 2025, 7, 25, 10, 42, 5),
    (Elul, Sat, 2025, 8, 23, 23, 26, 6),
];

/// Every month of 5786 (common year), from Chabad's synagogue table.
const Y5786: &[Row] = &[
    (Tishrei, Mon, 2025, 9, 22, 12, 10, 7),
    (Cheshvan, Wed, 2025, 10, 22, 0, 54, 8),
    (Kislev, Thu, 2025, 11, 20, 13, 38, 9),
    (Teves, Sat, 2025, 12, 20, 2, 22, 10),
    (Shevat, Sun, 2026, 1, 18, 15, 6, 11),
    (Adar, Tue, 2026, 2, 17, 3, 50, 12),
    (Nisan, Wed, 2026, 3, 18, 16, 34, 13),
    (Iyar, Fri, 2026, 4, 17, 5, 18, 14),
    (Sivan, Sat, 2026, 5, 16, 18, 2, 15),
    (Tammuz, Mon, 2026, 6, 15, 6, 46, 16),
    (Av, Tue, 2026, 7, 14, 19, 30, 17),
    (Elul, Thu, 2026, 8, 13, 8, 15, 0),
];

/// Every month of 5787 (leap year, 13 molads), from Chabad's table.
const Y5787: &[Row] = &[
    (Tishrei, Fri, 2026, 9, 11, 20, 59, 1),
    (Cheshvan, Sun, 2026, 10, 11, 9, 43, 2),
    (Kislev, Mon, 2026, 11, 9, 22, 27, 3),
    (Teves, Wed, 2026, 12, 9, 11, 11, 4),
    (Shevat, Thu, 2027, 1, 7, 23, 55, 5),
    (AdarI, Sat, 2027, 2, 6, 12, 39, 6),
    (Adar, Mon, 2027, 3, 8, 1, 23, 7),
    (Nisan, Tue, 2027, 4, 6, 14, 7, 8),
    (Iyar, Thu, 2027, 5, 6, 2, 51, 9),
    (Sivan, Fri, 2027, 6, 4, 15, 35, 10),
    (Tammuz, Sun, 2027, 7, 4, 4, 19, 11),
    (Av, Mon, 2027, 8, 2, 17, 3, 12),
    (Elul, Wed, 2027, 9, 1, 5, 47, 13),
];

#[test]
fn molad_tishrei_5786_matches_the_published_announcement() {
    let m = MoladCalculator::of_tishrei(5786).unwrap();
    assert_eq!(m.gregorian, NaiveDate::from_ymd_opt(2025, 9, 22).unwrap());
    assert_eq!(m.weekday, Mon);
    assert_eq!((m.hour, m.minute, m.chalakim), (12, 10, 7));
    assert_eq!(m.hebrew_month, Tishrei);
    assert!(m.is_zaken(), "a molad past noon is Molad Zaken");

    // The dehiyyot move Rosh Hashanah, never the announced molad: RH 5786 was
    // postponed to Tuesday 23 Sep, one day after the molad above.
    let rh = DateConverter::rd_to_gregorian(DateConverter::rosh_hashanah(5786)).unwrap();
    assert_eq!(rh, NaiveDate::from_ymd_opt(2025, 9, 23).unwrap());
    assert_eq!(rh.weekday(), Tue);
}

#[test]
fn molad_5785_table_matches_published_values() {
    check_table(5785, Y5785);
}

#[test]
fn molad_5786_table_matches_published_values() {
    check_table(5786, Y5786);
}

#[test]
fn molad_5787_leap_year_table_matches_published_values() {
    check_table(5787, Y5787);

    // In a leap year `Adar` names Adar II, and the extra month is Adar I.
    assert_eq!(
        MoladCalculator::of_month(5787, Adar)
            .unwrap()
            .hebrew_month
            .name_for_year(5787),
        "Adar II"
    );
    assert_eq!(MoladCalculator::of_year(5787).unwrap().len(), 13);
    assert_eq!(MoladCalculator::of_year(5786).unwrap().len(), 12);
}

#[test]
fn molad_announcement_uses_the_synagogue_form() {
    assert_eq!(
        MoladCalculator::of_tishrei(5786).unwrap().announcement(),
        "Monday afternoon, 10 minutes and 7 chalakim after 12:00 PM"
    );
    assert_eq!(
        MoladCalculator::of_month(5786, Cheshvan)
            .unwrap()
            .announcement(),
        "Wednesday night, 54 minutes and 8 chalakim after 12:00 AM"
    );
    assert_eq!(
        MoladCalculator::of_month(5787, Tishrei)
            .unwrap()
            .announcement(),
        "Friday evening, 59 minutes and 1 chelek after 8:00 PM"
    );
    assert_eq!(
        MoladCalculator::of_month(5786, Elul).unwrap().announcement(),
        "Thursday morning, 15 minutes after 8:00 AM"
    );

    let m = MoladCalculator::of_tishrei(5786).unwrap();
    assert_eq!(
        m.to_string(),
        "Molad Tishrei 5786: Monday, September 22, 2025, 12:10 PM + 7 chalakim"
    );
    assert_eq!(m.gregorian_date().iso_string, "2025-09-22");
}

#[test]
fn molad_steps_by_exactly_the_mean_lunation() {
    assert_eq!(PARTS_PER_LUNATION, 765_433);

    for year in [5785i32, 5786, 5787] {
        let months = MoladCalculator::of_year(year).unwrap();
        for pair in months.windows(2) {
            let delta = pair[1].parts_since_epoch() - pair[0].parts_since_epoch();
            assert_eq!(
                delta, PARTS_PER_LUNATION,
                "{} → {} of {}",
                pair[0].hebrew_month.name(),
                pair[1].hebrew_month.name(),
                year
            );
        }
    }

    // Twelve lunations is not a year: the last molad of 5786 precedes the
    // first of 5787 by less than a month, the next by more.
    let elul = MoladCalculator::of_month(5786, Elul).unwrap();
    let next = MoladCalculator::of_tishrei(5787).unwrap();
    let gap = next.parts_since_epoch() - elul.parts_since_epoch();
    assert!((PARTS_PER_LUNATION..2 * PARTS_PER_LUNATION).contains(&gap), "{}", gap);
}

#[test]
fn molad_of_tishrei_shares_the_calendar_parts_arithmetic() {
    // The announced molad is literally `DateConverter::molad_parts`, the same
    // count `hebrew_calendar_elapsed_days` uses for Rosh Hashanah.
    for year in [5784i32, 5785, 5786, 5787, 5788, 5800] {
        let m = MoladCalculator::of_tishrei(year).unwrap();
        assert_eq!(m.parts_since_epoch(), DateConverter::molad_parts(year));

        let days = DateConverter::molad_parts(year) / PARTS_PER_DAY;
        let rd = DateConverter::gregorian_to_rd(m.gregorian) as i64;
        assert!(
            (rd - (DateConverter::hebrew_epoch_rd() as i64 + days)).abs() <= 1,
            "{}: civil date {} drifted from the elapsed-day count",
            year,
            m.gregorian
        );
        // Rosh Hashanah is the molad day with the dehiyyot applied, so it can
        // never precede the molad.
        assert!(
            DateConverter::rosh_hashanah(year) as i64 >= rd,
            "{}: Rosh Hashanah precedes its own molad",
            year
        );
    }
}

#[test]
fn molad_announced_hours_use_both_numberings() {
    // Civil clock (as printed) and the halakhic 6-pm numbering, same instant.
    let m = MoladCalculator::of_tishrei(5786).unwrap();
    assert_eq!(m.hour, 12);
    assert_eq!(m.evening_hour(), 18, "noon is hour 18 from 6 pm");
    let m = MoladCalculator::of_month(5787, Tishrei).unwrap();
    assert_eq!(m.hour, 20);
    assert_eq!(m.evening_hour(), 2);
    assert!(MoladCalculator::of_tishrei(5787).unwrap().is_zaken());
    assert!(!MoladCalculator::of_month(5786, Adar).unwrap().is_zaken());
}

#[test]
fn molad_rejects_impossible_requests() {
    // Adar I exists only in a leap year.
    let err = MoladCalculator::of_month(5786, AdarI).unwrap_err();
    assert!(matches!(err, CalendarError::CalculationError(_)), "{:?}", err);
    assert!(MoladCalculator::of_month(5787, AdarI).is_ok());

    assert!(matches!(
        MoladCalculator::of_tishrei(0).unwrap_err(),
        CalendarError::InvalidDateFormat(_)
    ));
}

#[test]
fn molad_is_reachable_from_the_crate_root_and_serializable() {
    let m: crate::Molad = crate::MoladCalculator::of_tishrei(5786).unwrap();
    let json = serde_json::to_string(&m).unwrap();
    let back: crate::Molad = serde_json::from_str(&json).unwrap();
    assert_eq!(m, back);
    assert!(json.contains("2025-09-22"));
}
