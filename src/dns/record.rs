use crate::configuration::user::providers::Cloudflare;
use crate::configuration::user::records::{CloudflareRecord as CloudflareConfigRecord, DnsType};
use crate::io_helper::CliWriter;
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

pub(crate) trait SyncableRecord: Send {
    fn sync(&mut self, progress_bar: &Option<ProgressBar>) -> Result<(), Error>;
}

pub(crate) struct CloudflareRecord {
    pub(crate) ip_resolver: Arc<Box<dyn IpResolver>>,
    pub(crate) client: Arc<HttpApiClient>,
    pub(crate) provider: Arc<Cloudflare>,
    pub(crate) record: CloudflareConfigRecord,
    pub(crate) writer: Arc<CliWriter>,
    pub(crate) id_cache: HashMap<DnsType, String>,
}

impl CloudflareRecord {
    pub(crate) fn new(
        ip_resolver: Arc<Box<dyn IpResolver>>,
        client: Arc<HttpApiClient>,
        provider: Arc<Cloudflare>,
        record: CloudflareConfigRecord,
        writer: Arc<CliWriter>,
    ) -> Self {
        Self {
            ip_resolver,
            client,
            provider,
            record,
            writer,
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
        self.writer.debug(format!(
            "Searching for existing {} record for {}",
            dns_type,
            self.record.basic_record.name.as_str()
        ));
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
                self.writer.error(format!(
                    "DNS Search for {} resulted in more than 1 result. ({} results)",
                    self.record.basic_record.name.as_str(),
                    len
                ));

                Err(anyhow!("Invalid search result length!"))
            }
        }
    }

    fn create_new_dns_record(&self, dns_type: &DnsType) -> Result<DnsRecord, Error> {
        self.writer.debug(format!(
            "No existing {:?} record found for {}. Creating a new one.",
            dns_type,
            self.record.basic_record.name.as_str()
        ));
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
        let types = self.record.basic_record.dns_type.clone();
        for dns_type in &types {
            self.writer.debug(format!("Syncing record {} of type {}", self.record.basic_record.name, dns_type));

            self.sync_dns_record(dns_type)?;
            if let Some(progress_bar) = progress_bar {
                progress_bar.inc(1);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Verbosity;
    use crate::configuration::user::providers::Cloudflare;
    use crate::configuration::user::records::{BasicRecord, CloudflareRecord as CloudflareConfigRecord, DnsType};
    use crate::dns::record::{CloudflareRecord, SyncableRecord};
    use crate::io_helper::CliWriter;
    use crate::ip::cache::IpCache;
    use crate::ip::resolver::IpResolver;
    use crate::ip::resolver::ipify::IpifyResolver;
    use cloudflare::framework::Environment;
    use cloudflare::framework::auth::Credentials;
    use cloudflare::framework::client::ClientConfig;
    use cloudflare::framework::client::blocking_api::HttpApiClient;
    use mockito::{Matcher, Mock, Server, ServerGuard};
    use reqwest::blocking::Client;
    use serde_json::json;
    use std::sync::Arc;

    #[test]
    fn test_sync_record() {
        let (ip_res, mut mocks, _server_guard) = get_mock_ip_resolver();
        let mut server = Server::new();

        mocks.push(
            server
                .mock("GET", "/zones/ZoneID/dns_records?name=test.example.internal")
                .match_body(Matcher::Missing)
                .with_status(200)
                .with_body(
                    json!({
                      "result": [
                        {
                          "id": "2eef68ee36ba268bb9aa3593e3ff7dc3",
                          "name": "test.example.internal",
                          "type": "A",
                          "content": "192.168.0.1",
                          "proxiable": true,
                          "proxied": false,
                          "ttl": 60,
                          "settings": {},
                          "meta": {},
                          "comment": null,
                          "tags": [],
                          "created_on": "2024-12-27T16:33:47.054786Z",
                          "modified_on": "2025-10-08T20:48:29.15931Z"
                        }
                      ],
                      "success": true,
                      "errors": [],
                      "messages": [],
                      "result_info": {
                        "page": 1,
                        "per_page": 100,
                        "count": 1,
                        "total_count": 1,
                        "total_pages": 1
                      }
                    })
                    .to_string(),
                )
                .create(),
        );

        mocks.push(
            server
                .mock("PUT", "/zones/ZoneID/dns_records/2eef68ee36ba268bb9aa3593e3ff7dc3")
                .match_body(Matcher::PartialJson(json!({"content": "127.0.0.1"})))
                .with_status(200)
                .with_body(
                    json!({
                      "result": {
                        "id": "2eef68ee36ba268bb9aa3593e3ff7dc3",
                              "name": "test.example.internal",
                              "type": "A",
                              "content": "127.0.0.1",
                              "proxiable": true,
                              "proxied": false,
                              "ttl": 60,
                              "settings": {},
                              "meta": {},
                              "comment": null,
                              "tags": [],
                              "created_on": "2024-12-27T16:33:47.054786Z",
                              "modified_on": "2025-10-08T20:48:29.15931Z"
                      },
                      "success": true,
                      "errors": [],
                      "messages": []
                    })
                    .to_string(),
                )
                .create(),
        );

        let http_client = Arc::new(
            HttpApiClient::new(
                Credentials::UserAuthToken {
                    token: "CustomAuthToken".to_string(),
                },
                ClientConfig::default(),
                Environment::Custom(server.url()),
            )
            .unwrap(),
        );
        let mut record = CloudflareRecord {
            ip_resolver: ip_res.clone(),
            client: http_client.clone(),
            provider: Arc::new(Cloudflare {
                auth_token: "CustomAuthToken".to_string(),
                zone_id: "ZoneID".to_string(),
            }),
            record: CloudflareConfigRecord {
                basic_record: BasicRecord {
                    name: "test.example.internal".to_string(),
                    ttl: 60,
                    dns_type: vec![DnsType::A],
                },
                proxied: false,
            },
            writer: Arc::new((CliWriter::new(&Verbosity::default()))),
            id_cache: Default::default(),
        };

        assert!(record.sync(&None).is_ok());

        mocks.iter().for_each(|mock| mock.assert());
    }

    fn get_mock_ip_resolver() -> (Arc<Box<dyn IpResolver>>, Vec<Mock>, ServerGuard) {
        let mut server = Server::new();
        let mut mocks = Vec::new();

        mocks.push(
            server
                .mock("GET", "/ipv4")
                .with_status(200)
                .with_header("content-type", "text/plain")
                .with_body("127.0.0.1")
                .create(),
        );

        (
            Arc::new(Box::new(IpifyResolver::new(
                Arc::new(IpCache::new(10)),
                Client::new(),
                format!("{}/ipv4", server.url().as_str()),
                format!("{}/ipv6", server.url().as_str()),
            ))),
            mocks,
            server,
        )
    }
}
