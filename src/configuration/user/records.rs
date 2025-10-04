use crate::configuration::user::providers::Providers;
use crate::configuration::user::resolver::Resolver;
use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct RecordsGroup {
    #[serde(rename = "config")]
    pub(crate) providers: Providers,
    #[serde(default)]
    pub(crate) cloudflare: Vec<CloudflareRecord>,
    #[serde(default)]
    pub(crate) resolver: Resolver,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct BasicRecord {
    pub(crate) name: String,
    pub(crate) ttl: u32,
    #[serde(rename = "type")]
    pub(crate) dns_type: Vec<DnsType>,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) enum DnsType {
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
    #[serde(default)]
    pub(crate) id: Option<String>,
}

fn default_proxied() -> bool {
    false
}
