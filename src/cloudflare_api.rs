use cloudflare::framework::Environment;
use cloudflare::framework::auth::Credentials;
use cloudflare::framework::client::ClientConfig;
use cloudflare::framework::client::blocking_api::HttpApiClient;
use std::net::IpAddr;
use std::time::Duration;

pub(crate) fn build_cloudflare_client(auth_token: String) -> HttpApiClient {
    build_cloudflare_client_with_token_and_ip(auth_token, None)
}

pub(crate) fn build_cloudflare_client_with_token_and_ip(auth_token: String, ip: Option<IpAddr>) -> HttpApiClient {
    let client_config = ClientConfig {
        http_timeout: Duration::from_secs(30),
        default_headers: http::HeaderMap::default(),
        resolve_ip: ip,
    };

    match HttpApiClient::new(
        Credentials::UserAuthToken {
            token: auth_token,
        },
        client_config,
        Environment::Production,
    ) {
        Ok(client) => client,
        Err(e) => panic!("Error creating Cloudflare client: {e:?}"),
    }
}
