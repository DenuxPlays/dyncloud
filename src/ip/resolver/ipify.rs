use crate::ip::resolver::IpResolver;
use anyhow::Error;
use reqwest::blocking::Client;
use std::net::{Ipv4Addr, Ipv6Addr};

const IPIFY_V4_URL: &str = "https://api.ipify.org";
const IPIFY_V6_URL: &str = "https://api6.ipify.org";

pub(crate) struct IpifyResolver {
    client: Client,
    ipv4_url: String,
    ipv6_url: String,
}

impl IpifyResolver {
    pub(crate) fn new(client: Client, ipv4_url: String, ipv6_url: String) -> Self {
        Self {
            client,
            ipv4_url,
            ipv6_url,
        }
    }

    pub(crate) fn resolve_ipv4_address(&self) -> Result<Ipv4Addr, Error> {
        let ip = self.client.get(self.ipv4_url.as_str()).send()?.text()?;
        let ip = ip.parse::<Ipv4Addr>()?;

        Ok(ip)
    }

    pub(crate) fn resolve_ipv6_address(&self) -> Result<Ipv6Addr, Error> {
        let ip = self.client.get(self.ipv6_url.as_str()).send()?.text()?;
        let ip = ip.parse::<Ipv6Addr>()?;

        Ok(ip)
    }
}

impl IpResolver for IpifyResolver {
    fn get_ipv4(&self) -> Result<Ipv4Addr, Error> {
        self.resolve_ipv4_address()
    }

    fn get_ipv6(&self) -> Result<Ipv6Addr, Error> {
        self.resolve_ipv6_address()
    }
}

impl Default for IpifyResolver {
    fn default() -> Self {
        Self::new(Client::new(), IPIFY_V4_URL.to_string(), IPIFY_V6_URL.to_string())
    }
}
