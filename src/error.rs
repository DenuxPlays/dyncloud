use crate::commands::cloudflare::CloudflareCommandError;
use crate::configuration::user::error::ConfigError;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub(crate) enum ApplicationError {
    #[error(transparent)]
    CloudflareCommandError(#[from] CloudflareCommandError),
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
    #[error(transparent)]
    ValidationErrors(#[from] ValidationErrors),
}
