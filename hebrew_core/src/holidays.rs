//! Jewish holidays, fasts, special Shabbatot and Rosh Chodesh, for Israel
//! and outside it.
//!
//! Every rule here is checked, day by day, against Hebcal for 1950–2080 in
//! both observances (see the review harness in `hebrew_core/examples`).

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::calendar::{hebrew_numeral, DateConverter, HebrewDate, HebrewMonth};
use crate::parsha::{Parsha, ParshaCalculator};
use crate::{CalendarError, Observance};

/// A holiday, fast, special Shabbat or Rosh Chodesh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Holiday {
    // Tishrei
    RoshHashanahDay1,
    RoshHashanahDay2,
    TzomGedaliah,
    ShabbatShuva,
    ErevYomKippur,
    YomKippur,
    ErevSukkot,
    SukkotDay1,
    /// Second festival day of Sukkot (outside Israel).
    SukkotDay2,
    /// A weekday of the festival: the day of Sukkot, 2..=6.
    SukkotCholHamoed(u8),
    HoshanaRabbah,
    SheminiAtzeret,
    SimchatTorah,
    // Cheshvan
    Sigd,
    // Kislev, Tevet
    /// Day 1..=8 of Chanukah.
    Chanukah(u8),
    ChagHaBanot,
    AsaraBTevet,
    // Shevat
    TuBiShevat,
    ShabbatShirah,
    // Adar
    PurimKatan,
    ShushanPurimKatan,
    ShabbatShekalim,
    ShabbatZachor,
    TaanitEsther,
    ErevPurim,
    Purim,
    ShushanPurim,
    PurimMeshulash,
    ShabbatParah,
    ShabbatHaChodesh,
    // Nisan
    BirkatHachamah,
    YomHaAliyah,
    // Israeli civic days, listed in Israel only.
    YitzhakRabinMemorialDay,
    YomHaAliyahSchoolObservance,
    BenGurionDay,
    HebrewLanguageDay,
    FamilyDay,
    HerzlDay,
    JabotinskyDay,
    ShabbatHaGadol,
    TaanitBechorot,
    ErevPesach,
    PesachDay1,
    /// Second festival day of Pesach (outside Israel).
    PesachDay2,
    /// A weekday of the festival: the day of Pesach, 2..=6.
    PesachCholHamoed(u8),
    PesachDay7,
    /// Outside Israel only.
    PesachDay8,
    YomHaShoah,
    // Iyar
    YomHaZikaron,
    YomHaAtzmaut,
    PesachSheni,
    LagBaOmer,
    YomYerushalayim,
    // Sivan
    ErevShavuot,
    ShavuotDay1,
    /// Outside Israel only.
    ShavuotDay2,
    // Tammuz, Av
    ShivaAsarBTammuz,
    ShabbatChazon,
    ErevTishaBAv,
    TishaBAv,
    /// Tisha B'Av postponed from Shabbat to Sunday.
    TishaBAvObserved,
    ShabbatNachamu,
    TuBAv,
    // Elul
    RoshHashanaLaBehemot,
    LeilSelichot,
    ErevRoshHashanah,
    /// Rosh Chodesh of `month`; `leap` tells Adar I and Adar II apart.
    RoshChodesh {
        month: HebrewMonth,
        leap: bool,
    },
}

/// What kind of day a holiday is, for display and for the rules that depend
/// on it (candles, work, fasting).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HolidayCategory {
    /// A festival day on which work is forbidden.
    YomTov,
    /// The intermediate days of Sukkot and Pesach.
    CholHamoed,
    /// The day before a festival.
    Erev,
    Fast,
    Minor,
    /// Israeli national days.
    Modern,
    SpecialShabbat,
    RoshChodesh,
}

const ROMAN: [&str; 9] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII"];

impl Holiday {
    /// English name.
    pub fn name(&self) -> String {
        use Holiday::*;
        match self {
            RoshHashanahDay1 => "Rosh Hashanah I".into(),
            RoshHashanahDay2 => "Rosh Hashanah II".into(),
            TzomGedaliah => "Tzom Gedaliah".into(),
            ShabbatShuva => "Shabbat Shuva".into(),
            ErevYomKippur => "Erev Yom Kippur".into(),
            YomKippur => "Yom Kippur".into(),
            ErevSukkot => "Erev Sukkot".into(),
            SukkotDay1 => "Sukkot I".into(),
            SukkotDay2 => "Sukkot II".into(),
            SukkotCholHamoed(d) => format!("Sukkot {} (Chol HaMoed)", ROMAN[*d as usize]),
            HoshanaRabbah => "Hoshana Rabbah".into(),
            SheminiAtzeret => "Shemini Atzeret".into(),
            SimchatTorah => "Simchat Torah".into(),
            Sigd => "Sigd".into(),
            Chanukah(8) => "Chanukah: 8th day".into(),
            Chanukah(d) => format!("Chanukah: day {d}"),
            ChagHaBanot => "Chag HaBanot".into(),
            AsaraBTevet => "Asara B'Tevet".into(),
            TuBiShevat => "Tu BiShvat".into(),
            ShabbatShirah => "Shabbat Shirah".into(),
            PurimKatan => "Purim Katan".into(),
            ShushanPurimKatan => "Shushan Purim Katan".into(),
            ShabbatShekalim => "Shabbat Shekalim".into(),
            ShabbatZachor => "Shabbat Zachor".into(),
            TaanitEsther => "Ta'anit Esther".into(),
            ErevPurim => "Erev Purim".into(),
            Purim => "Purim".into(),
            ShushanPurim => "Shushan Purim".into(),
            PurimMeshulash => "Purim Meshulash".into(),
            ShabbatParah => "Shabbat Parah".into(),
            ShabbatHaChodesh => "Shabbat HaChodesh".into(),
            BirkatHachamah => "Birkat Hachamah".into(),
            YomHaAliyah => "Yom HaAliyah".into(),
            YitzhakRabinMemorialDay => "Yitzhak Rabin Memorial Day".into(),
            YomHaAliyahSchoolObservance => "Yom HaAliyah School Observance".into(),
            BenGurionDay => "Ben-Gurion Day".into(),
            HebrewLanguageDay => "Hebrew Language Day".into(),
            FamilyDay => "Family Day".into(),
            HerzlDay => "Herzl Day".into(),
            JabotinskyDay => "Jabotinsky Day".into(),
            ShabbatHaGadol => "Shabbat HaGadol".into(),
            TaanitBechorot => "Ta'anit Bechorot".into(),
            ErevPesach => "Erev Pesach".into(),
            PesachDay1 => "Pesach I".into(),
            PesachDay2 => "Pesach II".into(),
            PesachCholHamoed(d) => format!("Pesach {} (Chol HaMoed)", ROMAN[*d as usize]),
            PesachDay7 => "Pesach VII".into(),
            PesachDay8 => "Pesach VIII".into(),
            YomHaShoah => "Yom HaShoah".into(),
            YomHaZikaron => "Yom HaZikaron".into(),
            YomHaAtzmaut => "Yom HaAtzma'ut".into(),
            PesachSheni => "Pesach Sheni".into(),
            LagBaOmer => "Lag BaOmer".into(),
            YomYerushalayim => "Yom Yerushalayim".into(),
            ErevShavuot => "Erev Shavuot".into(),
            ShavuotDay1 => "Shavuot I".into(),
            ShavuotDay2 => "Shavuot II".into(),
            ShivaAsarBTammuz => "Tzom Tammuz".into(),
            ShabbatChazon => "Shabbat Chazon".into(),
            ErevTishaBAv => "Erev Tish'a B'Av".into(),
            TishaBAv => "Tish'a B'Av".into(),
            TishaBAvObserved => "Tish'a B'Av (observed)".into(),
            ShabbatNachamu => "Shabbat Nachamu".into(),
            TuBAv => "Tu B'Av".into(),
            RoshHashanaLaBehemot => "Rosh Hashana LaBehemot".into(),
            LeilSelichot => "Leil Selichot".into(),
            ErevRoshHashanah => "Erev Rosh Hashanah".into(),
            RoshChodesh { month, leap } => format!("Rosh Chodesh {}", month_name(*month, *leap)),
        }
    }

    /// Hebrew name.
    pub fn hebrew_name(&self) -> String {
        use Holiday::*;
        match self {
            RoshHashanahDay1 => "ראש השנה א׳".into(),
            RoshHashanahDay2 => "ראש השנה ב׳".into(),
            TzomGedaliah => "צום גדליה".into(),
            ShabbatShuva => "שבת שובה".into(),
            ErevYomKippur => "ערב יום כיפור".into(),
            YomKippur => "יום כיפור".into(),
            ErevSukkot => "ערב סוכות".into(),
            SukkotDay1 => "סוכות א׳".into(),
            SukkotDay2 => "סוכות ב׳".into(),
            SukkotCholHamoed(_) => "חול המועד סוכות".into(),
            HoshanaRabbah => "הושענא רבה".into(),
            SheminiAtzeret => "שמיני עצרת".into(),
            SimchatTorah => "שמחת תורה".into(),
            Sigd => "סיגד".into(),
            Chanukah(d) => format!("חנוכה – יום {}", hebrew_numeral(*d as u32)),
            ChagHaBanot => "חג הבנות".into(),
            AsaraBTevet => "עשרה בטבת".into(),
            TuBiShevat => "ט״ו בשבט".into(),
            ShabbatShirah => "שבת שירה".into(),
            PurimKatan => "פורים קטן".into(),
            ShushanPurimKatan => "שושן פורים קטן".into(),
            ShabbatShekalim => "שבת שקלים".into(),
            ShabbatZachor => "שבת זכור".into(),
            TaanitEsther => "תענית אסתר".into(),
            ErevPurim => "ערב פורים".into(),
            Purim => "פורים".into(),
            ShushanPurim => "שושן פורים".into(),
            PurimMeshulash => "פורים משולש".into(),
            ShabbatParah => "שבת פרה".into(),
            ShabbatHaChodesh => "שבת החודש".into(),
            BirkatHachamah => "ברכת החמה".into(),
            YomHaAliyah => "יום העלייה".into(),
            YitzhakRabinMemorialDay => "יום הזיכרון ליצחק רבין".into(),
            YomHaAliyahSchoolObservance => "יום העלייה במערכת החינוך".into(),
            BenGurionDay => "יום בן-גוריון".into(),
            HebrewLanguageDay => "יום השפה העברית".into(),
            FamilyDay => "יום המשפחה".into(),
            HerzlDay => "יום הרצל".into(),
            JabotinskyDay => "יום ז׳בוטינסקי".into(),
            ShabbatHaGadol => "שבת הגדול".into(),
            TaanitBechorot => "תענית בכורות".into(),
            ErevPesach => "ערב פסח".into(),
            PesachDay1 => "פסח א׳".into(),
            PesachDay2 => "פסח ב׳".into(),
            PesachCholHamoed(_) => "חול המועד פסח".into(),
            PesachDay7 => "שביעי של פסח".into(),
            PesachDay8 => "אחרון של פסח".into(),
            YomHaShoah => "יום השואה".into(),
            YomHaZikaron => "יום הזיכרון".into(),
            YomHaAtzmaut => "יום העצמאות".into(),
            PesachSheni => "פסח שני".into(),
            LagBaOmer => "ל״ג בעומר".into(),
            YomYerushalayim => "יום ירושלים".into(),
            ErevShavuot => "ערב שבועות".into(),
            ShavuotDay1 => "שבועות א׳".into(),
            ShavuotDay2 => "שבועות ב׳".into(),
            ShivaAsarBTammuz => "שבעה עשר בתמוז".into(),
            ShabbatChazon => "שבת חזון".into(),
            ErevTishaBAv => "ערב תשעה באב".into(),
            TishaBAv => "תשעה באב".into(),
            TishaBAvObserved => "תשעה באב (נדחה)".into(),
            ShabbatNachamu => "שבת נחמו".into(),
            TuBAv => "ט״ו באב".into(),
            RoshHashanaLaBehemot => "ראש השנה לבהמות".into(),
            LeilSelichot => "ליל סליחות".into(),
            ErevRoshHashanah => "ערב ראש השנה".into(),
            RoshChodesh { month, leap } => format!("ראש חודש {}", month.hebrew_name(*leap)),
        }
    }

    /// What kind of day this is.
    pub fn category(&self) -> HolidayCategory {
        use Holiday::*;
        use HolidayCategory as C;
        match self {
            RoshHashanahDay1 | RoshHashanahDay2 | YomKippur | SukkotDay1 | SukkotDay2
            | SheminiAtzeret | SimchatTorah | PesachDay1 | PesachDay2 | PesachDay7 | PesachDay8
            | ShavuotDay1 | ShavuotDay2 => C::YomTov,
            SukkotCholHamoed(_) | HoshanaRabbah | PesachCholHamoed(_) => C::CholHamoed,
            ErevRoshHashanah | ErevYomKippur | ErevSukkot | ErevPesach | ErevShavuot
            | ErevPurim | ErevTishaBAv => C::Erev,
            TzomGedaliah | AsaraBTevet | TaanitEsther | TaanitBechorot | ShivaAsarBTammuz
            | TishaBAv | TishaBAvObserved => C::Fast,
            YomHaShoah
            | YomHaZikaron
            | YomHaAtzmaut
            | YomYerushalayim
            | Sigd
            | YomHaAliyah
            | YitzhakRabinMemorialDay
            | YomHaAliyahSchoolObservance
            | BenGurionDay
            | HebrewLanguageDay
            | FamilyDay
            | HerzlDay
            | JabotinskyDay => C::Modern,
            ShabbatShuva | ShabbatShirah | ShabbatShekalim | ShabbatZachor | ShabbatParah
            | ShabbatHaChodesh | ShabbatHaGadol | ShabbatChazon | ShabbatNachamu => {
                C::SpecialShabbat
            }
            RoshChodesh { .. } => C::RoshChodesh,
            Chanukah(_) | ChagHaBanot | TuBiShevat | PurimKatan | ShushanPurimKatan | Purim
            | ShushanPurim | PurimMeshulash | BirkatHachamah | PesachSheni | LagBaOmer | TuBAv
            | RoshHashanaLaBehemot | LeilSelichot => C::Minor,
        }
    }

    /// A festival day on which work is forbidden.
    pub fn is_yom_tov(&self) -> bool {
        self.category() == HolidayCategory::YomTov
    }

    /// A fast day (including Yom Kippur).
    pub fn is_fast_day(&self) -> bool {
        self.category() == HolidayCategory::Fast || *self == Holiday::YomKippur
    }
}

fn month_name(month: HebrewMonth, leap: bool) -> &'static str {
    match month {
        HebrewMonth::Adar if leap => "Adar II",
        m => m.name(),
    }
}

/// Holiday calculator.
pub struct HolidayCalculator;

impl HolidayCalculator {
    /// Holidays on `date` outside Israel.
    pub fn get_holidays(date: &HebrewDate) -> Result<Vec<Holiday>, CalendarError> {
        Self::holidays_for(date, Observance::Diaspora)
    }

    /// Holidays on `date`.
    pub fn holidays_for(
        date: &HebrewDate,
        observance: Observance,
    ) -> Result<Vec<Holiday>, CalendarError> {
        let rd = DateConverter::hebrew_to_rd(*date)?;
        Ok(Self::year(date.year, observance)?
            .into_iter()
            .filter(|(d, _)| *d == rd)
            .map(|(_, h)| h)
            .collect())
    }

    /// Every holiday of Hebrew year `year` (Tishrei to Elul), as (R.D., holiday),
    /// in date order.
    pub fn year(year: i32, observance: Observance) -> Result<Vec<(i32, Holiday)>, CalendarError> {
        use Holiday::*;
        let y = Year::new(year);
        let israel = observance == Observance::Israel;
        let mut out: Vec<(i32, Holiday)> = Vec::new();
        let mut add = |rd: i32, h: Holiday| out.push((rd, h));

        // Tishrei
        add(y.rd(HebrewMonth::Tishrei, 1), RoshHashanahDay1);
        add(y.rd(HebrewMonth::Tishrei, 2), RoshHashanahDay2);
        add(
            y.postponed_from_shabbat(HebrewMonth::Tishrei, 3),
            TzomGedaliah,
        );
        add(
            y.shabbat_on_or_before(y.rd(HebrewMonth::Tishrei, 9)),
            ShabbatShuva,
        );
        add(y.rd(HebrewMonth::Tishrei, 9), ErevYomKippur);
        add(y.rd(HebrewMonth::Tishrei, 10), YomKippur);
        add(y.rd(HebrewMonth::Tishrei, 14), ErevSukkot);
        add(y.rd(HebrewMonth::Tishrei, 15), SukkotDay1);
        if israel {
            add(y.rd(HebrewMonth::Tishrei, 16), SukkotCholHamoed(2));
        } else {
            add(y.rd(HebrewMonth::Tishrei, 16), SukkotDay2);
        }
        for day in 17..=20 {
            add(y.rd(HebrewMonth::Tishrei, day), SukkotCholHamoed(day - 14));
        }
        add(y.rd(HebrewMonth::Tishrei, 21), HoshanaRabbah);
        add(y.rd(HebrewMonth::Tishrei, 22), SheminiAtzeret);
        // In Israel Simchat Torah is Shemini Atzeret itself.
        add(
            y.rd(HebrewMonth::Tishrei, if israel { 22 } else { 23 }),
            SimchatTorah,
        );

        // Cheshvan: Sigd, since 5769, moved to Thursday when it falls on Shabbat.
        if year >= 5769 {
            let sigd = y.rd(HebrewMonth::Cheshvan, 29);
            add(if weekday(sigd) == 6 { sigd - 2 } else { sigd }, Sigd);
        }

        // Kislev, Tevet
        let chanukah = y.rd(HebrewMonth::Kislev, 25);
        for day in 1..=8u8 {
            add(chanukah + day as i32 - 1, Chanukah(day));
        }
        // Chag HaBanot is the first day of Rosh Chodesh Tevet.
        add(
            y.rd(HebrewMonth::Teves, 1) - (y.days(HebrewMonth::Kislev) == 30) as i32,
            ChagHaBanot,
        );
        add(y.rd(HebrewMonth::Teves, 10), AsaraBTevet);

        // Shevat
        add(y.rd(HebrewMonth::Shevat, 15), TuBiShevat);
        for (date, parsha) in ParshaCalculator::year_readings(year, observance)? {
            if parsha == Some(Parsha::Beshalach) {
                add(DateConverter::gregorian_to_rd(date), ShabbatShirah);
            }
        }

        // Adar (Adar II in a leap year)
        if y.leap {
            add(y.rd(HebrewMonth::AdarI, 14), PurimKatan);
            add(y.rd(HebrewMonth::AdarI, 15), ShushanPurimKatan);
        }
        add(
            y.shabbat_on_or_before(y.rd(HebrewMonth::Adar, 1)),
            ShabbatShekalim,
        );
        add(
            y.shabbat_on_or_before(y.rd(HebrewMonth::Adar, 13)),
            ShabbatZachor,
        );
        add(y.advanced_from_shabbat(HebrewMonth::Adar, 13), TaanitEsther);
        add(y.rd(HebrewMonth::Adar, 13), ErevPurim);
        add(y.rd(HebrewMonth::Adar, 14), Purim);
        add(y.rd(HebrewMonth::Adar, 15), ShushanPurim);
        if weekday(y.rd(HebrewMonth::Adar, 15)) == 6 {
            add(y.rd(HebrewMonth::Adar, 16), PurimMeshulash);
        }
        let hachodesh = y.shabbat_on_or_before(y.rd(HebrewMonth::Nisan, 1));
        add(hachodesh - 7, ShabbatParah);
        add(hachodesh, ShabbatHaChodesh);

        // Nisan
        if let Some(rd) = birkat_hachamah(year) {
            add(rd, BirkatHachamah);
        }
        if year >= 5777 {
            add(y.rd(HebrewMonth::Nisan, 10), YomHaAliyah);
        }
        add(
            y.shabbat_on_or_before(y.rd(HebrewMonth::Nisan, 14)),
            ShabbatHaGadol,
        );
        add(
            y.advanced_from_shabbat(HebrewMonth::Nisan, 14),
            TaanitBechorot,
        );
        add(y.rd(HebrewMonth::Nisan, 14), ErevPesach);
        add(y.rd(HebrewMonth::Nisan, 15), PesachDay1);
        if israel {
            add(y.rd(HebrewMonth::Nisan, 16), PesachCholHamoed(2));
        } else {
            add(y.rd(HebrewMonth::Nisan, 16), PesachDay2);
        }
        for day in 17..=20 {
            add(y.rd(HebrewMonth::Nisan, day), PesachCholHamoed(day - 14));
        }
        add(y.rd(HebrewMonth::Nisan, 21), PesachDay7);
        if !israel {
            add(y.rd(HebrewMonth::Nisan, 22), PesachDay8);
        }
        if year >= 5711 {
            let shoah = y.rd(HebrewMonth::Nisan, 27);
            let moved = match weekday(shoah) {
                5 => shoah - 1, // Friday → Thursday
                0 => shoah + 1, // Sunday → Monday
                _ => shoah,
            };
            add(moved, YomHaShoah);
        }

        // Iyar
        if year >= 5708 {
            let zikaron = y.rd(HebrewMonth::Iyar, yom_hazikaron_day(&y));
            add(zikaron, YomHaZikaron);
            add(zikaron + 1, YomHaAtzmaut);
        }
        add(y.rd(HebrewMonth::Iyar, 14), PesachSheni);
        add(y.rd(HebrewMonth::Iyar, 18), LagBaOmer);
        if year >= 5727 {
            add(y.rd(HebrewMonth::Iyar, 28), YomYerushalayim);
        }

        // Sivan
        add(y.rd(HebrewMonth::Sivan, 5), ErevShavuot);
        add(y.rd(HebrewMonth::Sivan, 6), ShavuotDay1);
        if !israel {
            add(y.rd(HebrewMonth::Sivan, 7), ShavuotDay2);
        }

        // Tammuz, Av
        add(
            y.postponed_from_shabbat(HebrewMonth::Tammuz, 17),
            ShivaAsarBTammuz,
        );
        let av9 = y.rd(HebrewMonth::Av, 9);
        add(y.shabbat_on_or_before(av9), ShabbatChazon);
        if weekday(av9) == 6 {
            add(av9, ErevTishaBAv);
            add(av9 + 1, TishaBAvObserved);
        } else {
            add(av9 - 1, ErevTishaBAv);
            add(av9, TishaBAv);
        }
        add(y.shabbat_on_or_before(av9) + 7, ShabbatNachamu);
        add(y.rd(HebrewMonth::Av, 15), TuBAv);

        // Elul
        add(y.rd(HebrewMonth::Elul, 1), RoshHashanaLaBehemot);
        // Selichot begin the Saturday night at least four days before Rosh Hashanah.
        let next_rosh_hashanah = DateConverter::rosh_hashanah(year + 1);
        let mut selichot = y.shabbat_on_or_before(next_rosh_hashanah - 1);
        if next_rosh_hashanah - selichot < 5 {
            selichot -= 7;
        }
        add(selichot, LeilSelichot);
        add(y.rd(HebrewMonth::Elul, 29), ErevRoshHashanah);

        // Rosh Chodesh: the 30th of a full month and the 1st of the next.
        for (month, number) in y.months() {
            if month == HebrewMonth::Tishrei {
                continue;
            }
            let first = y.rd(month, 1);
            let rosh_chodesh = RoshChodesh {
                month,
                leap: y.leap,
            };
            let previous = previous_month_number(number, y.leap);
            if DateConverter::days_in_hebrew_month(year, previous) == 30 {
                add(first - 1, rosh_chodesh);
            }
            add(first, rosh_chodesh);
        }

        if israel {
            // Civic days fixed by Israeli law, most kept off Friday and Shabbat.
            let shift = |rd: i32, friday: i32, shabbat: i32| match weekday(rd) {
                5 => rd + friday,
                6 => rd + shabbat,
                _ => rd,
            };
            if year >= 5758 {
                let rabin = y.rd(HebrewMonth::Cheshvan, 12);
                add(shift(rabin, -1, -2), YitzhakRabinMemorialDay);
            }
            if year >= 5777 {
                add(y.rd(HebrewMonth::Cheshvan, 7), YomHaAliyahSchoolObservance);
            }
            if year >= 5737 {
                add(shift(y.rd(HebrewMonth::Kislev, 6), 2, 1), BenGurionDay);
            }
            if year >= 5773 {
                add(
                    shift(y.rd(HebrewMonth::Teves, 21), -1, -2),
                    HebrewLanguageDay,
                );
            }
            if year >= 5750 {
                add(y.rd(HebrewMonth::Shevat, 30), FamilyDay);
            }
            if year >= 5764 {
                add(shift(y.rd(HebrewMonth::Iyar, 10), 0, 1), HerzlDay);
            }
            if year >= 5765 {
                add(y.rd(HebrewMonth::Tammuz, 29), JabotinskyDay);
            }
        }

        out.sort_by_key(|(rd, _)| *rd);
        Ok(out)
    }

    /// The day of the Omer that `date` completes (counted the evening
    /// before): 16 Nisan is day 1, 5 Sivan day 49.
    pub fn omer_day(date: &HebrewDate) -> Option<u8> {
        let day = match date.month {
            HebrewMonth::Nisan if date.day >= 16 => date.day - 15,
            HebrewMonth::Iyar => 15 + date.day,
            HebrewMonth::Sivan if date.day <= 5 => 44 + date.day,
            _ => return None,
        };
        Some(day)
    }

    /// Candles lit on the evening that ends `date` (the next day's Chanukah
    /// night): 1 on 24 Kislev up to 8 on the seventh day.
    pub fn chanukah_candles_tonight(date: &HebrewDate) -> Result<Option<u8>, CalendarError> {
        let rd = DateConverter::hebrew_to_rd(*date)?;
        let first =
            DateConverter::hebrew_to_rd(HebrewDate::new(date.year, HebrewMonth::Kislev, 25))?;
        let n = rd - first + 2;
        Ok((1..=8).contains(&n).then_some(n as u8))
    }
}

/// Weekday of an R.D. day, 0 = Sunday … 6 = Shabbat.
fn weekday(rd: i32) -> i32 {
    (rd + 6).rem_euclid(7)
}

/// The Hebrew-month number that comes before `number` in a year.
fn previous_month_number(number: u8, leap: bool) -> u8 {
    match number {
        1 => {
            if leap {
                13
            } else {
                12
            }
        }
        7 => 6,
        n => n - 1,
    }
}

/// Day of Iyar on which Yom HaZikaron falls; Yom HaAtzma'ut is the next day.
/// Both move so that neither touches Shabbat, and since 5764 so that
/// Zikaron does not fall on a Sunday.
fn yom_hazikaron_day(y: &Year) -> u8 {
    let pesach = weekday(y.rd(HebrewMonth::Nisan, 15));
    match pesach {
        // Iyar 4 would be a Friday: Wednesday and Thursday instead.
        0 => 2,
        // Iyar 4 would be a Thursday, Atzma'ut a Friday: Wednesday and Thursday.
        6 => 3,
        // Iyar 4 would be a Sunday: Monday and Tuesday (since 5764).
        2 if y.year >= 5764 => 5,
        _ => 4,
    }
}

/// Birkat Hachamah: every 28 years, when Shmuel's vernal equinox falls at the
/// start of a Wednesday — Julian 26 March of a Gregorian year ≡ 21 (mod 28).
fn birkat_hachamah(hebrew_year: i32) -> Option<i32> {
    // The blessing falls in Nisan, which lies in Gregorian year hebrew_year − 3760.
    let gregorian_year = hebrew_year - 3760;
    if gregorian_year.rem_euclid(28) != 21 {
        return None;
    }
    // Julian day number of Julian-calendar 26 March (month counted from March).
    let (y, m, d) = (gregorian_year as i64 + 4800, 0i64, 26i64);
    let jdn = d + (153 * m + 2) / 5 + 365 * y + y / 4 - 32083;
    Some(DateConverter::julian_day_to_rd(jdn as i32))
}

/// Dates within one Hebrew year.
struct Year {
    year: i32,
    leap: bool,
}

impl Year {
    fn new(year: i32) -> Self {
        Self {
            year,
            leap: DateConverter::is_hebrew_leap_year(year),
        }
    }

    fn rd(&self, month: HebrewMonth, day: u8) -> i32 {
        DateConverter::hebrew_to_rd(HebrewDate::new(self.year, month, day))
            .expect("a fixed day of a valid year")
    }

    fn days(&self, month: HebrewMonth) -> u8 {
        DateConverter::days_in_hebrew_month(self.year, month.to_number(self.leap))
    }

    /// The months of the year from Tishrei, with their numbers.
    fn months(&self) -> Vec<(HebrewMonth, u8)> {
        let last = if self.leap { 13 } else { 12 };
        (7..=last)
            .chain(1..=6)
            .map(|n| {
                (
                    HebrewMonth::from_number(n, self.leap).expect("valid month"),
                    n,
                )
            })
            .collect()
    }

    fn shabbat_on_or_before(&self, rd: i32) -> i32 {
        rd - (weekday(rd) - 6).rem_euclid(7)
    }

    /// A fast that moves to Sunday when its day is Shabbat.
    fn postponed_from_shabbat(&self, month: HebrewMonth, day: u8) -> i32 {
        let rd = self.rd(month, day);
        if weekday(rd) == 6 {
            rd + 1
        } else {
            rd
        }
    }

    /// A fast that moves back to Thursday when its day is Shabbat.
    fn advanced_from_shabbat(&self, month: HebrewMonth, day: u8) -> i32 {
        let rd = self.rd(month, day);
        if weekday(rd) == 6 {
            rd - 2
        } else {
            rd
        }
    }
}

/// Every holiday between two Gregorian dates, inclusive.
pub fn holidays_between(
    start: NaiveDate,
    end: NaiveDate,
    observance: Observance,
) -> Result<Vec<(NaiveDate, Holiday)>, CalendarError> {
    let (from, to) = (
        DateConverter::gregorian_to_rd(start),
        DateConverter::gregorian_to_rd(end),
    );
    let first = DateConverter::gregorian_to_hebrew(start)?.year;
    let last = DateConverter::gregorian_to_hebrew(end)?.year;
    let mut out = Vec::new();
    for year in first..=last {
        for (rd, h) in HolidayCalculator::year(year, observance)? {
            if (from..=to).contains(&rd) {
                out.push((DateConverter::rd_to_gregorian(rd)?, h));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Holiday::*;

    fn on(y: i32, m: u32, d: u32, observance: Observance) -> Vec<Holiday> {
        let g = NaiveDate::from_ymd_opt(y, m, d).unwrap();
        let h = DateConverter::gregorian_to_hebrew(g).unwrap();
        HolidayCalculator::holidays_for(&h, observance).unwrap()
    }

    fn diaspora(y: i32, m: u32, d: u32) -> Vec<Holiday> {
        on(y, m, d, Observance::Diaspora)
    }

    #[test]
    fn high_holidays_5784() {
        assert!(diaspora(2023, 9, 16).contains(&RoshHashanahDay1));
        assert!(diaspora(2023, 9, 18).contains(&TzomGedaliah));
        assert!(diaspora(2023, 9, 23).contains(&ShabbatShuva));
        assert!(diaspora(2023, 9, 25).contains(&YomKippur));
        assert!(diaspora(2023, 9, 24).contains(&ErevYomKippur));
    }

    #[test]
    fn sukkot_differs_in_israel() {
        // 1 October 2023 was 16 Tishrei 5784.
        assert!(diaspora(2023, 10, 1).contains(&SukkotDay2));
        assert!(on(2023, 10, 1, Observance::Israel).contains(&SukkotCholHamoed(2)));
        assert!(diaspora(2023, 10, 8).contains(&SimchatTorah));
        let israel = on(2023, 10, 7, Observance::Israel);
        assert!(israel.contains(&SheminiAtzeret) && israel.contains(&SimchatTorah));
    }

    #[test]
    fn pesach_and_shavuot_are_shorter_in_israel() {
        assert!(diaspora(2024, 4, 30).contains(&PesachDay8));
        assert!(on(2024, 4, 30, Observance::Israel).is_empty());
        assert!(diaspora(2024, 6, 13).contains(&ShavuotDay2));
        assert!(!on(2024, 6, 13, Observance::Israel).contains(&ShavuotDay2));
    }

    #[test]
    fn fasts_move_off_shabbat() {
        // 5774: 3 Tishrei was Shabbat → Tzom Gedaliah on Sunday.
        assert!(diaspora(2013, 9, 8).contains(&TzomGedaliah));
        // 5784: 13 Adar II was Shabbat → Ta'anit Esther on Thursday 21 March 2024.
        assert!(diaspora(2024, 3, 21).contains(&TaanitEsther));
        // 5782: 9 Av was Shabbat → observed Sunday 7 August 2022.
        assert!(diaspora(2022, 8, 7).contains(&TishaBAvObserved));
        assert!(diaspora(2022, 8, 6).contains(&ErevTishaBAv));
    }

    #[test]
    fn chanukah_and_rosh_chodesh_tevet() {
        // 5784: Kislev had 29 days; Chanukah 8 December 2023 – 15 December 2023.
        assert!(diaspora(2023, 12, 8).contains(&Chanukah(1)));
        assert!(diaspora(2023, 12, 15).contains(&Chanukah(8)));
        let rc = RoshChodesh {
            month: HebrewMonth::Teves,
            leap: true,
        };
        assert!(diaspora(2023, 12, 13).contains(&rc));
        assert!(diaspora(2023, 12, 13).contains(&ChagHaBanot));
    }

    #[test]
    fn special_shabbatot_5784() {
        assert!(diaspora(2024, 1, 27).contains(&ShabbatShirah));
        assert!(diaspora(2024, 3, 9).contains(&ShabbatShekalim));
        assert!(diaspora(2024, 3, 23).contains(&ShabbatZachor));
        assert!(diaspora(2024, 4, 20).contains(&ShabbatHaGadol));
    }

    #[test]
    fn modern_days_5784() {
        assert!(diaspora(2024, 5, 6).contains(&YomHaShoah));
        assert!(diaspora(2024, 5, 13).contains(&YomHaZikaron));
        assert!(diaspora(2024, 5, 14).contains(&YomHaAtzmaut));
        assert!(diaspora(2024, 6, 5).contains(&YomYerushalayim));
    }

    #[test]
    fn birkat_hachamah_every_28_years() {
        assert!(diaspora(2009, 4, 8).contains(&BirkatHachamah));
        assert!(diaspora(2037, 4, 8).contains(&BirkatHachamah));
        assert!(!diaspora(2010, 4, 8).contains(&BirkatHachamah));
    }

    #[test]
    fn names_and_categories() {
        assert_eq!(SukkotCholHamoed(3).name(), "Sukkot III (Chol HaMoed)");
        assert_eq!(
            RoshChodesh {
                month: HebrewMonth::Adar,
                leap: true
            }
            .name(),
            "Rosh Chodesh Adar II"
        );
        assert_eq!(LagBaOmer.hebrew_name(), "ל״ג בעומר");
        assert!(YomKippur.is_yom_tov() && YomKippur.is_fast_day());
        assert!(!HoshanaRabbah.is_yom_tov());
        assert_eq!(ShabbatHaGadol.category(), HolidayCategory::SpecialShabbat);
    }

    #[test]
    fn omer_and_candles() {
        let d = |m, day| HebrewDate::new(5784, m, day);
        assert_eq!(
            HolidayCalculator::omer_day(&d(HebrewMonth::Nisan, 16)),
            Some(1)
        );
        assert_eq!(
            HolidayCalculator::omer_day(&d(HebrewMonth::Iyar, 18)),
            Some(33)
        );
        assert_eq!(HolidayCalculator::omer_day(&d(HebrewMonth::Sivan, 6)), None);
        let candles = |m, day| HolidayCalculator::chanukah_candles_tonight(&d(m, day)).unwrap();
        assert_eq!(candles(HebrewMonth::Kislev, 24), Some(1));
        assert_eq!(candles(HebrewMonth::Kislev, 25), Some(2));
        assert_eq!(candles(HebrewMonth::Teves, 2), Some(8));
        assert_eq!(candles(HebrewMonth::Teves, 3), None);
    }
}
