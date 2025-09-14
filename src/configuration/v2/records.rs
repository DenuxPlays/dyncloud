use crate::configuration::v2::providers::Providers;
use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct RecordsGroup {
    #[serde(rename = "config")]
    pub(crate) providers: Providers,
    #[serde(default)]
    pub(crate) cloudflare: Vec<CloudflareRecord>,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(super) struct BasicRecord {
    pub(crate) name: String,
    pub(crate) ttl: u32,
    #[serde(rename = "type")]
    pub(crate) dns_type: Vec<DnsType>,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(super) enum DnsType {
    A,
    #[serde(rename = "AAAA")]
    Aaaa,
}

#[derive(Deserialize)]
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
