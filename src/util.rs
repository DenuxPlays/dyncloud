use crate::config::config::Config;
use cloudflare::framework::async_api::Client;
use cloudflare::framework::auth::Credentials;
use cloudflare::framework::{Environment, HttpApiClientConfig};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use time::macros::format_description;

pub fn init_logger() {
    SimpleLogger::new()
        .env()
        .with_level(LevelFilter::Info)
        .with_timestamp_format(format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"))
        .init()
        .unwrap();
}

pub fn build_cloudflare_client(config: &Config) -> Client {
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
