use crate::cloudflare_api::build_cloudflare_client;
use clap::Subcommand;
use cloudflare::endpoints::zones::zone::ListZones;
use cloudflare::framework::client::blocking_api::HttpApiClient;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;
use tracing::info;

#[derive(Subcommand)]
pub(crate) enum CloudflareCommands {
    #[command(name = "listZones", about = "List all of your cloudflare zones")]
    ListZones {
        #[arg(short, long)]
        auth_token: String,
    },
}

pub(crate) fn handle_cloudflare_commands(command: CloudflareCommands) {
    match command {
        CloudflareCommands::ListZones {
            auth_token,
        } => {
            let client = build_cloudflare_client(auth_token);
            list_all_zones(&client)
        }
    }
}

fn list_all_zones(client: &HttpApiClient) {
    info!("Requesting all of your zones from Cloudflare.\n");

    // TODO: add special  handling for invalid API Token & Not correct rights api token
    let response = client
        .request(&ListZones {
            params: Default::default(),
        })
        .expect("Listing zones failed");

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Zone ID", "Zone Name"]);
    for zone in response.result {
        table.add_row(vec![zone.id, zone.name]);
    }

    println!("{}", table);
}
