use crate::commands::cloudflare::CloudflareCommandError;
use crate::configuration::user::error::ConfigError;
use thiserror::Error;
use tracing::error;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

#[derive(Debug, Error)]
pub(crate) enum ApplicationError {
    #[error(transparent)]
    CloudflareCommandError(#[from] CloudflareCommandError),
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
    #[error(transparent)]
    ValidationErrors(#[from] ValidationErrors),
}

pub(crate) fn print_validation_errors(errors: &ValidationErrors) {
    error!("Validation failed:");
    walk_and_log(errors);
}

fn walk_and_log(root: &ValidationErrors) {
    let mut stack: Vec<(String, &ValidationErrors)> = Vec::new();
    stack.push((String::new(), root));

    while let Some((prefix, errs)) = stack.pop() {
        for (field, kind) in errs.errors().iter() {
            let path = build_path(&prefix, field);

            match kind {
                ValidationErrorsKind::Field(field_errors) => {
                    for e in field_errors {
                        log_field_error(&path, e);
                    }
                }
                ValidationErrorsKind::Struct(inner) => {
                    stack.push((path, inner));
                }
                ValidationErrorsKind::List(map) => {
                    for (idx, inner) in map {
                        let list_path = format!("{}[{}]", path, idx);
                        stack.push((list_path, inner));
                    }
                }
            }
        }
    }
}

fn build_path(prefix: &str, field: &str) -> String {
    if prefix.is_empty() {
        field.to_string()
    } else {
        let mut s = String::with_capacity(prefix.len() + 1 + field.len());
        s.push_str(prefix);
        s.push('.');
        s.push_str(field);
        s
    }
}

fn log_field_error(path: &str, e: &ValidationError) {
    let msg = e.message.as_ref().map(|m| m.as_ref()).unwrap_or_default();
    if msg.is_empty() {
        error!("- {}: {}", path, e.code);
    } else {
        error!("- {}: {} \u{2013} {}", path, e.code, msg);
    }

    if !e.params.is_empty() {
        let mut params: Vec<_> = e.params.iter().collect();
        params.sort_by(|a, b| a.0.cmp(b.0));
        let joined = params.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(", ");
        error!("  {}", joined);
    }
}
