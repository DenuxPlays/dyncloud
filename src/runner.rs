use crate::cloudflare_api::build_cloudflare_client;
use crate::configuration::user::config::Config;
use crate::configuration::user::records::RecordsGroup;
use crate::configuration::user::resolver::Resolver;
use crate::dns::record::{CloudflareRecord, SyncableRecord};
use crate::ip::cache::IpCache;
use crate::ip::resolver::IpResolver;
use crate::ip::resolver::ipify::IpifyResolver;
use job_scheduler_ng::{Cron, Job, JobScheduler};
use std::str::FromStr;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tracing::error;

pub(crate) struct Runner {
    records: Vec<Box<dyn SyncableRecord>>,
    cron: String,
}

impl Runner {
    pub(crate) fn new(config: Config) -> Self {
        let cron = config.cron.clone();
        let records = Self::build_records(config);

        Self {
            records,
            cron,
        }
    }

    pub(crate) fn sync(&self) -> Result<(), anyhow::Error> {
        for record in &self.records {
            record.sync()?
        }

        Ok(())
    }

    // TODO: add custom timezone support
    pub(crate) fn run(self) -> Result<(), anyhow::Error> {
        let mut scheduler = JobScheduler::new();

        let cron: Cron = Cron::from_str(&self.cron)?;
        for record in self.records {
            scheduler.add(Job::new(cron.clone(), move || {
                if let Err(err) = record.sync() {
                    error!("An error occurred while syncing records: {}", err);

                    std::process::exit(1);
                }
            }));
        }

        loop {
            scheduler.tick();

            sleep(Duration::from_millis(500))
        }
    }

    fn build_records(config: Config) -> Vec<Box<dyn SyncableRecord>> {
        let mut cf_records: Vec<Box<dyn SyncableRecord>> = Vec::new();

        let ip_cache = Arc::new(IpCache::new(config.get_total_number_of_records() as u64 * 2));

        for group in config.records {
            let resolver = Self::build_resolver(&group, ip_cache.clone());
            let provider = Arc::new(group.providers.cloudflare.unwrap());
            let client = Arc::new(build_cloudflare_client(provider.auth_token.clone()));
            for record in group.cloudflare {
                cf_records.push(Box::new(CloudflareRecord::new(
                    resolver.clone(),
                    client.clone(),
                    provider.clone(),
                    record,
                )));
            }
        }

        cf_records
    }

    fn build_resolver(records_group: &RecordsGroup, cache: Arc<IpCache>) -> Arc<Box<dyn IpResolver>> {
        match records_group.resolver {
            Resolver::Ipfiy => Arc::new(Box::new(IpifyResolver::from_ip_cache(cache))),
        }
    }
}
