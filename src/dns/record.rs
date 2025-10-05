use crate::configuration::user::providers::Cloudflare;
use crate::configuration::user::records::{CloudflareRecord as CloudflareConfigRecord, DnsType};
use crate::ip::resolver::IpResolver;
use anyhow::{Error, anyhow};
use cloudflare::endpoints::dns::dns::{
    CreateDnsRecord, CreateDnsRecordParams, DnsContent, DnsRecord, ListDnsRecords, ListDnsRecordsParams,
    UpdateDnsRecord, UpdateDnsRecordParams,
};
use cloudflare::framework::client::blocking_api::HttpApiClient;
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::error;

pub(crate) trait SyncableRecord: Send {
    fn sync(&mut self, progress_bar: &Option<ProgressBar>) -> Result<(), Error>;
}

pub(crate) struct CloudflareRecord {
    pub(crate) ip_resolver: Arc<Box<dyn IpResolver>>,
    pub(crate) client: Arc<HttpApiClient>,
    pub(crate) provider: Arc<Cloudflare>,
    pub(crate) record: CloudflareConfigRecord,
    pub(crate) id_cache: HashMap<DnsType, String>,
}

impl CloudflareRecord {
    pub(crate) fn new(
        ip_resolver: Arc<Box<dyn IpResolver>>,
        client: Arc<HttpApiClient>,
        provider: Arc<Cloudflare>,
        record: CloudflareConfigRecord,
    ) -> Self {
        Self {
            ip_resolver,
            client,
            provider,
            record,
            id_cache: HashMap::new(),
        }
    }

    fn sync_dns_record(&mut self, dns_type: &DnsType) -> Result<(), Error> {
        let id = self.get_record_id(dns_type)?;

        self.update_dns_record(id.as_str(), dns_type)?;
        Ok(())
    }

    fn update_dns_record(&self, id: &str, dns_type: &DnsType) -> Result<(), Error> {
        self.client.request(&UpdateDnsRecord {
            zone_identifier: self.provider.zone_id.as_str(),
            identifier: id,
            params: UpdateDnsRecordParams {
                ttl: Some(self.record.basic_record.ttl),
                proxied: Some(self.record.proxied),
                name: self.record.basic_record.name.as_str(),
                content: self.build_dns_content(dns_type)?,
            },
        })?;

        Ok(())
    }

    fn get_record_id(&mut self, dns_type: &DnsType) -> Result<String, Error> {
        match self.id_cache.get(dns_type) {
            None => self.resolve_record_id(dns_type),
            Some(id) => Ok(id.clone()),
        }
    }

    fn resolve_record_id(&mut self, dns_type: &DnsType) -> Result<String, Error> {
        let rs = self.client.request(&ListDnsRecords {
            zone_identifier: self.provider.zone_id.as_str(),
            params: ListDnsRecordsParams {
                record_type: None,
                name: Some(self.record.basic_record.name.clone()),
                page: None,
                per_page: None,
                order: None,
                direction: None,
                search_match: None,
            },
        })?;
        let results: Vec<DnsRecord> = rs
            .result
            .into_iter()
            .filter(|record| match (dns_type, &record.content) {
                (
                    DnsType::A,
                    DnsContent::A {
                        content: _,
                    },
                ) => true,
                (
                    DnsType::Aaaa,
                    DnsContent::AAAA {
                        content: _,
                    },
                ) => true,
                (_, _) => false,
            })
            .collect();

        match results.len() {
            0 => {
                let record = self.create_new_dns_record(dns_type)?;
                self.id_cache.insert(*dns_type, record.id.clone());

                Ok(record.id)
            }
            1 => {
                let id = results[0].id.clone();
                self.id_cache.insert(*dns_type, id.clone());

                Ok(id)
            }
            len => {
                error!(
                    "DNS Search for {} resulted in more than 1 result. ({} results)",
                    self.record.basic_record.name.as_str(),
                    len
                );

                Err(anyhow!("Invalid search result length!"))
            }
        }
    }

    fn create_new_dns_record(&self, dns_type: &DnsType) -> Result<DnsRecord, Error> {
        let rs = self.client.request(&CreateDnsRecord {
            zone_identifier: self.provider.zone_id.as_str(),
            params: CreateDnsRecordParams {
                ttl: Some(self.record.basic_record.ttl),
                priority: None,
                proxied: Some(self.record.proxied),
                name: self.record.basic_record.name.as_str(),
                content: self.build_dns_content(dns_type)?,
            },
        })?;

        Ok(rs.result)
    }

    fn build_dns_content(&self, dns_type: &DnsType) -> Result<DnsContent, Error> {
        Ok(match dns_type {
            DnsType::A => DnsContent::A {
                content: self.ip_resolver.get_ipv4()?,
            },
            DnsType::Aaaa => DnsContent::AAAA {
                content: self.ip_resolver.get_ipv6()?,
            },
        })
    }
}

impl SyncableRecord for CloudflareRecord {
    fn sync(&mut self, progress_bar: &Option<ProgressBar>) -> Result<(), Error> {
        let types = std::mem::take(&mut self.record.basic_record.dns_type);
        for dns_type in &types {
            self.sync_dns_record(dns_type)?;
            if let Some(progress_bar) = progress_bar {
                progress_bar.inc(1);
            }
        }

        Ok(())
    }
}
