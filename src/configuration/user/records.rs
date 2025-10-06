use crate::configuration::user::providers::Providers;
use crate::configuration::user::resolver::Resolver;
use crate::configuration::validation::records_group::validate_record_groups_schema;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[cfg_attr(test, derive(serde::Serialize))]
#[validate(schema(function = "validate_record_groups_schema"))]
pub(crate) struct RecordsGroup {
    #[serde(rename = "config")]
    pub(crate) providers: Providers,
    #[serde(default)]
    pub(crate) cloudflare: Vec<CloudflareRecord>,
    #[serde(default)]
    pub(crate) resolver: Resolver,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct BasicRecord {
    pub(crate) name: String,
    pub(crate) ttl: u32,
    #[serde(rename = "type")]
    pub(crate) dns_type: Vec<DnsType>,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) enum DnsType {
    A,
    #[serde(rename = "AAAA")]
    Aaaa,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct CloudflareRecord {
    #[serde(flatten)]
    pub(crate) basic_record: BasicRecord,
    #[serde(default = "default_proxied")]
    pub(crate) proxied: bool,
}

fn default_proxied() -> bool {
    false
}
