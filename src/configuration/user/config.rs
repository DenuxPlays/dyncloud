use crate::configuration::user::error::ConfigError;
use crate::configuration::user::records::RecordsGroup;
use serde::Deserialize;
use std::path::PathBuf;

// TODO: add validation
#[derive(Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct Config {
    pub(crate) cron: String,
    #[serde(rename = "domains")]
    pub(crate) records: Vec<RecordsGroup>,
}

impl Config {
    pub(crate) fn from_file(file: PathBuf) -> Result<Self, ConfigError> {
        if !file.exists() {
            return Err(ConfigError::FileNotFound(file));
        }

        let toml_str = std::fs::read_to_string(file)?;

        let config: Config = toml::from_str(&toml_str)?;

        Ok(config)
    }

    pub(crate) fn get_total_number_of_records(&self) -> u32 {
        self.records
            .iter()
            .flat_map(|record| &record.cloudflare)
            .map(|record| record.basic_record.dns_type.len() as u32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::user::config::Config;
    use crate::configuration::user::providers::Providers;
    use crate::configuration::user::records::{BasicRecord, CloudflareRecord, DnsType, RecordsGroup};
    use std::path::PathBuf;

    impl Config {
        fn create_test_cloudflare_record(dns_types: Vec<DnsType>) -> CloudflareRecord {
            CloudflareRecord {
                basic_record: BasicRecord {
                    name: "test.example.com".to_string(),
                    ttl: 300,
                    dns_type: dns_types,
                },
                proxied: false,
                id: None,
            }
        }

        fn create_test_records_group(cloudflare_records: Vec<CloudflareRecord>) -> RecordsGroup {
            RecordsGroup {
                providers: Providers {
                    cloudflare: None,
                },
                cloudflare: cloudflare_records,
                resolver: Default::default(),
            }
        }
    }

    #[test]
    fn can_parse_dist_config() {
        const DIST_PATH: &str = "./config.toml.dist";
        let path = PathBuf::from(DIST_PATH);

        let config = Config::from_file(path);

        assert!(config.is_ok());
    }

    #[test]
    fn test_file_not_found() {
        let config = Config::from_file(PathBuf::from("non_existing_file"));

        assert!(matches!(config, Err(crate::configuration::user::error::ConfigError::FileNotFound(_))));
    }

    #[test]
    fn test_get_total_number_of_records_empty() {
        let config = Config {
            cron: "0 0 * * *".to_string(),
            records: vec![],
        };

        assert_eq!(config.get_total_number_of_records(), 0);
    }

    #[test]
    fn test_get_total_number_of_records_single_record() {
        let config = Config {
            cron: "0 0 * * *".to_string(),
            records: vec![Config::create_test_records_group(vec![Config::create_test_cloudflare_record(vec![
                DnsType::A,
                DnsType::Aaaa,
            ])])],
        };

        assert_eq!(config.get_total_number_of_records(), 2);
    }

    #[test]
    fn test_get_total_number_of_records_multiple_groups() {
        let config = Config {
            cron: "0 0 * * *".to_string(),
            records: vec![
                Config::create_test_records_group(vec![Config::create_test_cloudflare_record(vec![DnsType::A])]),
                Config::create_test_records_group(vec![Config::create_test_cloudflare_record(vec![DnsType::Aaaa])]),
            ],
        };

        assert_eq!(config.get_total_number_of_records(), 2);
    }

    #[test]
    fn test_get_total_number_of_records_empty_dns_types() {
        let config = Config {
            cron: "0 0 * * *".to_string(),
            records: vec![Config::create_test_records_group(vec![Config::create_test_cloudflare_record(vec![])])],
        };

        assert_eq!(config.get_total_number_of_records(), 0);
    }
}
