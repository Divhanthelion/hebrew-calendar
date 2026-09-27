//! Daf Yomi — the daily page of the Babylonian Talmud.
//!
//! For any Gregorian date this module reports which maseches and which daf the
//! worldwide cycle was learning that day, counting from the first cycle, which
//! began on the first day of Rosh Hashanah 5684 — **11 September 1923** — with
//! Berachos 2, and has run without interruption ever since.
//!
//! The cycle is pure day-count arithmetic from that epoch: no lookup table of
//! cycle starts, and no dependence on the Hebrew calendar beyond the epoch date
//! itself. Day `n` after the epoch is the `n`-th daf of the walk in
//! [`DAF_TABLE`], and the walk repeats every cycle.
//!
//! # Cycle length
//!
//! Exactly one change has ever been made to the cycle's length. Rav Meir
//! Shapiro's original calendar was tailored to a Yerushalmi Shekalim of 13
//! folios, so cycles 1 through 7 ran **2702** dapim. When that edition became
//! uncommon, the Daf Yomi Commission of Agudath Israel switched Shekalim to the
//! 22-folio Vilna edition, which lengthened the cycle to **2711** dapim from the
//! eighth cycle onward — the eighth cycle having begun in June 1975. Hence
//! [`DafYomiCalculator::cycle_length`] branches at cycle 8 and nowhere else.
//!
//! * 2711 − 2702 = 9 = 21 − 12, i.e. the 13-daf Shekalim contributed *twelve*
//!   learning days (folios 2..=13), not thirteen.
//! * Shekalim is entry 5 of the walk, immediately after Pesachim — not last.
//! * Me'ilah, Kinim, Tamid and Midos are four entries over **one continuous
//!   folio numbering** (2..=37), so Kinim opens at 23, Tamid at 26 and Midos at
//!   34. The walk index, not the folio number, decides which maseches a day
//!   belongs to, which is what makes the shared numbering harmless.
//!
//! # Known imprecision
//!
//! known: 2711-step is 6 days early for cycles 8-9. A constant 2711-day step
//! from 1975 puts the 8th and 9th cycle completions on 1982-11-24 and
//! 1990-04-27, where the published siyum dates are 1982-11-21 and 1990-04-24.
//! The discrepancy is gone by cycle 12, whose first day (2005-03-02) the model
//! reproduces exactly, as it does every boundary from there on. Nothing in
//! between is asserted as an oracle below.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::CalendarError;

/// The first day of the first Daf Yomi cycle: Berachos 2.
pub const CYCLE_EPOCH: NaiveDate = NaiveDate::from_ymd_opt(1923, 9, 11).unwrap();

/// Dapim in a cycle of the first seven (13-folio Shekalim).
pub const CYCLE_DAPIM_YERUSHALMI: u32 = 2702;

/// Dapim in a cycle from the eighth onward (22-folio Vilna Shekalim).
pub const CYCLE_DAPIM_VILNA: u32 = 2711;

/// The first cycle to use the 22-folio Shekalim, and therefore 2711 dapim.
pub const VILNA_CYCLE_FIRST: u32 = 8;

/// A maseches of the Daf Yomi cycle.
///
/// The variants are declared in the order the cycle learns them, so
/// [`Tractate::index`] is the walk index and the four small Kodashim
/// masechtos — whose folio numbering is shared — stay separate entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Tractate {
    Berachos,
    Shabbos,
    Eruvin,
    Pesachim,
    Shekalim,
    Yoma,
    Sukah,
    Beitzah,
    RoshHashanah,
    Taanis,
    Megilah,
    MoedKatan,
    Chagigah,
    Yevamos,
    Kesuvos,
    Nedarim,
    Nazir,
    Sotah,
    Gitin,
    Kidushin,
    BavaKama,
    BavaMetzia,
    BavaBasra,
    Sanhedrin,
    Makos,
    Shevuos,
    AvodahZarah,
    Horayos,
    Zevachim,
    Menachos,
    Chulin,
    Bechoros,
    Erchin,
    Temurah,
    Kerisus,
    Meilah,
    Kinim,
    Tamid,
    Midos,
    Nidah,
}

impl Tractate {
    /// Position in the cycle's walk, 0-based.
    pub fn index(self) -> usize {
        DAF_TABLE.iter().position(|(t, _, _)| *t == self).expect("Tractate must be in DAF_TABLE")
    }

    /// The first daf learned of this maseches (37-folio Shekalim aside — see
    /// [`DafYomiCalculator::last_daf`]).
    pub fn first_daf(self) -> u32 {
        DAF_TABLE[self.index()].1
    }

    /// The last daf learned of this maseches in a modern (2711-daf) cycle.
    pub fn last_daf(self) -> u32 {
        DAF_TABLE[self.index()].2
    }

    /// The printed name of the maseches.
    pub fn name(self) -> &'static str {
        use Tractate::*;
        match self {
            Berachos => "Berachos",
            Shabbos => "Shabbos",
            Eruvin => "Eruvin",
            Pesachim => "Pesachim",
            Shekalim => "Shekalim",
            Yoma => "Yoma",
            Sukah => "Sukah",
            Beitzah => "Beitzah",
            RoshHashanah => "Rosh Hashanah",
            Taanis => "Taanis",
            Megilah => "Megilah",
            MoedKatan => "Moed Katan",
            Chagigah => "Chagigah",
            Yevamos => "Yevamos",
            Kesuvos => "Kesuvos",
            Nedarim => "Nedarim",
            Nazir => "Nazir",
            Sotah => "Sotah",
            Gitin => "Gitin",
            Kidushin => "Kidushin",
            BavaKama => "Bava Kama",
            BavaMetzia => "Bava Metzia",
            BavaBasra => "Bava Basra",
            Sanhedrin => "Sanhedrin",
            Makos => "Makos",
            Shevuos => "Shevuos",
            AvodahZarah => "Avodah Zarah",
            Horayos => "Horayos",
            Zevachim => "Zevachim",
            Menachos => "Menachos",
            Chulin => "Chulin",
            Bechoros => "Bechoros",
            Erchin => "Erchin",
            Temurah => "Temurah",
            Kerisus => "Kerisus",
            Meilah => "Me'ilah",
            Kinim => "Kinim",
            Tamid => "Tamid",
            Midos => "Midos",
            Nidah => "Nidah",
        }
    }

    /// The 40 masechtos in the order the cycle learns them.
    pub fn all() -> Vec<Tractate> {
        DAF_TABLE.iter().map(|&(tractate, _, _)| tractate).collect()
    }
}

impl std::fmt::Display for Tractate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// The masechos of the Daf Yomi cycle in learning order, with the first and
/// last folio learned of each. `last - first + 1` is the number of days the
/// cycle spends in that maseches; the 40 counts sum to 2711.
///
/// Shekalim's `last` is the Vilna 22; in cycles 1..=7 the walk stops at 13
/// instead (see [`DafYomiCalculator::cycle_length`]). Kinim, Tamid and Midos
/// open mid-numbering because they share Me'ilah's folios.
pub const DAF_TABLE: [(Tractate, u32, u32); 40] = [
    (Tractate::Berachos, 2, 64),      //  63
    (Tractate::Shabbos, 2, 157),      // 156
    (Tractate::Eruvin, 2, 105),       // 104
    (Tractate::Pesachim, 2, 121),     // 120
    (Tractate::Shekalim, 2, 22),      //  21 (12 in cycles 1..=7)
    (Tractate::Yoma, 2, 88),          //  87
    (Tractate::Sukah, 2, 56),         //  55
    (Tractate::Beitzah, 2, 40),       //  39
    (Tractate::RoshHashanah, 2, 35),  //  34
    (Tractate::Taanis, 2, 31),        //  30
    (Tractate::Megilah, 2, 32),       //  31
    (Tractate::MoedKatan, 2, 29),     //  28
    (Tractate::Chagigah, 2, 27),      //  26
    (Tractate::Yevamos, 2, 122),      // 121
    (Tractate::Kesuvos, 2, 112),      // 111
    (Tractate::Nedarim, 2, 91),       //  90
    (Tractate::Nazir, 2, 66),         //  65
    (Tractate::Sotah, 2, 49),         //  48
    (Tractate::Gitin, 2, 90),         //  89
    (Tractate::Kidushin, 2, 82),      //  81
    (Tractate::BavaKama, 2, 119),     // 118
    (Tractate::BavaMetzia, 2, 119),   // 118
    (Tractate::BavaBasra, 2, 176),    // 175
    (Tractate::Sanhedrin, 2, 113),    // 112
    (Tractate::Makos, 2, 24),         //  23
    (Tractate::Shevuos, 2, 49),       //  48
    (Tractate::AvodahZarah, 2, 76),   //  75
    (Tractate::Horayos, 2, 14),       //  13
    (Tractate::Zevachim, 2, 120),     // 119
    (Tractate::Menachos, 2, 110),     // 109
    (Tractate::Chulin, 2, 142),       // 141
    (Tractate::Bechoros, 2, 61),      //  60
    (Tractate::Erchin, 2, 34),        //  33
    (Tractate::Temurah, 2, 34),       //  33
    (Tractate::Kerisus, 2, 28),       //  27
    (Tractate::Meilah, 2, 22),        //  21  \
    (Tractate::Kinim, 23, 25),        //   3   | one continuous
    (Tractate::Tamid, 26, 33),        //   8   | folio numbering
    (Tractate::Midos, 34, 37),        //   4  /  2..=37
    (Tractate::Nidah, 2, 73),         //  72
];

/// The daf of a given day: which maseches, which folio, and which cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DafYomi {
    /// The maseches learned that day.
    pub tractate: Tractate,
    /// The folio learned that day.
    pub daf: u32,
    /// The cycle, 1 for the cycle that began on 11 September 1923.
    pub cycle_number: u32,
}

impl std::fmt::Display for DafYomi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.tractate, self.daf)
    }
}

/// Daf Yomi lookups.
pub struct DafYomiCalculator;

impl DafYomiCalculator {
    /// Dapim — and therefore days — in the given cycle, 1-based.
    ///
    /// Cycles 1..=7 learn the 13-folio Yerushalmi Shekalim and run 2702 dapim;
    /// from cycle 8 the 22-folio Vilna Shekalim makes them 2711.
    pub fn cycle_length(cycle: u32) -> u32 {
        if cycle >= VILNA_CYCLE_FIRST {
            CYCLE_DAPIM_VILNA
        } else {
            CYCLE_DAPIM_YERUSHALMI
        }
    }

    /// The first day of the given cycle — the day it learns Berachos 2 — or
    /// `None` for cycle 0.
    ///
    /// This is the epoch plus the lengths of every earlier cycle, which is the
    /// whole of the cycle model: 1923-09-11, then seven steps of 2702 days, then
    /// steps of 2711.
    pub fn cycle_start(cycle: u32) -> Option<NaiveDate> {
        if cycle == 0 {
            return None;
        }
        let days = (1..cycle).map(|i| Self::cycle_length(i) as u64).sum();
        CYCLE_EPOCH.checked_add_days(chrono::Days::new(days))
    }

    /// The last day of the given cycle — the day it learns its final daf.
    ///
    /// This is the day of the last *daf*, which from cycle 8 onward is about a
    /// month before the Siyum HaShas celebration.
    pub fn cycle_end(cycle: u32) -> Option<NaiveDate> {
        Self::cycle_start(cycle)?
            .checked_sub_days(chrono::Days::new(1))?
            .checked_add_days(chrono::Days::new(Self::cycle_length(cycle) as u64))
    }

    /// The cycle containing `date`, 1-based.
    pub fn cycle_number(date: NaiveDate) -> u32 {
        if date < CYCLE_EPOCH {
            return 0;
        }
        let mut cycle = 1;
        let mut day = CYCLE_EPOCH;
        loop {
            let len = Self::cycle_length(cycle) as i64;
            match day.checked_add_days(chrono::Days::new(len as u64)) {
                Some(next) if date >= next => {
                    day = next;
                    cycle += 1;
                }
                _ => return cycle,
            }
        }
    }

    /// The daf learned on `date`.
    ///
    /// Errors with [`CalendarError::DateOutOfRange`] before 11 September 1923,
    /// when the cycle did not yet exist.
    pub fn for_date(date: NaiveDate) -> Result<DafYomi, CalendarError> {
        if date < CYCLE_EPOCH {
            return Err(CalendarError::DateOutOfRange(format!(
                "Daf Yomi cycle 1 began on {}; {} predates the cycle",
                CYCLE_EPOCH.format("%d %B %Y"),
                date.format("%d %B %Y")
            )));
        }

        let cycle = Self::cycle_number(date);
        let start = Self::cycle_start(cycle)
            .ok_or_else(|| CalendarError::CalculationError("cycle 0 has no start".into()))?;
        // Days since the cycle's first day, which is also the index of today's
        // daf within the cycle's walk.
        let offset = (date - start).num_days().max(0) as u32;

        let (tractate, daf) = Self::daf_at(cycle, offset)?;
        Ok(DafYomi {
            tractate,
            daf,
            cycle_number: cycle,
        })
    }

    /// The `offset`-th daf (0-based) of the given cycle's walk.
    fn daf_at(cycle: u32, offset: u32) -> Result<(Tractate, u32), CalendarError> {
        if offset >= Self::cycle_length(cycle) {
            return Err(CalendarError::CalculationError(format!(
                "daf offset {} is outside a {}-daf cycle",
                offset,
                Self::cycle_length(cycle)
            )));
        }
        let mut remaining = offset;
        for &(tractate, first, last) in DAF_TABLE.iter() {
            // Cycles 1..=7 stop the Shekalim walk at folio 13 of the 13-folio
            // Yerushalmi edition; the table's 22 is the Vilna form.
            let last = if tractate == Tractate::Shekalim && cycle < VILNA_CYCLE_FIRST {
                13
            } else {
                last
            };
            let count = last - first + 1;
            if remaining < count {
                return Ok((tractate, first + remaining));
            }
            remaining -= count;
        }
        Err(CalendarError::CalculationError(
            "daf fell off the end of the tractate table".into(),
        ))
    }

    /// Every daf of the given cycle, in order. Used by the tests to check the
    /// table's arithmetic; a cycle is ~2700 entries, so do not call it per day.
    pub fn cycle_dapim(cycle: u32) -> Vec<DafYomi> {
        let len = Self::cycle_length(cycle);
        (0..len)
            .map(|offset| {
                let (tractate, daf) = Self::daf_at(cycle, offset)
                    .expect("every offset below the cycle length is on the walk");
                DafYomi {
                    tractate,
                    daf,
                    cycle_number: cycle,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests;
