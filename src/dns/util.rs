use cloudflare::endpoints::dns::dns::DnsContent;
use crate::configuration::record_type::DnsRecordType;
use crate::ip::ip_getter::{get_public_ipv4_address, get_public_ipv6_address};

pub async fn build_dns_content(typ: DnsRecordType) -> DnsContent {
    match typ {
        DnsRecordType::A => DnsContent::A {
            content: get_public_ipv4_address().await,
        },
        DnsRecordType::AAAA => DnsContent::AAAA {
            content: get_public_ipv6_address().await,
        },
    }
}
