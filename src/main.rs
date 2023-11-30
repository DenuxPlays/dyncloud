use std::net::Ipv4Addr;

use crate::config::config::Config;
use cloudflare::endpoints::dns::{DnsContent, UpdateDnsRecord, UpdateDnsRecordParams};
use cloudflare::framework::async_api::Client;
use cloudflare::framework::auth::Credentials;
use cloudflare::framework::{Environment, HttpApiClientConfig};

mod config;
mod scheduling;
mod public_ip;

#[tokio::main]
async fn main() {
    let client = build_cloudflare_client();
    client
        .request(&UpdateDnsRecord {
            zone_identifier: "",
            identifier: "",
            params: UpdateDnsRecordParams {
                ttl: Some(1),
                proxied: Some(false),
                name: "",
                content: DnsContent::A {
                    content: get_public_ip_address().await,
                },
            },
        })
        .await
        .expect("Error updating DNS record");
}

fn build_cloudflare_client() -> Client {
    let config = Config::init();
    match Client::new(
        Credentials::UserAuthToken {
            token: config.domains[0].auth_token.clone(),
        },
        HttpApiClientConfig::default(),
        Environment::Production,
    ) {
        Ok(client) => client,
        Err(e) => panic!("Error creating Cloudflare client: {:?}", e),
    }
}

async fn get_public_ip_address() -> Ipv4Addr {
    match public_ip::addr_v4().await {
        Some(ip) => ip,
        None => panic!("Error getting public IP address"),
    }
}
