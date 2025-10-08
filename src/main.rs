use crate::clap_utils::get_styles;
use crate::commands::cloudflare::{CloudflareCommands, handle_cloudflare_commands};
use crate::error::{ApplicationError, print_validation_errors};
use crate::logger::init_tracing;
use crate::runner::Runner;
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity as ClapVerbosity};
use configuration::user::config::Config;
use indicatif::ProgressBar;
use std::path::PathBuf;
use tracing::{error, info};
use validator::Validate;

#[cfg(feature = "mimalloc")]
#[cfg_attr(feature = "mimalloc", global_allocator)]
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
pub(crate) struct CliArgs {
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

    if let Err(err) = run_command(args) {
        if let ApplicationError::ValidationErrors(errors) = err {
            print_validation_errors(&errors);
        } else {
            error!("{}", err);
        }

        std::process::exit(1);
    }
}

fn run_command(args: CliArgs) -> Result<(), ApplicationError> {
    match args.command {
        Commands::Cloudflare {
            command,
        } => handle_cloudflare_commands(command)?,
        Commands::Sync {
            common,
        } => {
            let config = Config::from_file(common.config_file)?;
            config.validate()?;

            let records_len = config.get_total_number_of_records();
            info!("Syncing DNS {} records...", records_len);

            let progress_bar = ProgressBar::new(records_len as u64);
            let mut runner = Runner::new(config);
            if let Err(err) = runner.sync(progress_bar) {
                error!("{}", err);

                return Ok(());
            }

            info!("Successfully synced {} record", records_len);
        }
        Commands::Run {
            common,
        } => {
            let config = Config::from_file(common.config_file)?;
            config.validate()?;

            let records_len = config.get_total_number_of_records();
            info!("Running DNS sync for {} records...", records_len);

            let runner = Runner::new(config);
            if let Err(err) = runner.run() {
                error!("{}", err);
            }
        }
    }

    Ok(())
}
