pub(crate) mod ipify;

use std::net::{Ipv4Addr, Ipv6Addr};

pub(crate) trait IpResolver: Send + Sync {
    fn get_ipv4(&self) -> Result<Ipv4Addr, anyhow::Error>;
    fn get_ipv6(&self) -> Result<Ipv6Addr, anyhow::Error>;
}
