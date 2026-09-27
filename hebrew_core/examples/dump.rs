//! Print the calendar's answer for every day in a range, one JSON object per
//! line, for `tools/check_against_hebcal.py` to compare with Hebcal.
//!
//! cargo run --release -p hebrew_core --example dump -- 1950-01-01 2080-12-31 diaspora

use chrono::{Datelike, NaiveDate, Weekday};
use hebrew_core::{HebrewCalendar, Observance, Settings};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: dump FROM TO diaspora|israel");
        std::process::exit(2);
    }
    let from: NaiveDate = args[1].parse().expect("FROM as YYYY-MM-DD");
    let to: NaiveDate = args[2].parse().expect("TO as YYYY-MM-DD");
    let observance = match args[3].as_str() {
        "israel" => Observance::Israel,
        _ => Observance::Diaspora,
    };
    let settings = Settings {
        observance,
        ..Settings::default()
    };
    for date in from.iter_days().take_while(|d| *d <= to) {
        let day = HebrewCalendar::day(date, &settings).expect("every day in range");
        let shabbat = date.weekday() == Weekday::Sat;
        let line = serde_json::json!({
            "g": date.to_string(),
            "hy": day.hebrew.year,
            "hm": day.hebrew.month.name_for_year(day.hebrew.year),
            "hd": day.hebrew.day,
            "hol": day.holidays.iter().map(|h| h.name.clone()).collect::<Vec<_>>(),
            "candles": day.chanukah_candles,
            "parsha": if shabbat { Some(day.parsha.map(|p| p.name).unwrap_or_default()) } else { None },
            "daf": day.daf_yomi.map(|d| d.name),
            "mevarchim": day.mevarchim.map(|m| serde_json::json!({
                "month": m.month,
                "wd": format!("{:?}", m.molad.weekday),
                "h": m.molad.hour,
                "min": m.molad.minute,
                "ch": m.molad.chalakim,
            })),
        });
        println!("{line}");
    }
}
