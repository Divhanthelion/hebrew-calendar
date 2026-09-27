//! Zmanim: halachic times of day.
//!
//! The sun's position comes from NOAA's solar equations, solved twice so the
//! position is taken at the moment of the event rather than at noon. Times
//! are computed as instants and shown in the location's own time zone (an
//! IANA name such as `America/New_York`), so daylight saving time is applied
//! on the dates it is in force.
//!
//! Conventions match Hebcal's defaults: sunrise and sunset at sea level
//! (0.833° below the horizon) unless an elevation is given; dawn (alot
//! hashachar) at 16.1°; misheyakir at 11.5°; nightfall (tzeit) at 8.5°;
//! proportional hours from sunrise to sunset (GRA) or from 72 minutes before
//! sunrise to 72 minutes after sunset (Magen Avraham).

use chrono::{DateTime, Duration, NaiveDate, Offset, TimeZone, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use crate::calendar::DateConverter;
use crate::CalendarError;

/// Where zmanim are calculated for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    /// Metres above sea level. Only sunrise and sunset use it (the sun is
    /// seen earlier from a height); zero gives sea-level times.
    pub elevation_meters: f64,
    /// IANA time zone name, e.g. `Asia/Jerusalem`.
    pub timezone: String,
    pub location_name: Option<String>,
}

impl GeoLocation {
    /// A location in UTC at sea level; set the zone with [`Self::with_timezone`].
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, CalendarError> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(CalendarError::InvalidLatitude(latitude));
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(CalendarError::InvalidLongitude(longitude));
        }
        Ok(Self {
            latitude,
            longitude,
            elevation_meters: 0.0,
            timezone: "UTC".to_string(),
            location_name: None,
        })
    }

    pub fn with_elevation(mut self, elevation_meters: f64) -> Self {
        self.elevation_meters = elevation_meters.max(0.0);
        self
    }

    /// Set the time zone by IANA name.
    pub fn with_timezone(mut self, timezone: &str) -> Result<Self, CalendarError> {
        parse_timezone(timezone)?;
        self.timezone = timezone.to_string();
        Ok(self)
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.location_name = Some(name.into());
        self
    }

    /// The time zone.
    pub fn tz(&self) -> Result<Tz, CalendarError> {
        parse_timezone(&self.timezone)
    }

    /// Jerusalem.
    pub fn jerusalem() -> Self {
        Self {
            latitude: 31.7683,
            longitude: 35.2137,
            elevation_meters: 0.0,
            timezone: "Asia/Jerusalem".to_string(),
            location_name: Some("Jerusalem".to_string()),
        }
    }

    /// New York.
    pub fn new_york() -> Self {
        Self {
            latitude: 40.7128,
            longitude: -74.0060,
            elevation_meters: 0.0,
            timezone: "America/New_York".to_string(),
            location_name: Some("New York".to_string()),
        }
    }
}

fn parse_timezone(name: &str) -> Result<Tz, CalendarError> {
    name.parse::<Tz>()
        .map_err(|_| CalendarError::InvalidTimezone(name.to_string()))
}

/// Zmanim for one day, as local clock times ("HH:MM"). A time is `None`
/// when the sun does not reach that angle that day (far north or south).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Zmanim {
    pub date: String,
    pub location: GeoLocation,
    /// The zone's UTC offset that day, e.g. "+03:00".
    pub utc_offset: String,
    /// Dawn: sun 16.1° below the horizon.
    pub alot_hashachar: Option<String>,
    /// Earliest tallit and tefillin: 11.5°.
    pub misheyakir: Option<String>,
    pub sunrise: Option<String>,
    pub sof_zman_shema_mga: Option<String>,
    pub sof_zman_shema_gra: Option<String>,
    pub sof_zman_tefila_mga: Option<String>,
    pub sof_zman_tefila_gra: Option<String>,
    /// Midday: halfway between sunrise and sunset.
    pub chatzot: Option<String>,
    pub mincha_gedola: Option<String>,
    pub mincha_ketana: Option<String>,
    pub plag_hamincha: Option<String>,
    pub sunset: Option<String>,
    /// Nightfall: 8.5°.
    pub tzeit_hakochavim: Option<String>,
    /// 72 minutes after sunset.
    pub tzeit_72_min: Option<String>,
}

/// Solar angles below the horizon, in degrees.
const SUNRISE_SUNSET: f64 = 0.833;
const ALOT: f64 = 16.1;
const MISHEYAKIR: f64 = 11.5;
const TZEIT: f64 = 8.5;

/// Zmanim calculator for one location.
pub struct ZmanimCalculator {
    location: GeoLocation,
}

impl ZmanimCalculator {
    pub fn new(location: GeoLocation) -> Self {
        Self { location }
    }

    /// All zmanim for `date` (a civil date in the location's zone).
    pub fn calculate(&self, date: NaiveDate) -> Result<Zmanim, CalendarError> {
        let tz = self.location.tz()?;
        let show = |t: Option<DateTime<Utc>>| t.map(|t| clock(t, &tz));
        let sunrise = self.sunrise(date);
        let sunset = self.sunset(date);
        let hours = |start: DateTime<Utc>, end: DateTime<Utc>, h: f64| {
            let span = (end - start).num_milliseconds() as f64;
            start + Duration::milliseconds((span * h / 12.0) as i64)
        };
        let gra = |h: f64| sunrise.zip(sunset).map(|(r, s)| hours(r, s, h));
        let mga = |h: f64| {
            sunrise
                .zip(sunset)
                .map(|(r, s)| hours(r - Duration::minutes(72), s + Duration::minutes(72), h))
        };
        Ok(Zmanim {
            date: date.to_string(),
            location: self.location.clone(),
            utc_offset: utc_offset(date, &tz),
            alot_hashachar: show(self.event(date, ALOT, true)),
            misheyakir: show(self.event(date, MISHEYAKIR, true)),
            sunrise: show(sunrise),
            sof_zman_shema_mga: show(mga(3.0)),
            sof_zman_shema_gra: show(gra(3.0)),
            sof_zman_tefila_mga: show(mga(4.0)),
            sof_zman_tefila_gra: show(gra(4.0)),
            chatzot: show(gra(6.0)),
            mincha_gedola: show(gra(6.5)),
            mincha_ketana: show(gra(9.5)),
            plag_hamincha: show(gra(10.75)),
            sunset: show(sunset),
            tzeit_hakochavim: show(self.event(date, TZEIT, false)),
            tzeit_72_min: show(sunset.map(|s| s + Duration::minutes(72))),
        })
    }

    /// Candle lighting: `minutes` before sunset on `date`.
    pub fn candle_lighting(
        &self,
        date: NaiveDate,
        minutes: i64,
    ) -> Result<Option<String>, CalendarError> {
        let tz = self.location.tz()?;
        Ok(self
            .sunset(date)
            .map(|s| clock(s - Duration::minutes(minutes), &tz)))
    }

    /// Nightfall (8.5°), when Shabbat and festivals end and a second
    /// festival night's candles are lit.
    pub fn nightfall(&self, date: NaiveDate) -> Result<Option<String>, CalendarError> {
        let tz = self.location.tz()?;
        Ok(self.event(date, TZEIT, false).map(|t| clock(t, &tz)))
    }

    /// Sunrise as an instant.
    pub fn sunrise(&self, date: NaiveDate) -> Option<DateTime<Utc>> {
        self.event(date, self.horizon(), true)
    }

    /// Sunset as an instant.
    pub fn sunset(&self, date: NaiveDate) -> Option<DateTime<Utc>> {
        self.event(date, self.horizon(), false)
    }

    /// Degrees below the horizon at which the sun's upper edge is seen,
    /// lowered for an observer above sea level.
    fn horizon(&self) -> f64 {
        SUNRISE_SUNSET + 0.0347 * self.location.elevation_meters.max(0.0).sqrt()
    }

    /// When the sun is `depression` degrees below the horizon on `date`,
    /// rising or setting, as an instant.
    pub fn event(&self, date: NaiveDate, depression: f64, rising: bool) -> Option<DateTime<Utc>> {
        let midnight = Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0)?);
        // Julian day of 0h UT on `date`.
        let jd0 =
            DateConverter::rd_to_julian_day(DateConverter::gregorian_to_rd(date)) as f64 - 0.5;
        // Start from local noon, then solve again at the event itself.
        let mut minutes = 720.0 - 4.0 * self.location.longitude;
        for _ in 0..2 {
            minutes = self.event_minutes(jd0 + minutes / 1440.0, depression, rising)?;
        }
        Some(midnight + Duration::milliseconds((minutes * 60_000.0).round() as i64))
    }

    /// Minutes after 0h UT of the event, with the sun's position taken at
    /// Julian day `jd`.
    fn event_minutes(&self, jd: f64, depression: f64, rising: bool) -> Option<f64> {
        let (declination, equation_of_time) = sun_position(jd);
        let lat = self.location.latitude.to_radians();
        let cos_hour_angle = ((-depression).to_radians().sin() - lat.sin() * declination.sin())
            / (lat.cos() * declination.cos());
        if !(-1.0..=1.0).contains(&cos_hour_angle) {
            return None;
        }
        let hour_angle = cos_hour_angle.acos().to_degrees();
        let noon = 720.0 - 4.0 * self.location.longitude - equation_of_time;
        Some(if rising {
            noon - 4.0 * hour_angle
        } else {
            noon + 4.0 * hour_angle
        })
    }
}

/// The sun's declination (radians) and the equation of time (minutes) at
/// Julian day `jd`, from NOAA's solar calculator.
fn sun_position(jd: f64) -> (f64, f64) {
    let t = (jd - 2451545.0) / 36525.0;
    let mean_longitude = (280.46646 + t * (36000.76983 + t * 0.0003032)).rem_euclid(360.0);
    let mean_anomaly = 357.52911 + t * (35999.05029 - 0.0001537 * t);
    let m = mean_anomaly.to_radians();
    let eccentricity = 0.016708634 - t * (0.000042037 + 0.0000001267 * t);
    let center = m.sin() * (1.914602 - t * (0.004817 + 0.000014 * t))
        + (2.0 * m).sin() * (0.019993 - 0.000101 * t)
        + (3.0 * m).sin() * 0.000289;
    let omega = (125.04 - 1934.136 * t).to_radians();
    let apparent_longitude =
        (mean_longitude + center - 0.00569 - 0.00478 * omega.sin()).to_radians();
    let mean_obliquity =
        23.0 + (26.0 + (21.448 - t * (46.815 + t * (0.00059 - t * 0.001813))) / 60.0) / 60.0;
    let obliquity = (mean_obliquity + 0.00256 * omega.cos()).to_radians();
    let declination = (obliquity.sin() * apparent_longitude.sin()).asin();
    let y = (obliquity / 2.0).tan().powi(2);
    let l = mean_longitude.to_radians();
    let equation_of_time = 4.0
        * (y * (2.0 * l).sin() - 2.0 * eccentricity * m.sin()
            + 4.0 * eccentricity * y * m.sin() * (2.0 * l).cos()
            - 0.5 * y * y * (4.0 * l).sin()
            - 1.25 * eccentricity * eccentricity * (2.0 * m).sin())
        .to_degrees();
    (declination, equation_of_time)
}

/// Local clock time, rounded to the nearest minute.
fn clock(t: DateTime<Utc>, tz: &Tz) -> String {
    (t + Duration::seconds(30))
        .with_timezone(tz)
        .format("%H:%M")
        .to_string()
}

/// The zone's UTC offset at noon on `date`, as "+03:00".
fn utc_offset(date: NaiveDate, tz: &Tz) -> String {
    let noon = date.and_hms_opt(12, 0, 0).expect("noon exists");
    let seconds = tz.offset_from_utc_datetime(&noon).fix().local_minus_utc();
    let sign = if seconds < 0 { '-' } else { '+' };
    let seconds = seconds.abs();
    format!("{sign}{:02}:{:02}", seconds / 3600, seconds % 3600 / 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    fn minutes(hhmm: &str) -> i32 {
        let (h, m) = hhmm.split_once(':').unwrap();
        h.parse::<i32>().unwrap() * 60 + m.parse::<i32>().unwrap()
    }

    /// Compare with Hebcal's published times (its defaults, sea level), to
    /// within a minute.
    fn check(z: &Zmanim, expected: [(&str, &Option<String>); 7]) {
        for (want, got) in expected {
            let got = got.as_ref().expect("time exists");
            assert!(
                (minutes(got) - minutes(want)).abs() <= 1,
                "{}: expected {want}, got {got}",
                z.date
            );
        }
    }

    #[test]
    fn jerusalem_summer_matches_hebcal_with_daylight_time() {
        let z = ZmanimCalculator::new(GeoLocation::jerusalem())
            .calculate(day(2025, 7, 1))
            .unwrap();
        assert_eq!(z.utc_offset, "+03:00");
        check(
            &z,
            [
                ("04:10", &z.alot_hashachar),
                ("05:37", &z.sunrise),
                ("08:34", &z.sof_zman_shema_mga),
                ("09:10", &z.sof_zman_shema_gra),
                ("12:43", &z.chatzot),
                ("19:49", &z.sunset),
                ("20:31", &z.tzeit_hakochavim),
            ],
        );
    }

    #[test]
    fn jerusalem_winter_matches_hebcal() {
        let z = ZmanimCalculator::new(GeoLocation::jerusalem())
            .calculate(day(2025, 1, 1))
            .unwrap();
        assert_eq!(z.utc_offset, "+02:00");
        check(
            &z,
            [
                ("05:22", &z.alot_hashachar),
                ("06:39", &z.sunrise),
                ("08:35", &z.sof_zman_shema_mga),
                ("09:11", &z.sof_zman_shema_gra),
                ("11:43", &z.chatzot),
                ("16:47", &z.sunset),
                ("17:26", &z.tzeit_hakochavim),
            ],
        );
    }

    #[test]
    fn new_york_both_seasons_match_hebcal() {
        let calc = ZmanimCalculator::new(GeoLocation::new_york());
        let z = calc.calculate(day(2025, 7, 1)).unwrap();
        assert_eq!(z.utc_offset, "-04:00");
        check(
            &z,
            [
                ("03:41", &z.alot_hashachar),
                ("05:29", &z.sunrise),
                ("08:38", &z.sof_zman_shema_mga),
                ("09:14", &z.sof_zman_shema_gra),
                ("13:00", &z.chatzot),
                ("20:31", &z.sunset),
                ("21:21", &z.tzeit_hakochavim),
            ],
        );
        let z = calc.calculate(day(2025, 12, 15)).unwrap();
        assert_eq!(z.utc_offset, "-05:00");
        check(
            &z,
            [
                ("05:45", &z.alot_hashachar),
                ("07:13", &z.sunrise),
                ("08:56", &z.sof_zman_shema_mga),
                ("09:32", &z.sof_zman_shema_gra),
                ("11:51", &z.chatzot),
                ("16:30", &z.sunset),
                ("17:15", &z.tzeit_hakochavim),
            ],
        );
    }

    #[test]
    fn candle_lighting_is_before_sunset() {
        let calc = ZmanimCalculator::new(GeoLocation::jerusalem());
        let sunset = calc.calculate(day(2024, 6, 14)).unwrap().sunset.unwrap();
        for offset in [18, 40] {
            let candles = calc
                .candle_lighting(day(2024, 6, 14), offset)
                .unwrap()
                .unwrap();
            let diff = minutes(&sunset) - minutes(&candles);
            assert!(
                (offset as i32 - 1..=offset as i32 + 1).contains(&diff),
                "{offset}: {diff}"
            );
        }
    }

    #[test]
    fn elevation_brings_sunrise_earlier_and_sunset_later() {
        let sea = ZmanimCalculator::new(GeoLocation::jerusalem());
        let hill = ZmanimCalculator::new(GeoLocation::jerusalem().with_elevation(754.0));
        let d = day(2025, 3, 20);
        assert!(hill.sunrise(d).unwrap() < sea.sunrise(d).unwrap());
        assert!(hill.sunset(d).unwrap() > sea.sunset(d).unwrap());
    }

    #[test]
    fn no_sunset_in_the_arctic_summer() {
        let tromso = GeoLocation::new(69.65, 18.96)
            .unwrap()
            .with_timezone("Europe/Oslo")
            .unwrap();
        let z = ZmanimCalculator::new(tromso)
            .calculate(day(2025, 6, 21))
            .unwrap();
        assert!(z.sunrise.is_none() && z.sunset.is_none() && z.chatzot.is_none());
    }

    #[test]
    fn locations_are_validated() {
        assert!(GeoLocation::new(91.0, 0.0).is_err());
        assert!(GeoLocation::new(0.0, -181.0).is_err());
        assert!(GeoLocation::new(40.0, -74.0)
            .unwrap()
            .with_timezone("Mars/Olympus")
            .is_err());
        let loc = GeoLocation::new(40.0, -74.0)
            .unwrap()
            .with_timezone("America/New_York")
            .unwrap()
            .with_name("Test");
        assert_eq!(loc.location_name.as_deref(), Some("Test"));
    }
}
