use crate::cloudflare_api::build_cloudflare_client;
use crate::io_helper::CliWriter;
use clap::Subcommand;
use cloudflare::endpoints::zones::zone::ListZones;
use cloudflare::framework::client::blocking_api::HttpApiClient;
use cloudflare::framework::response::ApiFailure;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;
use thiserror::Error;

#[derive(Subcommand)]
pub(crate) enum CloudflareCommands {
    #[command(name = "listZones", about = "List all of your cloudflare zones")]
    ListZones {
        #[arg(value_name = "AUTH_TOKEN", help = "Cloudflare API token")]
        auth_token: String,
    },
}

#[derive(Debug, Error)]
pub(crate) enum CloudflareCommandError {
    #[error(transparent)]
    ApiFailure(#[from] ApiFailure),
}

pub(crate) fn handle_cloudflare_commands(
    command: CloudflareCommands,
    writer: &CliWriter,
) -> Result<(), CloudflareCommandError> {
    match command {
        CloudflareCommands::ListZones {
            auth_token,
        } => {
            let client = build_cloudflare_client(auth_token);
            list_all_zones(&client, writer)?;
        }
    }

    Ok(())
}

fn list_all_zones(client: &HttpApiClient, writer: &CliWriter) -> Result<(), CloudflareCommandError> {
    writer.info("Requesting all of your zones from Cloudflare.\n");

    let response = client.request(&ListZones {
        params: Default::default(),
    })?;

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Zone ID", "Zone Name"]);
    for zone in response.result {
        table.add_row(vec![zone.id, zone.name]);
    }

    println!("{}", table);

    Ok(())
}
