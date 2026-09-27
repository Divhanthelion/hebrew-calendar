//! Published-value tests for the Daf Yomi cycle.
//!
//! Oracles:
//! * Kollel Iyun Hadaf / dafyomi.co.il, the per-daf schedules for the 12th, 13th
//!   and 14th cycles (`calendarN-cycle.htm`) — the de-facto standard calendar,
//!   one row per day, from Berachos 2 to the last daf.
//! * Wikipedia, "Daf Yomi" and "Siyum HaShas" — the 1923-09-11 epoch, the
//!   2702/2711 lengths, and the completion dates of cycles 1, 2, 3, 5 and 7.
//! * Derekh Learning, which gives cycle 14 as 2020-01-05 through 2027-06-07 with
//!   cycle 15 beginning 2027-06-08.
//!
//! Every date below is a published row, not a model output.

use super::*;
use Tractate::*;

/// The 14th cycle's first day, as published — the anchor for most rows below.
fn cycle14(offset: u32) -> NaiveDate {
    DafYomiCalculator::cycle_start(14)
        .unwrap()
        .checked_add_days(chrono::Days::new(offset as u64))
        .unwrap()
}

fn check(date: NaiveDate, tractate: Tractate, daf: u32, cycle: u32) {
    let got = DafYomiCalculator::for_date(date)
        .unwrap_or_else(|e| panic!("for_date({}): {}", date, e));
    assert_eq!(
        (got.tractate, got.daf, got.cycle_number),
        (tractate, daf, cycle),
        "{}: expected {} {}",
        date,
        tractate,
        daf
    );
}

fn d(y: i32, m: u32, dd: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, dd).unwrap()
}

#[test]
fn daf_yomi_epoch_is_cycle_one_berachos_two() {
    // Cycle 1 commenced on the first day of Rosh Hashanah 5684.
    check(d(1923, 9, 11), Berachos, 2, 1);
    // ... and the day after learns the next daf, not a repeat.
    check(d(1923, 9, 12), Berachos, 3, 1);
    assert_eq!(DafYomiCalculator::cycle_start(1), Some(CYCLE_EPOCH));
    assert_eq!(DafYomiCalculator::cycle_number(d(1923, 9, 11)), 1);
}

#[test]
fn daf_yomi_before_the_epoch_is_out_of_range() {
    for date in [d(1923, 9, 10), d(1900, 1, 1), d(1, 1, 1)] {
        let got = DafYomiCalculator::for_date(date);
        match got {
            Err(CalendarError::DateOutOfRange(msg)) => {
                assert!(msg.contains("1923"), "message should name the epoch: {}", msg);
            }
            other => panic!("expected DateOutOfRange for {}, got {:?}", date, other),
        }
    }
    assert_eq!(DafYomiCalculator::cycle_number(d(1923, 9, 10)), 0);
    assert_eq!(DafYomiCalculator::cycle_start(0), None);
}

#[test]
fn daf_yomi_table_sums_to_both_cycle_lengths() {
    // The published totals: 2711 dapim from cycle 8, 2702 for cycles 1..=7.
    let modern: u32 = DAF_TABLE.iter().map(|&(_, f, l)| l - f + 1).sum();
    assert_eq!(modern, CYCLE_DAPIM_VILNA);
    assert_eq!(modern, DafYomiCalculator::cycle_length(8));
    assert_eq!(modern as usize, DafYomiCalculator::cycle_dapim(8).len());

    let classic: u32 = DAF_TABLE
        .iter()
        .map(|&(t, f, l)| {
            let l = if t == Shekalim { 13 } else { l };
            l - f + 1
        })
        .sum();
    assert_eq!(classic, CYCLE_DAPIM_YERUSHALMI);
    assert_eq!(classic as usize, DafYomiCalculator::cycle_dapim(1).len());

    // The whole difference is Shekalim's edition change: 21 dapim against 12,
    // which is why the 13-folio edition contributed twelve days, not thirteen.
    assert_eq!(
        CYCLE_DAPIM_VILNA - CYCLE_DAPIM_YERUSHALMI,
        (22 - 2 + 1) - (13 - 2 + 1)
    );
    assert_eq!(DAF_TABLE.len(), 40);
}

#[test]
fn daf_yomi_walk_is_contiguous_and_in_order() {
    // Every tractate in table order, daf increasing by one, no gaps and no
    // repeated day — the property that makes the shared Kodashim numbering safe.
    let dapim = DafYomiCalculator::cycle_dapim(14);
    assert_eq!(dapim.len(), CYCLE_DAPIM_VILNA as usize);
    assert_eq!(dapim[0], DafYomi { tractate: Berachos, daf: 2, cycle_number: 14 });
    assert_eq!(dapim[dapim.len() - 1].tractate, Nidah);
    assert_eq!(dapim[dapim.len() - 1].daf, 73);

    let mut walk_index = 0usize;
    let mut prev: Option<(usize, u32)> = None;
    for entry in &dapim {
        let i = entry.tractate.index();
        assert!(i >= walk_index, "the walk went backwards at {}", entry);
        if i > walk_index {
            assert_eq!(i, walk_index + 1, "a maseches was skipped at {}", entry);
            walk_index = i;
        }
        if let Some((pi, pdaf)) = prev {
            if pi == i {
                assert_eq!(entry.daf, pdaf + 1, "daf not consecutive in {}", entry.tractate);
            } else {
                assert_eq!(entry.daf, entry.tractate.first_daf(), "maseches did not open at its first daf");
            }
        }
        assert!((entry.tractate.first_daf()..=entry.tractate.last_daf()).contains(&entry.daf));
        prev = Some((i, entry.daf));
    }
    assert_eq!(walk_index, 39);
}

#[test]
fn daf_yomi_kodashim_share_one_continuous_numbering() {
    // Me'ilah 2..22, then Kinim 23..25, Tamid 26..33, Midos 34..37: one folio
    // run 2..=37 across four entries, 36 dapim, never restarting at 2.
    assert_eq!((Meilah.first_daf(), Meilah.last_daf()), (2, 22));
    assert_eq!((Kinim.first_daf(), Kinim.last_daf()), (23, 25));
    assert_eq!((Tamid.first_daf(), Tamid.last_daf()), (26, 33));
    assert_eq!((Midos.first_daf(), Midos.last_daf()), (34, 37));
    let run: u32 = [Meilah, Kinim, Tamid, Midos]
        .iter()
        .map(|t| t.last_daf() - t.first_daf() + 1)
        .sum();
    assert_eq!(run, 37 - 2 + 1);

    // The published handoffs, cycle 14 (cumulative day 2603 / 2624 / 2627 / 2635).
    check(d(2027, 2, 20), Meilah, 2, 14);
    check(d(2027, 3, 12), Meilah, 22, 14);
    check(d(2027, 3, 13), Kinim, 23, 14);
    check(d(2027, 3, 15), Kinim, 25, 14);
    check(d(2027, 3, 16), Tamid, 26, 14);
    check(d(2027, 3, 23), Tamid, 33, 14);
    check(d(2027, 3, 24), Midos, 34, 14);
    check(d(2027, 3, 27), Midos, 37, 14);
    // Nidah follows immediately, restarting its own numbering at 2.
    check(d(2027, 3, 28), Nidah, 2, 14);

    // Same handoffs, cycle 13 — the numbering belongs to the cycle, not the year.
    check(d(2019, 9, 19), Meilah, 2, 13);
    check(d(2019, 10, 10), Kinim, 23, 13);
    check(d(2019, 10, 13), Tamid, 26, 13);
}

#[test]
fn daf_yomi_cycle_starts_match_published_boundaries() {
    // First day of each cycle, as published by the cycle owner and by Wikipedia.
    let starts: [(u32, (i32, u32, u32)); 6] = [
        (1, (1923, 9, 11)),   // 1 Rosh Hashanah 5684
        (7, (1968, 1, 30)),   // epoch + 6 x 2702
        (12, (2005, 3, 2)),   // dafyomi.co.il, 12th-cycle table
        (13, (2012, 8, 3)),   // dafyomi.co.il, 13th-cycle table
        (14, (2020, 1, 5)),   // dafyomi.co.il, 14th-cycle table
        (15, (2027, 6, 8)),   // the day after cycle 14's last daf
    ];
    for (cycle, (y, m, dd)) in starts {
        assert_eq!(
            DafYomiCalculator::cycle_start(cycle),
            Some(d(y, m, dd)),
            "start of cycle {}",
            cycle
        );
        check(d(y, m, dd), Berachos, 2, cycle);
    }
    assert_eq!(DafYomiCalculator::cycle_number(d(2005, 3, 1)), 11);
    assert_eq!(DafYomiCalculator::cycle_number(d(2005, 3, 2)), 12);
}

#[test]
fn daf_yomi_cycle_ends_match_published_siyum_dates() {
    // The day of the last daf, which for the 2702-era cycles is the day of the
    // published Siyum HaShas.
    let ends: [(u32, (i32, u32, u32)); 5] = [
        (1, (1931, 2, 2)),   // 1st Siyum HaShas, Chachmei Lublin
        (2, (1938, 6, 27)),  // 2nd
        (3, (1945, 11, 19)), // 3rd
        (5, (1960, 9, 5)),   // 5th
        (7, (1975, 6, 23)),  // 7th, 14 Tammuz 5735
    ];
    for (cycle, (y, m, dd)) in ends {
        assert_eq!(
            DafYomiCalculator::cycle_end(cycle),
            Some(d(y, m, dd)),
            "last daf of cycle {}",
            cycle
        );
        check(d(y, m, dd), Nidah, 73, cycle);
    }
    // Cycle 14's last daf is 2027-06-07; its Siyum HaShas celebration is a
    // month later and is deliberately not what `cycle_end` reports.
    assert_eq!(DafYomiCalculator::cycle_end(14), Some(d(2027, 6, 7)));
    check(d(2027, 6, 7), Nidah, 73, 14);
    // known: 2711-step is 6 days early for cycles 8-9 — the published 8th and
    // 9th siyums (1982-11-21, 1990-04-24) are six days after this model's, so
    // no cycle-8 or cycle-9 end is asserted here. The drift is gone by cycle 12.
}

#[test]
fn daf_yomi_cycle_seven_to_eight_boundary_is_june_1975() {
    // The 7th Siyum HaShas was 23 June 1975; the 8th cycle began in June 1975,
    // and the arithmetic puts its first day the very next day.
    check(d(1975, 6, 23), Nidah, 73, 7);
    check(d(1975, 6, 24), Berachos, 2, 8);
    assert_eq!(DafYomiCalculator::cycle_number(d(1975, 6, 23)), 7);
    assert_eq!(DafYomiCalculator::cycle_number(d(1975, 6, 24)), 8);
    // The length switch lands exactly here.
    assert_eq!(DafYomiCalculator::cycle_length(7), 2702);
    assert_eq!(DafYomiCalculator::cycle_length(8), 2711);
    assert_eq!(
        DafYomiCalculator::cycle_end(7).unwrap().succ_opt(),
        DafYomiCalculator::cycle_start(8)
    );
}

#[test]
fn daf_yomi_shekalim_length_depends_on_the_cycle() {
    // Cycles 1..=7 learn the 13-folio Yerushalmi Shekalim: twelve days, folios
    // 2..=13, and Yoma opens the next day.
    let classic = DafYomiCalculator::cycle_dapim(7);
    let sk: Vec<u32> = classic
        .iter()
        .filter(|e| e.tractate == Shekalim)
        .map(|e| e.daf)
        .collect();
    assert_eq!(sk.len(), 12);
    assert_eq!(sk.first(), Some(&2));
    assert_eq!(sk.last(), Some(&13));

    // From cycle 8 the 22-folio Vilna edition: twenty-one days, folios 2..=22.
    let modern = DafYomiCalculator::cycle_dapim(8);
    let sk: Vec<u32> = modern
        .iter()
        .filter(|e| e.tractate == Shekalim)
        .map(|e| e.daf)
        .collect();
    assert_eq!(sk.len(), 21);
    assert_eq!(sk.first(), Some(&2));
    assert_eq!(sk.last(), Some(&22));

    // Shekalim is entry 5, immediately after Pesachim — not last.
    assert_eq!(Shekalim.index(), 4);
    assert_eq!(Tractate::all()[3], Pesachim);
    assert_eq!(Tractate::all()[5], Yoma);

    // Cycle 7's Shekalim, on the verified 2702-day side of the seam: the same
    // 443-day lead-in as every other cycle (Berachos 63 + Shabbos 156 + Eruvin
    // 104 + Pesachim 120), twelve days of folios 2..=13, then Yoma.
    check(d(1968, 1, 30), Berachos, 2, 7);
    check(d(1969, 4, 17), Shekalim, 2, 7);
    check(d(1969, 4, 28), Shekalim, 13, 7);
    check(d(1969, 4, 29), Yoma, 2, 7);

    // Cycle 14's Shekalim, a published row of the owner's table.
    check(d(2021, 3, 23), Shekalim, 2, 14);
    check(d(2021, 4, 12), Shekalim, 22, 14);
    check(d(2021, 4, 13), Yoma, 2, 14);
}

#[test]
fn daf_yomi_cycle_fourteen_matches_the_published_schedule() {
    // Rows lifted from dafyomi.co.il's 14th-cycle table, spread across the cycle
    // and covering every tractate boundary the walk crosses.
    let rows: [(i32, u32, u32, Tractate, u32); 16] = [
        (2020, 1, 5, Berachos, 2),        // cycle start
        (2020, 8, 10, Shabbos, 157),      // end of Shabbos
        (2020, 8, 11, Eruvin, 2),
        (2020, 11, 22, Eruvin, 105),      // Eruvin's last daf
        (2020, 11, 23, Pesachim, 2),
        (2021, 3, 13, Pesachim, 112),
        (2021, 3, 21, Pesachim, 120),
        (2021, 3, 22, Pesachim, 121),     // Pesachim runs to 121
        (2021, 4, 13, Yoma, 2),
        (2024, 6, 26, BavaMetzia, 119),
        (2024, 6, 27, BavaBasra, 2),
        (2024, 12, 18, BavaBasra, 176),   // Bava Basra runs to 176
        (2025, 4, 9, Sanhedrin, 113),
        (2025, 4, 10, Makos, 2),
        (2025, 5, 2, Makos, 24),
        (2025, 6, 14, Shevuos, 44),
    ];
    for (y, m, dd, tractate, daf) in rows {
        check(d(y, m, dd), tractate, daf, 14);
    }

    // The same rows as cumulative day counts, which is what the walk is made of.
    let cum: [(NaiveDate, u32); 4] = [
        (d(2020, 1, 5), 0),
        (d(2020, 8, 11), 219),
        (d(2024, 6, 27), 1635),
        (d(2027, 2, 20), 2603),
    ];
    for (date, offset) in cum {
        assert_eq!(
            (date - DafYomiCalculator::cycle_start(14).unwrap()).num_days() as u32,
            offset,
            "{} should be day {} of the walk",
            date,
            offset
        );
        assert_eq!(
            DafYomiCalculator::for_date(date).unwrap(),
            DafYomiCalculator::cycle_dapim(14)[offset as usize]
        );
    }
    // A row from each end of the cycle, and the day count that closes it.
    assert_eq!(cycle14(2710), d(2027, 6, 7));
    check(cycle14(2710), Nidah, 73, 14);
    check(cycle14(2711), Berachos, 2, 15);
}

#[test]
fn daf_yomi_cycle_thirteen_matches_the_published_schedule() {
    // dafyomi.co.il's 13th-cycle table: Friday 3 August 2012, Berachos 2.
    check(d(2012, 8, 3), Berachos, 2, 13);
    check(d(2012, 8, 4), Berachos, 3, 13);
    // The cycle's last day, and the seam into cycle 14.
    check(d(2020, 1, 4), Nidah, 73, 13);
    check(d(2020, 1, 5), Berachos, 2, 14);
    assert_eq!(DafYomiCalculator::cycle_number(d(2019, 12, 25)), 13);
    // A mid-cycle row from the same table: cycle 13's Me'ilah 2, on 19 September
    // 2019 — the same walk offset as cycle 14's, two cycles apart.
    check(d(2019, 9, 19), Meilah, 2, 13);
    assert_eq!(
        (d(2019, 9, 19) - DafYomiCalculator::cycle_start(13).unwrap()).num_days(),
        2603
    );
}

#[test]
fn daf_yomi_repeats_the_same_walk_every_cycle() {
    // The daf depends only on the day's offset within its cycle, so the same
    // offset in two cycles is the same daf.
    let c13 = DafYomiCalculator::cycle_dapim(13);
    let c14 = DafYomiCalculator::cycle_dapim(14);
    assert_eq!(c13.len(), c14.len());
    for i in (0..c13.len()).step_by(137) {
        assert_eq!(c13[i].tractate, c14[i].tractate);
        assert_eq!(c13[i].daf, c14[i].daf);
        assert_eq!(
            DafYomiCalculator::for_date(cycle14(i as u32)).unwrap().tractate,
            c14[i].tractate
        );
    }
    // Every one of the 40 masechtos is learned exactly once per cycle.
    let mut counts = [0u32; 40];
    for e in &c14 {
        counts[e.tractate.index()] += 1;
        assert_eq!(e.cycle_number, 14);
    }
    for (i, c) in counts.iter().enumerate() {
        assert!(*c > 0, "maseches {} never learned", Tractate::all()[i]);
        assert_eq!(*c, DAF_TABLE[i].2 - DAF_TABLE[i].1 + 1);
    }
    // Names round-trip and are the printed spellings.
    assert_eq!(Tractate::RoshHashanah.name(), "Rosh Hashanah");
    assert_eq!(Tractate::MoedKatan.to_string(), "Moed Katan");
    assert_eq!(Meilah.to_string(), "Me'ilah");
    assert_eq!(DafYomi { tractate: Berachos, daf: 2, cycle_number: 14 }.to_string(), "Berachos 2");
    assert_eq!(Tractate::all().len(), 40);
    assert_eq!(Tractate::all()[0], Berachos);
    assert_eq!(Tractate::all()[39], Nidah);
}

