use crate::config::record_type::DnsRecordType;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Domain {
    pub auth_token: String,
    pub zone_id: String,
    pub records: Vec<Record>,
}

#[derive(Deserialize, Clone)]
pub struct Record {
    #[serde(rename = "record_id")]
    pub id: String,
    pub ttl: u32,
    pub proxied: bool,
    pub dns_name: String,
    pub dns_type: DnsRecordType,
}
