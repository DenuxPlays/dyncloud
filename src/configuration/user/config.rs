use crate::configuration::user::error::ConfigError;
use crate::configuration::user::records::RecordsGroup;
use crate::configuration::validation::cron::validate_cron_expression;
use serde::Deserialize;
use std::path::PathBuf;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct Config {
    #[validate(custom(function = "validate_cron_expression"))]
    pub(crate) cron: String,
    #[serde(rename = "domains")]
    #[validate(nested)]
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
    use crate::configuration::user::providers::Cloudflare;
    use crate::configuration::user::providers::Providers;
    use crate::configuration::user::records::{BasicRecord, CloudflareRecord, DnsType, RecordsGroup};
    use crate::configuration::user::resolver::Resolver;
    use std::path::PathBuf;
    use validator::Validate;

    impl Config {
        fn create_test_cloudflare_record(dns_types: Vec<DnsType>) -> CloudflareRecord {
            CloudflareRecord {
                basic_record: BasicRecord {
                    name: "test.example.com".to_string(),
                    ttl: 300,
                    dns_type: dns_types,
                },
                proxied: false,
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
        const DIST_PATH: &str = "./config.dist.toml";
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

    #[test]
    fn test_valid_config() {
        let config = Config {
            cron: "* * * * *".to_string(),
            records: vec![
                RecordsGroup {
                    providers: Providers {
                        cloudflare: Some(Cloudflare {
                            auth_token: "My auth token".to_string(),
                            zone_id: "My Zone id".to_string(),
                        }),
                    },
                    cloudflare: vec![CloudflareRecord {
                        basic_record: BasicRecord {
                            name: "test.example.test".to_string(),
                            ttl: 60,
                            dns_type: vec![DnsType::A, DnsType::Aaaa],
                        },
                        proxied: false,
                    }],
                    resolver: Resolver::Ipfiy,
                },
                RecordsGroup {
                    providers: Providers {
                        cloudflare: None,
                    },
                    cloudflare: vec![],
                    resolver: Resolver::Ipfiy,
                },
            ],
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_config() {
        let config = Config {
            cron: "".to_string(), // Invalid cron
            records: vec![RecordsGroup {
                providers: Providers {
                    cloudflare: None, // Cloudflare Record given but no provider
                },
                cloudflare: vec![CloudflareRecord {
                    basic_record: BasicRecord {
                        name: "".to_string(), // Empty record name
                        ttl: 60,
                        dns_type: vec![], // No DnsType specified
                    },
                    proxied: false,
                }],
                resolver: Resolver::Ipfiy,
            }],
        };

        assert!(config.validate().is_err());
    }
}
