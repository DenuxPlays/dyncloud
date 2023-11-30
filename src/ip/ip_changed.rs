use std::net::{Ipv4Addr, Ipv6Addr};

use crate::configuration::record_type::DnsRecordType;
use crate::ip::ip_getter::{get_public_ipv4_address, get_public_ipv6_address};

pub struct LastIpAddresses {
    pub ipv4: Ipv4Addr,
    pub ipv6: Ipv6Addr,
}

impl Default for LastIpAddresses {
    fn default() -> Self {
        LastIpAddresses {
            ipv4: Ipv4Addr::new(127, 0, 0, 1),
            ipv6: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
        }
    }
}

pub async fn has_ip_changed(typ: DnsRecordType, last_ip_addresses: &mut LastIpAddresses) -> bool {
    match typ {
        DnsRecordType::A => has_ipv4_changed(last_ip_addresses).await,
        DnsRecordType::AAAA => has_ipv6_changed(last_ip_addresses).await,
    }
}

async fn has_ipv4_changed(last_ip_addresses: &mut LastIpAddresses) -> bool {
    let current_ip = get_public_ipv4_address().await;
    if last_ip_addresses.ipv4.eq(&current_ip) {
        false
    } else {
        last_ip_addresses.ipv4 = current_ip;
        true
    }
}

async fn has_ipv6_changed(last_ip_addresses: &mut LastIpAddresses) -> bool {
    let current_ip = get_public_ipv6_address().await;
    if last_ip_addresses.ipv6.eq(&current_ip) {
        false
    } else {
        last_ip_addresses.ipv6 = current_ip;
        true
    }
}
