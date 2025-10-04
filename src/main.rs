use crate::clap_utils::get_styles;
use crate::commands::cloudflare::{CloudflareCommands, handle_cloudflare_commands};
use crate::logger::init_tracing;
use crate::runner::Runner;
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity as ClapVerbosity};
use indicatif::ProgressBar;
use std::path::PathBuf;
use tracing::{error, info};

#[cfg(feature = "enable_mimalloc")]
#[cfg_attr(feature = "enable_mimalloc", global_allocator)]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod clap_utils;
mod cloudflare_api;
mod commands;
mod configuration;
mod dns;
mod error;
mod ip;
mod logger;
mod runner;

pub(crate) type Verbosity = ClapVerbosity<InfoLevel>;

#[derive(Parser)]
#[command(version, about, long_about = None, styles=get_styles())]
pub struct CliArgs {
    #[command(flatten)]
    pub(crate) verbosity: Verbosity,

    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    #[command(about = "Syncs all DNS Records one time.")]
    Sync {
        #[command(flatten)]
        common: CommonSyncRunArgs,
    },

    #[command(about = "Starts the process that periodically syncs DNS records.")]
    Run {
        #[command(flatten)]
        common: CommonSyncRunArgs,
    },

    #[command(about = "Helper commands for getting information from cloudflare")]
    Cloudflare {
        #[command(subcommand)]
        command: CloudflareCommands,
    },
}

#[derive(Args, Debug, Clone)]
struct CommonSyncRunArgs {
    #[arg(long, value_name = "FILE", help = "Path to the config file", default_value = "config.toml")]
    config_file: PathBuf,
}

fn main() {
    let args = CliArgs::parse();
    init_tracing(&args.verbosity);

    match args.command {
        Commands::Cloudflare {
            command,
        } => handle_cloudflare_commands(command),
        Commands::Sync {
            common,
        } => {
            // TODO: add progress bar
            let config = configuration::user::config::Config::from_file(common.config_file)
                .expect("Config file could not be parsed");

            let records_len = config.get_total_number_of_records();
            info!("Syncing DNS {} records...", records_len);

            let progress_bar = ProgressBar::new(records_len as u64);
            let runner = Runner::new(config);
            if let Err(err) = runner.sync(progress_bar) {
                error!("{}", err);

                return;
            }

            info!("Successfully synced {} record", records_len);
        }
        Commands::Run {
            common,
        } => {
            let config = configuration::user::config::Config::from_file(common.config_file)
                .expect("Config file could not be parsed");

            let records_len = config.get_total_number_of_records();
            info!("Running DNS sync for {} records...", records_len);

            let runner = Runner::new(config);
            if let Err(err) = runner.run() {
                error!("{}", err);
            }
        }
    }
}
