use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum DnsRecordType {
    A,
    AAAA,
}

impl Display for DnsRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsRecordType::A => write!(f, "A"),
            DnsRecordType::AAAA => write!(f, "AAAA"),
        }
    }
}
