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

pub const IPIFY_V4_URL: &str = "https://api.ipify.org";
pub const IPIFY_V6_URL: &str = "https://api6.ipify.org";

#[derive(Debug)]
pub struct IpGetter;

impl IpGetter {
    pub async fn get_public_ipv4() -> Result<Ipv4Addr, anyhow::Error> {
        let resp = reqwest::get(IPIFY_V4_URL).await?.text().await?;

        Ok(resp.parse()?)
    }

    pub async fn get_public_ipv6() -> Result<Ipv6Addr, anyhow::Error> {
        let resp = reqwest::get(IPIFY_V6_URL).await?.text().await?;

        Ok(resp.parse()?)
    }
}
