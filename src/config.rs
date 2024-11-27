use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;

use log::warn;

#[derive(Serialize, Deserialize)]
pub struct Config {
    device_name: String,
    mqtt_host: String,
    mqtt_port: u16,
    mqtt_channel: String,
    wheel_length: Option<i32>,
    strip_length: Option<i32>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Config {
            device_name: "DDD_leds".to_string(),
            mqtt_host: "localhost".to_string(),
            mqtt_port: 1883,
            mqtt_channel: "home/leds".to_string(),
            wheel_length: Some(78),
            strip_length: Some(96)
        }
    }
}

impl Config {
    /// Creates a new config from the given file path. If the file does
    pub fn from_file<T>(config_path: T) -> Self
    where
        T: AsRef<Path>
    {
        match fs::read_to_string(config_path) {
            Ok(config_str) => toml::from_str(config_str.as_str())
                .unwrap_or_else(|e| {
                    warn!("Error in configuration file: {}", e.to_string());
                    Config::default()
                }),
            Err(e) => {
                warn!("Unable to read config file: {}", e.to_string());
                Config::default()
            }
        }
    }

    pub fn get_wheel_length(&self) -> i32 {
        match self.wheel_length {
            Some(wl) => wl,
            None => 78
        }
    }

    pub fn get_strip_length(&self) -> i32 {
        match self.strip_length {
            Some(sl) => sl,
            None => 96
        }
    }

    pub fn dump(&self) {
        println!("{}", toml::to_string(self)
            .unwrap_or(
                "Error while deserializing the configuration".to_string()));
    }
}
