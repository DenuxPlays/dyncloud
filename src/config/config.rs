use std::fs;

use serde::Deserialize;

use crate::system::domain::Domain;

pub const CONFIG_FILE: &str = "config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub domains: Vec<Domain>,
}

impl Config {
    pub fn init() -> Config {
        let toml_str = Config::read_config_file();
        match toml::from_str(&toml_str) {
            Ok(config) => config,
            Err(error) => panic!("Unable to parse config file, {}", error),
        }
    }

    fn read_config_file() -> String {
        match fs::read_to_string(CONFIG_FILE) {
            Ok(toml_str) => toml_str,
            Err(error) => panic!("Unable to read config file, {}", error),
        }
    }
}
