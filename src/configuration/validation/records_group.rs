use crate::configuration::user::records::RecordsGroup;
use validator::ValidationError;

pub(crate) fn validate_record_groups_schema(group: &RecordsGroup) -> Result<(), ValidationError> {
    if !group.cloudflare.is_empty() && group.providers.cloudflare.is_none() {
        return Err(ValidationError::new("Must provide a cloudflare config if you define Cloudflare records."));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::configuration::user::providers::{Cloudflare, Providers};
    use crate::configuration::user::records::{BasicRecord, CloudflareRecord, DnsType, RecordsGroup};
    use crate::configuration::validation::records_group::validate_record_groups_schema;

    #[test]
    fn test_valid_cloudflare_record_group() {
        let group = RecordsGroup {
            providers: Providers {
                cloudflare: Some(create_base_cloudflare_provider()),
            },
            cloudflare: vec![create_base_cloudflare_record()],
            resolver: Default::default(),
        };

        assert!(validate_record_groups_schema(&group).is_ok());
    }

    #[test]
    fn test_invalid_cloudflare_record_group() {
        let group = RecordsGroup {
            providers: Providers {
                cloudflare: None,
            },
            cloudflare: vec![create_base_cloudflare_record()],
            resolver: Default::default(),
        };

        assert!(validate_record_groups_schema(&group).is_err());
    }

    #[test]
    fn test_cloudflare_provider_is_not_always_necessary() {
        let group = RecordsGroup {
            providers: Providers {
                cloudflare: None,
            },
            cloudflare: vec![],
            resolver: Default::default(),
        };

        assert!(validate_record_groups_schema(&group).is_ok());
    }

    fn create_base_cloudflare_provider() -> Cloudflare {
        Cloudflare {
            auth_token: "Some auth token".to_string(),
            zone_id: "My Zone id".to_string(),
        }
    }

    fn create_basic_record() -> BasicRecord {
        BasicRecord {
            name: "My Record".to_string(),
            ttl: 120,
            dns_type: vec![DnsType::Aaaa],
        }
    }

    fn create_base_cloudflare_record() -> CloudflareRecord {
        CloudflareRecord {
            basic_record: create_basic_record(),
            proxied: false,
        }
    }
}
