use crate::configuration::user::records::RecordsGroup;
use validator::ValidationError;

pub(crate) fn validate_record_groups_schema(group: &RecordsGroup) -> Result<(), ValidationError> {
    if !group.cloudflare.is_empty() && group.providers.cloudflare.is_none() {
        return Err(ValidationError::new("Must provide a cloudflare config if you define Cloudflare records."));
    }

    Ok(())
}
