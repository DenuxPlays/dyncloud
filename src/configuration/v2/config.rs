use crate::configuration::v2::error::ConfigError;
use crate::configuration::v2::records::RecordsGroup;
use serde::Deserialize;
use std::path::Path;

const CONFIG_FILE: &str = "config.toml";

#[derive(Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct Config {
    pub(crate) cron: String,
    #[serde(rename = "domains")]
    pub(crate) records: Vec<RecordsGroup>,
}

impl Config {
    pub(crate) fn init() -> Result<Self, ConfigError> {
        Self::from_file(CONFIG_FILE)
    }

    fn from_file(file_name: &str) -> Result<Self, ConfigError> {
        let path = Path::new(file_name);
        if !path.exists() {
            return Err(ConfigError::FileNotFound(file_name.to_string()));
        }

        let toml_str = std::fs::read_to_string(file_name)?;

        let config: Config = toml::from_str(&toml_str)?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::v2::config::Config;

    #[test]
    fn can_parse_dist_config() {
        const DIST_PATH: &str = "./config.toml.dist";

        let config = Config::from_file(DIST_PATH);

        assert!(config.is_ok());
    }

    #[test]
    fn test_file_not_found() {
        let config = Config::from_file("non_existent_config.toml");

        assert!(matches!(config, Err(crate::configuration::v2::error::ConfigError::FileNotFound(_))));
    }
}
