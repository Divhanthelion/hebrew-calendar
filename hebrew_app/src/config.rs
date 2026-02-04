//! Configuration Module
//! 
//! Handles loading and saving application configuration.

use hebrew_core::zmanim::GeoLocation;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Default location for zmanim calculations
    pub default_location: GeoLocation,
    
    /// Candle lighting offset in minutes (default: 18)
    pub candle_lighting_offset_minutes: i64,
    
    /// Whether to use Ashkenazi or Sefardi customs (affects some zmanim)
    pub ashkenazi_customs: bool,
    
    /// API server settings
    pub api_settings: ApiSettings,
}

/// API server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSettings {
    /// Default port for API server
    pub port: u16,
    
    /// Host to bind to
    pub host: String,
    
    /// Enable CORS
    pub enable_cors: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_location: GeoLocation::jerusalem(),
            candle_lighting_offset_minutes: 18,
            ashkenazi_customs: true,
            api_settings: ApiSettings::default(),
        }
    }
}

impl Default for ApiSettings {
    fn default() -> Self {
        Self {
            port: 3000,
            host: "127.0.0.1".to_string(),
            enable_cors: true,
        }
    }
}

impl AppConfig {
    /// Load configuration from file or create default
    pub fn load() -> anyhow::Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            let config: AppConfig = serde_json::from_str(&contents)?;
            Ok(config)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }
    
    /// Save configuration to file
    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::config_path()?;
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, contents)?;
        
        Ok(())
    }
    
    /// Get the configuration file path
    pub fn config_path() -> anyhow::Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        
        Ok(config_dir.join("hebrew-calendar").join("config.json"))
    }
    
    /// Update the default location
    pub fn set_location(&mut self, location: GeoLocation) {
        self.default_location = location;
    }
    
    /// Update candle lighting offset
    pub fn set_candle_offset(&mut self, minutes: i64) {
        self.candle_lighting_offset_minutes = minutes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.candle_lighting_offset_minutes, 18);
        assert_eq!(config.api_settings.port, 3000);
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let config = AppConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.candle_lighting_offset_minutes, config.candle_lighting_offset_minutes);
        assert_eq!(deserialized.api_settings.port, config.api_settings.port);
        assert_eq!(deserialized.api_settings.host, config.api_settings.host);
        assert_eq!(deserialized.ashkenazi_customs, config.ashkenazi_customs);
    }

    #[test]
    fn test_set_location() {
        let mut config = AppConfig::default();
        let ny = hebrew_core::zmanim::GeoLocation::new_york();
        config.set_location(ny.clone());
        assert!((config.default_location.latitude - 40.7128).abs() < 0.001);
        assert!((config.default_location.longitude - (-74.0060)).abs() < 0.001);
    }

    #[test]
    fn test_set_candle_offset() {
        let mut config = AppConfig::default();
        config.set_candle_offset(40);
        assert_eq!(config.candle_lighting_offset_minutes, 40);
    }

    #[test]
    fn test_default_api_settings() {
        let settings = ApiSettings::default();
        assert_eq!(settings.port, 3000);
        assert_eq!(settings.host, "127.0.0.1");
        assert!(settings.enable_cors);
    }
}
