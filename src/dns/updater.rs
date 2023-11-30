use crate::config::domain::{Domain, Record};
use crate::dns::util::build_dns_content;
use cloudflare::endpoints::dns::{UpdateDnsRecord, UpdateDnsRecordParams};
use cloudflare::framework::async_api::Client;
use cloudflare::framework::response::ApiFailure;

pub async fn update_dns_record(client: &Client, domain: &Domain, record: &Record) -> Result<(), ApiFailure> {
    match client
        .request(&UpdateDnsRecord {
            zone_identifier: domain.zone_id.as_str(),
            identifier: record.id.as_str(),
            params: UpdateDnsRecordParams {
                ttl: Some(record.ttl),
                proxied: Some(record.proxied),
                name: record.dns_name.as_str(),
                content: build_dns_content(record.dns_type).await,
            },
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
