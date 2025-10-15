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

#[cfg(test)]
mod tests {
    use crate::configuration::validation::cron::validate_cron_expression;

    #[test]
    fn test_valid_cron() {
        let cron = "* * * * *";
        assert!(validate_cron_expression(cron).is_ok());
    }

    #[test]
    fn test_invalid_cron() {
        let cron = "Not a valid cron expression";
        assert!(validate_cron_expression(cron).is_err());
    }

    #[test]
    fn test_empty_cron() {
        let cron = "";
        assert!(validate_cron_expression(cron).is_err());
    }
}
