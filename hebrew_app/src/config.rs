//! Defaults for requests that do not say where they are for, kept in the
//! user's config directory (`hebrew-calendar/config.json`).

use hebrew_core::{GeoLocation, Observance};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    /// Where to calculate zmanim for when a request gives no location.
    pub location: GeoLocation,
    pub observance: Observance,
    /// Minutes before sunset to light candles.
    pub candle_lighting_minutes: i64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            location: GeoLocation::jerusalem(),
            observance: Observance::Israel,
            candle_lighting_minutes: 40,
        }
    }
}

impl AppConfig {
    /// The saved configuration, or the default when there is none. A file
    /// from an older version that no longer parses is replaced.
    pub fn load() -> Self {
        let Some(path) = Self::path() else {
            return Self::default();
        };
        match fs::read_to_string(&path).map(|s| serde_json::from_str::<Self>(&s)) {
            Ok(Ok(config)) => config,
            _ => {
                let config = Self::default();
                // Best effort: a read-only config directory is not an error.
                let _ = config.save();
                config
            }
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::path().ok_or_else(|| std::io::Error::other("no config directory"))?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, serde_json::to_string_pretty(self)?)
    }

    pub fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("hebrew-calendar").join("config.json"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_jerusalem() {
        let config = AppConfig::default();
        assert_eq!(config.location.timezone, "Asia/Jerusalem");
        assert_eq!(config.candle_lighting_minutes, 40);
    }

    #[test]
    fn an_old_or_partial_file_still_loads() {
        let partial: AppConfig =
            serde_json::from_str(r#"{"candle_lighting_minutes": 18}"#).unwrap();
        assert_eq!(partial.candle_lighting_minutes, 18);
        assert_eq!(partial.observance, Observance::Israel);
    }
}
