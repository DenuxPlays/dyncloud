use crate::clock::get_system_timezone_offset;
use crate::cloudflare_api::build_cloudflare_client;
use crate::configuration::user::config::Config;
use crate::configuration::user::records::RecordsGroup;
use crate::configuration::user::resolver::Resolver;
use crate::dns::record::{CloudflareRecord, SyncableRecord};
use crate::io_helper::CliWriter;
use crate::ip::cache::IpCache;
use crate::ip::resolver::IpResolver;
use crate::ip::resolver::ipify::IpifyResolver;
use indicatif::ProgressBar;
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
    pub(crate) fn new(config: Config, writer: &Arc<CliWriter>) -> Self {
        let cron = config.cron.clone();
        let records = Self::build_records(config, writer);

        Self {
            records,
            cron,
        }
    }

    pub(crate) fn sync(&mut self, progress_bar: ProgressBar) -> Result<(), anyhow::Error> {
        let progress_bar = Some(progress_bar);
        for record in &mut self.records {
            record.sync(&progress_bar)?
        }

        Ok(())
    }

    pub(crate) fn run(self) -> Result<(), anyhow::Error> {
        let mut scheduler = JobScheduler::new();
        scheduler.set_timezone(get_system_timezone_offset());

        let cron: Cron = Cron::from_str(&self.cron)?;
        for mut record in self.records {
            scheduler.add(Job::new(cron.clone(), move || {
                if let Err(err) = record.sync(&None) {
                    error!("An error occurred while syncing records: {}", err);
                }
            }));
        }

        loop {
            scheduler.tick();

            sleep(Duration::from_millis(1_000))
        }
    }

    fn build_records(config: Config, writer: &Arc<CliWriter>) -> Vec<Box<dyn SyncableRecord>> {
        let mut cf_records: Vec<Box<dyn SyncableRecord>> = Vec::new();

        let ip_cache = Arc::new(IpCache::new(config.get_total_number_of_records() as u64 * 2));

        for group in config.records {
            let resolver = Self::build_resolver(&group, ip_cache.clone());
            if let Some(provider) = group.providers.cloudflare {
                let provider = Arc::new(provider);
                let client = Arc::new(build_cloudflare_client(provider.auth_token.clone()));
                for record in group.cloudflare {
                    cf_records.push(Box::new(CloudflareRecord::new(
                        resolver.clone(),
                        client.clone(),
                        provider.clone(),
                        record,
                        writer.clone(),
                    )));
                }
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
