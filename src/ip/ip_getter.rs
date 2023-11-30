use public_ip::{addr_v4, addr_v6};
use std::net::{Ipv4Addr, Ipv6Addr};

pub async fn get_public_ipv4_address() -> Ipv4Addr {
    match addr_v4().await {
        Some(ip) => ip,
        None => panic!("Error getting public IP address"),
    }
}

pub async fn get_public_ipv6_address() -> Ipv6Addr {
    match addr_v6().await {
        Some(ip) => ip,
        None => panic!("Error getting public IP address"),
    }
}
