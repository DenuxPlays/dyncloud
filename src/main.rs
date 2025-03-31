use std::time::Duration;

use cloudflare::framework::client::async_api::Client;
use log::{error, info};
#[cfg(feature = "enable_mimalloc")]
use mimalloc::MiMalloc;
use tokio::time::interval;

use crate::configuration::config::Config;
use crate::configuration::domain::{Domain, Record};
use crate::dns::updater::update_dns_record;
use crate::ip::ip_changed::{has_ip_changed, LastIpAddresses};
use crate::util::{build_cloudflare_client, init_logger};

#[cfg(feature = "enable_mimalloc")]
#[cfg_attr(feature = "enable_mimalloc", global_allocator)]
static GLOBAL: MiMalloc = MiMalloc;

mod configuration;
mod dns;
mod ip;
mod util;

#[tokio::main]
async fn main() {
    init_logger();
    info!("Starting Dyncloud...!");
    let config = Config::init();
    let mut interval = interval(Duration::from_secs(config.update_interval_in_seconds as u64));
    let client = build_cloudflare_client(&config);
    let mut last_ip_addresses = LastIpAddresses::default();
    loop {
        for domain in &config.domains {
            update_every_record_in_domain(&client, domain, &mut last_ip_addresses).await;
        }
        interval.tick().await;
    }
}

async fn update_every_record_in_domain(client: &Client, domain: &Domain, last_ip_addresses: &mut LastIpAddresses) {
    for record in &domain.records {
        if has_ip_changed(record.dns_type, last_ip_addresses).await {
            info!("[*] Updating record '{}' with type '{}'", &record.dns_name, &record.dns_type);
            update_record(client, domain, record).await;
            continue;
        }
    }
}

async fn update_record(client: &Client, domain: &Domain, record: &Record) {
    match update_dns_record(client, domain, record).await {
        Ok(_) => info!("\t[*] Successfully updated record '{}' with type '{}'", &record.dns_name, &record.dns_type),
        Err(e) => {
            error!("\t[*] Failed to update record '{}' with type '{}': {:?}", &record.dns_name, &record.dns_type, e)
        }
    }
}
