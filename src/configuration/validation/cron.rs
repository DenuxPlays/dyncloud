use job_scheduler_ng::Cron;
use std::str::FromStr;
use tracing::debug;
use validator::ValidationError;

pub(crate) fn validate_cron_expression(expression: &str) -> Result<(), ValidationError> {
    match Cron::from_str(expression) {
        Ok(_) => Ok(()),
        Err(err) => {
            debug!("Invalid cron expression: {}", err);

            Err(ValidationError::new("Invalid cron expression"))
        }
    }
}
