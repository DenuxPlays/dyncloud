use crate::configuration::user::providers::Providers;
use crate::configuration::user::resolver::Resolver;
use crate::configuration::validation::records_group::validate_record_groups_schema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[cfg_attr(test, derive(serde::Serialize))]
#[validate(schema(function = "validate_record_groups_schema"))]
pub(crate) struct RecordsGroup {
    #[serde(rename = "config")]
    #[validate(nested)]
    pub(crate) providers: Providers,
    #[serde(default)]
    #[validate(nested)]
    pub(crate) cloudflare: Vec<CloudflareRecord>,
    #[serde(default)]
    pub(crate) resolver: Resolver,
}

#[derive(Debug, Deserialize, Validate)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct BasicRecord {
    #[validate(length(min = 1))]
    pub(crate) name: String,
    pub(crate) ttl: u32,
    #[serde(rename = "type")]
    #[validate(length(min = 1))]
    pub(crate) dns_type: Vec<DnsType>,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Deserialize, Serialize)]
pub(crate) enum DnsType {
    A,
    #[serde(rename = "AAAA")]
    Aaaa,
}

impl Display for DnsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsType::A => write!(f, "A"),
            DnsType::Aaaa => write!(f, "AAAA"),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct CloudflareRecord {
    #[serde(flatten)]
    #[validate(nested)]
    pub(crate) basic_record: BasicRecord,
    #[serde(default = "default_proxied")]
    pub(crate) proxied: bool,
}

fn default_proxied() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::configuration::user::records::{BasicRecord, DnsType};
    use validator::Validate;

    #[test]
    fn test_valid_basic_record() {
        let record = BasicRecord {
            name: "My Record".to_string(),
            ttl: 120,
            dns_type: vec![DnsType::A],
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn test_invalid_record() {
        let record = BasicRecord {
            name: "".to_string(),
            ttl: 0,
            dns_type: vec![],
        };

        let rs = record.validate();
        assert!(rs.is_err());
        assert_eq!(rs.err().unwrap().0.len(), 2);
    }
}
