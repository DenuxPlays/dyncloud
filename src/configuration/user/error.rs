use std::io::Error as IoError;
use std::path::PathBuf;
use thiserror::Error;
use toml::de::Error as DeserializationError;

#[derive(Debug, Error)]
pub(crate) enum ConfigError {
    #[error("Could not find configuration file: {0}")]
    FileNotFound(PathBuf),
    #[error("I/O error occurred: {0}")]
    Io(#[from] IoError),
    #[error("TOML deserialization error: {0}")]
    Deserialization(#[from] DeserializationError),
}
