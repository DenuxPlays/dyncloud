use crate::ip::cache::IpCache;
use crate::ip::resolver::IpResolver;
use anyhow::Error;
use reqwest::blocking::Client;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::Arc;

const IPIFY_V4_URL: &str = "https://api.ipify.org";
const IPIFY_V6_URL: &str = "https://api6.ipify.org";

pub(crate) struct IpifyResolver {
    cache: Arc<IpCache>,
    client: Client,
    ipv4_url: String,
    ipv6_url: String,
}

impl IpifyResolver {
    pub(crate) fn new(cache: Arc<IpCache>, client: Client, ipv4_url: String, ipv6_url: String) -> Self {
        Self {
            cache,
            client,
            ipv4_url,
            ipv6_url,
        }
    }

    pub(crate) fn from_ip_cache(cache: Arc<IpCache>) -> Self {
        Self::new(cache, Client::new(), IPIFY_V4_URL.to_string(), IPIFY_V6_URL.to_string())
    }

    pub(crate) fn resolve_ipv4_address(&self) -> Result<Ipv4Addr, Error> {
        let ip = self.client.get(self.ipv4_url.as_str()).send()?.text()?;
        let ip = ip.parse::<Ipv4Addr>()?;
        self.cache.set_ipv4addr(ip);

        Ok(ip)
    }

    pub(crate) fn resolve_ipv6_address(&self) -> Result<Ipv6Addr, Error> {
        let ip = self.client.get(self.ipv6_url.as_str()).send()?.text()?;
        let ip = ip.parse::<Ipv6Addr>()?;
        self.cache.set_ipv6addr(ip);

        Ok(ip)
    }
}

impl IpResolver for IpifyResolver {
    fn get_ipv4(&self) -> Result<Ipv4Addr, Error> {
        match self.cache.get_ipv4addr() {
            None => self.resolve_ipv4_address(),
            Some(ipv4_addr) => Ok(ipv4_addr),
        }
    }

    fn get_ipv6(&self) -> Result<Ipv6Addr, Error> {
        match self.cache.get_ipv6addr() {
            None => self.resolve_ipv6_address(),
            Some(ipv6_addr) => Ok(ipv6_addr),
        }
    }
}
