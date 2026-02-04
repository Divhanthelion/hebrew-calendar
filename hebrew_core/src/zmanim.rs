//! Zmanim (Halachic Times) Calculation Module
//! 
//! Implements astronomical calculations for sunrise, sunset, and other halachic times.
//! Uses NOAA algorithms for solar position calculations.

use chrono::{Duration, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::CalendarError;

/// Geographic location for zmanim calculations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation_meters: f64,
    pub timezone_offset_minutes: i32,
    pub location_name: Option<String>,
}

impl GeoLocation {
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, CalendarError> {
        if latitude < -90.0 || latitude > 90.0 {
            return Err(CalendarError::InvalidLatitude(latitude));
        }
        if longitude < -180.0 || longitude > 180.0 {
            return Err(CalendarError::InvalidLongitude(longitude));
        }
        
        Ok(Self {
            latitude,
            longitude,
            elevation_meters: 0.0,
            timezone_offset_minutes: 0,
            location_name: None,
        })
    }
    
    pub fn with_elevation(mut self, elevation: f64) -> Self {
        self.elevation_meters = elevation;
        self
    }
    
    pub fn with_timezone(mut self, offset_minutes: i32) -> Self {
        self.timezone_offset_minutes = offset_minutes;
        self
    }
    
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.location_name = Some(name.into());
        self
    }
    
    /// Create a location for Jerusalem
    pub fn jerusalem() -> Self {
        Self {
            latitude: 31.7683,
            longitude: 35.2137,
            elevation_meters: 754.0,
            timezone_offset_minutes: 120, // UTC+2 (standard), +3 in summer
            location_name: Some("Jerusalem".to_string()),
        }
    }
    
    /// Create a location for New York
    pub fn new_york() -> Self {
        Self {
            latitude: 40.7128,
            longitude: -74.0060,
            elevation_meters: 10.0,
            timezone_offset_minutes: -300, // UTC-5 (EST)
            location_name: Some("New York".to_string()),
        }
    }
}

/// Zmanim for a specific day
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Zmanim {
    pub date: String,
    pub location: GeoLocation,
    pub alot_hashachar: Option<String>,    // Dawn (16.1° below horizon)
    pub misheyakir: Option<String>,        // Earliest tallit (11.5° below horizon)
    pub sunrise: Option<String>,           // Netz
    pub sof_zman_shema_mga: Option<String>, // Latest shema (Magen Avraham)
    pub sof_zman_shema_gra: Option<String>, // Latest shema (Gra)
    pub sof_zman_tefila_mga: Option<String>, // Latest shacharit (Magen Avraham)
    pub sof_zman_tefila_gra: Option<String>, // Latest shacharit (Gra)
    pub chatzot: Option<String>,           // Midday
    pub mincha_gedola: Option<String>,     // Earliest mincha
    pub mincha_ketana: Option<String>,     // Preferred mincha
    pub plag_hamincha: Option<String>,     // Plag
    pub sunset: Option<String>,            // Shkiah
    pub tzeit_hakochavim: Option<String>, // Nightfall (8.5° below horizon)
    pub tzeit_72_min: Option<String>,      // 72 minutes after sunset
}

/// Zmanim calculator
pub struct ZmanimCalculator {
    location: GeoLocation,
}

impl ZmanimCalculator {
    /// Create a new calculator for a location
    pub fn new(location: GeoLocation) -> Self {
        Self { location }
    }
    
    /// Calculate all zmanim for a date
    pub fn calculate(&self, date: NaiveDate) -> Result<Zmanim, CalendarError> {
        let times = self.calculate_times(date)?;
        
        Ok(Zmanim {
            date: date.to_string(),
            location: self.location.clone(),
            alot_hashachar: times.alot.map(|t| t.format("%H:%M").to_string()),
            misheyakir: times.misheyakir.map(|t| t.format("%H:%M").to_string()),
            sunrise: times.sunrise.map(|t| t.format("%H:%M").to_string()),
            sof_zman_shema_mga: times.sof_shema_mga.map(|t| t.format("%H:%M").to_string()),
            sof_zman_shema_gra: times.sof_shema_gra.map(|t| t.format("%H:%M").to_string()),
            sof_zman_tefila_mga: times.sof_tefila_mga.map(|t| t.format("%H:%M").to_string()),
            sof_zman_tefila_gra: times.sof_tefila_gra.map(|t| t.format("%H:%M").to_string()),
            chatzot: times.chatzot.map(|t| t.format("%H:%M").to_string()),
            mincha_gedola: times.mincha_gedola.map(|t| t.format("%H:%M").to_string()),
            mincha_ketana: times.mincha_ketana.map(|t| t.format("%H:%M").to_string()),
            plag_hamincha: times.plag.map(|t| t.format("%H:%M").to_string()),
            sunset: times.sunset.map(|t| t.format("%H:%M").to_string()),
            tzeit_hakochavim: times.tzeit.map(|t| t.format("%H:%M").to_string()),
            tzeit_72_min: times.tzeit_72.map(|t| t.format("%H:%M").to_string()),
        })
    }
    
    /// Calculate candle lighting time
    pub fn candle_lighting(
        &self,
        zmanim: &Zmanim,
        offset_minutes: i64,
    ) -> Result<Option<String>, CalendarError> {
        let sunset_str = match &zmanim.sunset {
            Some(s) => s,
            None => return Ok(None),
        };
        
        let sunset_time = NaiveTime::parse_from_str(sunset_str, "%H:%M")
            .map_err(|e| CalendarError::CalculationError(e.to_string()))?;
        
        let candle_time = sunset_time - Duration::minutes(offset_minutes);
        
        Ok(Some(candle_time.format("%H:%M").to_string()))
    }
    
    /// Calculate specific time for an elevation angle
    pub fn time_at_elevation(
        &self,
        date: NaiveDate,
        elevation: f64,
        rising: bool,
    ) -> Result<Option<NaiveTime>, CalendarError> {
        let rd = crate::calendar::DateConverter::gregorian_to_rd(date);
        let jd = crate::calendar::DateConverter::rd_to_julian_day(rd) as f64;

        // Calculate solar position
        let time = self.calculate_solar_time(jd, elevation, rising);
        
        Ok(time)
    }
    
    /// Internal: Calculate all times for a date
    fn calculate_times(&self, date: NaiveDate) -> Result<CalculatedTimes, CalendarError> {
        let rd = crate::calendar::DateConverter::gregorian_to_rd(date);
        let jd = crate::calendar::DateConverter::rd_to_julian_day(rd) as f64;
        
        // Calculate sunrise and sunset (0.833° below horizon for refraction)
        let sunrise = self.calculate_solar_time(jd, -0.833, true);
        let sunset = self.calculate_solar_time(jd, -0.833, false);
        
        // Dawn (16.1° below horizon - Alot Hashachar)
        let alot = self.calculate_solar_time(jd, -16.1, true);
        
        // Misheyakir (11.5° below horizon)
        let misheyakir = self.calculate_solar_time(jd, -11.5, true);
        
        // Tzeit (8.5° below horizon)
        let tzeit = self.calculate_solar_time(jd, -8.5, false);
        
        // Calculate derived times
        let (sof_shema_gra, sof_shema_mga, sof_tefila_gra, sof_tefila_mga, 
             chatzot, mincha_gedola, mincha_ketana, plag, tzeit_72) = 
            if let (Some(sr), Some(ss)) = (sunrise, sunset) {
                let day_length = ss.signed_duration_since(sr);
                let _hours = day_length.num_minutes() as f64 / 60.0;
                
                // Shaot zmaniyot (proportional hours)
                let shaah = day_length / 12;
                
                // Sof zman shema (3 hours)
                let sof_shema_gra = sr + shaah * 3;
                // Magen Avraham uses alot to tzeit (72 min)
                let alot_72 = sr - Duration::minutes(72);
                let tzeit_72_calc = ss + Duration::minutes(72);
                let day_length_mga = tzeit_72_calc.signed_duration_since(alot_72);
                let shaah_mga = day_length_mga / 12;
                let sof_shema_mga = alot_72 + shaah_mga * 3;
                
                // Sof zman tefila (4 hours)
                let sof_tefila_gra = sr + shaah * 4;
                let sof_tefila_mga = alot_72 + shaah_mga * 4;
                
                // Chatzot (midday)
                let chatzot_time = sr + day_length / 2;
                
                // Mincha gedola (6.5 hours)
                let mincha_g = sr + shaah * 6 + shaah / 2;
                
                // Mincha ketana (9.5 hours)
                let mincha_k = sr + shaah * 9 + shaah / 2;
                
                // Plag hamincha (10.75 hours)
                let plag_time = sr + shaah * 10 + (shaah * 3) / 4;
                
                (Some(sof_shema_gra), Some(sof_shema_mga), 
                 Some(sof_tefila_gra), Some(sof_tefila_mga),
                 Some(chatzot_time), Some(mincha_g), Some(mincha_k), 
                 Some(plag_time), Some(tzeit_72_calc))
            } else {
                (None, None, None, None, None, None, None, None, None)
            };
        
        Ok(CalculatedTimes {
            alot,
            misheyakir,
            sunrise,
            sof_shema_mga,
            sof_shema_gra,
            sof_tefila_mga,
            sof_tefila_gra,
            chatzot,
            mincha_gedola,
            mincha_ketana,
            plag,
            sunset,
            tzeit,
            tzeit_72,
        })
    }
    
    /// Calculate solar time for a specific elevation angle
    /// Uses standard NOAA solar calculator algorithm
    fn calculate_solar_time(&self, jd: f64, elevation: f64, rising: bool) -> Option<NaiveTime> {
        let tz = self.location.timezone_offset_minutes as f64 / 60.0;
        let lat = self.location.latitude;
        let lng = self.location.longitude;

        // Julian century from J2000.0
        let jc = (jd - 2451545.0) / 36525.0;

        // Geometric mean longitude of the sun (degrees)
        let gm_long = (280.46646 + jc * (36000.76983 + jc * 0.0003032)) % 360.0;

        // Geometric mean anomaly of the sun (degrees)
        let gm_anom = 357.52911 + jc * (35999.05029 - 0.0001537 * jc);
        let gm_anom_rad = gm_anom.to_radians();

        // Eccentricity of Earth's orbit
        let ecc = 0.016708634 - jc * (0.000042037 + 0.0000001267 * jc);

        // Sun equation of center (degrees)
        let sun_eq_ctr = gm_anom_rad.sin() * (1.914602 - jc * (0.004817 + 0.000014 * jc))
            + (2.0 * gm_anom_rad).sin() * (0.019993 - 0.000101 * jc)
            + (3.0 * gm_anom_rad).sin() * 0.000289;

        // Sun true longitude (degrees)
        let sun_true_long = gm_long + sun_eq_ctr;

        // Sun apparent longitude (degrees)
        let omega = 125.04 - 1934.136 * jc;
        let sun_app_long = sun_true_long - 0.00569 - 0.00478 * omega.to_radians().sin();

        // Mean obliquity of the ecliptic (degrees)
        let mean_obliq = 23.0 + (26.0 + (21.448 - jc * (46.815 + jc * (0.00059 - jc * 0.001813))) / 60.0) / 60.0;

        // Obliquity correction (degrees)
        let obliq_corr = mean_obliq + 0.00256 * omega.to_radians().cos();
        let obliq_corr_rad = obliq_corr.to_radians();

        // Sun declination (radians)
        let sun_declin = (obliq_corr_rad.sin() * sun_app_long.to_radians().sin()).asin();

        // Equation of time (minutes)
        let y = (obliq_corr_rad / 2.0).tan().powi(2);
        let gm_long_rad = gm_long.to_radians();
        let eq_time = 4.0 * (
            y * (2.0 * gm_long_rad).sin()
            - 2.0 * ecc * gm_anom_rad.sin()
            + 4.0 * ecc * y * gm_anom_rad.sin() * (2.0 * gm_long_rad).cos()
            - 0.5 * y * y * (4.0 * gm_long_rad).sin()
            - 1.25 * ecc * ecc * (2.0 * gm_anom_rad).sin()
        ).to_degrees();

        // Solar noon (minutes from midnight, local time)
        let solar_noon_min = 720.0 - 4.0 * lng - eq_time + tz * 60.0;

        // Hour angle for the desired elevation
        let lat_rad = lat.to_radians();
        let elevation_rad = elevation.to_radians();
        let cos_hour = (elevation_rad.sin() - lat_rad.sin() * sun_declin.sin())
            / (lat_rad.cos() * sun_declin.cos());

        // Check if sun reaches this elevation at this latitude
        if cos_hour < -1.0 || cos_hour > 1.0 {
            return None;
        }

        let hour_angle_deg = cos_hour.acos().to_degrees();

        // Time of event (minutes from midnight)
        let event_minutes = if rising {
            solar_noon_min - hour_angle_deg * 4.0
        } else {
            solar_noon_min + hour_angle_deg * 4.0
        };

        // Convert to hours and minutes, handling wrap-around
        let total_minutes = event_minutes.round() as i64;
        let total_minutes = total_minutes.rem_euclid(1440);
        let hours = (total_minutes / 60) as u32;
        let minutes = (total_minutes % 60) as u32;

        NaiveTime::from_hms_opt(hours, minutes, 0)
    }
}

/// Internal structure for calculated times
struct CalculatedTimes {
    alot: Option<NaiveTime>,
    misheyakir: Option<NaiveTime>,
    sunrise: Option<NaiveTime>,
    sof_shema_mga: Option<NaiveTime>,
    sof_shema_gra: Option<NaiveTime>,
    sof_tefila_mga: Option<NaiveTime>,
    sof_tefila_gra: Option<NaiveTime>,
    chatzot: Option<NaiveTime>,
    mincha_gedola: Option<NaiveTime>,
    mincha_ketana: Option<NaiveTime>,
    plag: Option<NaiveTime>,
    sunset: Option<NaiveTime>,
    tzeit: Option<NaiveTime>,
    tzeit_72: Option<NaiveTime>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    
    #[test]
    fn test_zmanim_jerusalem() {
        let loc = GeoLocation::jerusalem();
        let calc = ZmanimCalculator::new(loc);
        
        // Test a specific date
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(); // Summer solstice nearby
        let zmanim = calc.calculate(date).unwrap();
        
        println!("Jerusalem Zmanim for {}:", date);
        println!("  Sunrise: {:?}", zmanim.sunrise);
        println!("  Sunset: {:?}", zmanim.sunset);
        println!("  Alot: {:?}", zmanim.alot_hashachar);
        println!("  Tzeit: {:?}", zmanim.tzeit_hakochavim);
        
        assert!(zmanim.sunrise.is_some());
        assert!(zmanim.sunset.is_some());
    }
    
    #[test]
    fn test_candle_lighting() {
        let loc = GeoLocation::new_york();
        let calc = ZmanimCalculator::new(loc);
        
        let date = NaiveDate::from_ymd_opt(2024, 6, 14).unwrap(); // Friday
        let zmanim = calc.calculate(date).unwrap();
        let candle = calc.candle_lighting(&zmanim, 18).unwrap();
        
        println!("NYC Candle lighting: {:?}", candle);
        assert!(candle.is_some());
    }
    
    #[test]
    fn test_geolocation_validation() {
        assert!(GeoLocation::new(91.0, 0.0).is_err());
        assert!(GeoLocation::new(-91.0, 0.0).is_err());
        assert!(GeoLocation::new(0.0, 181.0).is_err());
        assert!(GeoLocation::new(0.0, -181.0).is_err());
        assert!(GeoLocation::new(40.7128, -74.0060).is_ok());
    }

    #[test]
    fn test_jerusalem_sunrise_summer_solstice() {
        let loc = GeoLocation::jerusalem();
        let calc = ZmanimCalculator::new(loc);
        let date = NaiveDate::from_ymd_opt(2024, 6, 21).unwrap();
        let zmanim = calc.calculate(date).unwrap();
        let sunrise = zmanim.sunrise.as_ref().expect("sunrise should exist");
        let time = NaiveTime::parse_from_str(sunrise, "%H:%M").unwrap();
        // Jerusalem sunrise ~05:29 IST (UTC+2) on summer solstice
        // Allow wide tolerance due to timezone/DST differences
        let earliest = NaiveTime::from_hms_opt(3, 0, 0).unwrap();
        let latest = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
        assert!(time >= earliest && time <= latest,
            "Jerusalem sunrise {} should be between 03:00 and 08:00", sunrise);
    }

    #[test]
    fn test_jerusalem_sunset_summer_solstice() {
        let loc = GeoLocation::jerusalem();
        let calc = ZmanimCalculator::new(loc);
        let date = NaiveDate::from_ymd_opt(2024, 6, 21).unwrap();
        let zmanim = calc.calculate(date).unwrap();
        let sunset = zmanim.sunset.as_ref().expect("sunset should exist");
        let time = NaiveTime::parse_from_str(sunset, "%H:%M").unwrap();
        let earliest = NaiveTime::from_hms_opt(15, 0, 0).unwrap();
        let latest = NaiveTime::from_hms_opt(21, 0, 0).unwrap();
        assert!(time >= earliest && time <= latest,
            "Jerusalem sunset {} should be between 15:00 and 21:00", sunset);
    }

    #[test]
    fn test_new_york_zmanim_equinox() {
        let loc = GeoLocation::new_york();
        let calc = ZmanimCalculator::new(loc);
        let date = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap();
        let zmanim = calc.calculate(date).unwrap();
        assert!(zmanim.sunrise.is_some(), "NYC equinox should have sunrise");
        assert!(zmanim.sunset.is_some(), "NYC equinox should have sunset");
    }

    #[test]
    fn test_candle_lighting_18_min() {
        let loc = GeoLocation::jerusalem();
        let calc = ZmanimCalculator::new(loc);
        let date = NaiveDate::from_ymd_opt(2024, 6, 14).unwrap();
        let zmanim = calc.calculate(date).unwrap();
        let candle = calc.candle_lighting(&zmanim, 18).unwrap();
        assert!(candle.is_some());
        // Candle should be 18 min before sunset
        let sunset = NaiveTime::parse_from_str(zmanim.sunset.as_ref().unwrap(), "%H:%M").unwrap();
        let candle_time = NaiveTime::parse_from_str(candle.as_ref().unwrap(), "%H:%M").unwrap();
        let diff = sunset.signed_duration_since(candle_time).num_minutes();
        assert_eq!(diff, 18, "Candle lighting should be 18 minutes before sunset");
    }

    #[test]
    fn test_candle_lighting_40_min() {
        let loc = GeoLocation::jerusalem();
        let calc = ZmanimCalculator::new(loc);
        let date = NaiveDate::from_ymd_opt(2024, 6, 14).unwrap();
        let zmanim = calc.calculate(date).unwrap();
        let candle = calc.candle_lighting(&zmanim, 40).unwrap();
        assert!(candle.is_some());
        let sunset = NaiveTime::parse_from_str(zmanim.sunset.as_ref().unwrap(), "%H:%M").unwrap();
        let candle_time = NaiveTime::parse_from_str(candle.as_ref().unwrap(), "%H:%M").unwrap();
        let diff = sunset.signed_duration_since(candle_time).num_minutes();
        assert_eq!(diff, 40, "Candle lighting should be 40 minutes before sunset");
    }

    #[test]
    fn test_candle_lighting_no_sunset() {
        let zmanim = Zmanim {
            date: "2024-06-21".to_string(),
            location: GeoLocation::jerusalem(),
            alot_hashachar: None,
            misheyakir: None,
            sunrise: None,
            sof_zman_shema_mga: None,
            sof_zman_shema_gra: None,
            sof_zman_tefila_mga: None,
            sof_zman_tefila_gra: None,
            chatzot: None,
            mincha_gedola: None,
            mincha_ketana: None,
            plag_hamincha: None,
            sunset: None,
            tzeit_hakochavim: None,
            tzeit_72_min: None,
        };
        let loc = GeoLocation::jerusalem();
        let calc = ZmanimCalculator::new(loc);
        let candle = calc.candle_lighting(&zmanim, 18).unwrap();
        assert!(candle.is_none(), "No sunset means no candle lighting");
    }

    #[test]
    fn test_geolocation_builders() {
        let loc = GeoLocation::new(40.0, -74.0).unwrap()
            .with_elevation(100.0)
            .with_timezone(-300)
            .with_name("Test City");
        assert_eq!(loc.elevation_meters, 100.0);
        assert_eq!(loc.timezone_offset_minutes, -300);
        assert_eq!(loc.location_name.as_deref(), Some("Test City"));
    }

    #[test]
    fn test_geolocation_jerusalem_preset() {
        let loc = GeoLocation::jerusalem();
        assert!((loc.latitude - 31.7683).abs() < 0.001);
        assert!((loc.longitude - 35.2137).abs() < 0.001);
        assert_eq!(loc.elevation_meters, 754.0);
        assert_eq!(loc.timezone_offset_minutes, 120);
        assert_eq!(loc.location_name.as_deref(), Some("Jerusalem"));
    }

    #[test]
    fn test_geolocation_new_york_preset() {
        let loc = GeoLocation::new_york();
        assert!((loc.latitude - 40.7128).abs() < 0.001);
        assert!((loc.longitude - (-74.0060)).abs() < 0.001);
        assert_eq!(loc.elevation_meters, 10.0);
        assert_eq!(loc.timezone_offset_minutes, -300);
        assert_eq!(loc.location_name.as_deref(), Some("New York"));
    }

    #[test]
    fn test_zmanim_temporal_ordering() {
        let loc = GeoLocation::jerusalem();
        let calc = ZmanimCalculator::new(loc);
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let zmanim = calc.calculate(date).unwrap();

        let parse = |s: &Option<String>| -> NaiveTime {
            NaiveTime::parse_from_str(s.as_ref().unwrap(), "%H:%M").unwrap()
        };

        let alot = parse(&zmanim.alot_hashachar);
        let sunrise = parse(&zmanim.sunrise);
        let chatzot = parse(&zmanim.chatzot);
        let sunset = parse(&zmanim.sunset);
        let tzeit = parse(&zmanim.tzeit_hakochavim);

        assert!(alot < sunrise, "alot {} should be before sunrise {}", alot, sunrise);
        assert!(sunrise < chatzot, "sunrise {} should be before chatzot {}", sunrise, chatzot);
        assert!(chatzot < sunset, "chatzot {} should be before sunset {}", chatzot, sunset);
        assert!(sunset < tzeit, "sunset {} should be before tzeit {}", sunset, tzeit);
    }
}
