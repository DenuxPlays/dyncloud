use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};

pub async fn get_public_ipv4_address() -> Ipv4Addr {
    match IpGetter::get_public_ipv4().await {
        Ok(ip) => ip,
        Err(err) => panic!("Error getting public IPv4 address: {}", err),
    }
}

pub async fn get_public_ipv6_address() -> Ipv6Addr {
    match IpGetter::get_public_ipv6().await {
        Ok(ip) => ip,
        Err(err) => panic!("Error getting public IPv6 address: {}", err),
    }
}

#[derive(Debug, Deserialize)]
struct IpifyResponse {
    pub ip: String,
}

pub const IPIFY_V4_URL: &str = "https://api.ipify.org?format=json";
pub const IPIFY_V6_URL: &str = "https://api64.ipify.org?format=json";

#[derive(Debug)]
pub struct IpGetter;

impl IpGetter {
    pub async fn get_public_ipv4() -> Result<Ipv4Addr, anyhow::Error> {
        let resp = reqwest::get(IPIFY_V4_URL).await?.json::<IpifyResponse>().await?;

        Ok(resp.ip.parse()?)
    }

    pub async fn get_public_ipv6() -> Result<Ipv6Addr, anyhow::Error> {
        let resp = reqwest::get(IPIFY_V6_URL).await?.json::<IpifyResponse>().await?;

        Ok(resp.ip.parse()?)
    }
}
