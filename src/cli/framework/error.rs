//! CPM CLI Framework - Error types

use std::fmt;

/// CLI error types
#[derive(Debug)]
pub enum CliError {
    /// File operation error
    FileOperationError {
        operation: String,
        path: String,
        message: String,
    },
    /// File already exists
    FileExists { path: String },
    /// Execution error
    ExecutionError { command: String, message: String },
    /// Internal error
    InternalError { message: String },
    /// IO error
    IoError(std::io::Error),
    /// JSON error
    JsonError(serde_json::Error),
    /// TOML error
    TomlError(toml::de::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::FileOperationError {
                operation,
                path,
                message,
            } => {
                write!(
                    f,
                    "File operation '{operation}' failed on '{path}': {message}"
                )
            }
            CliError::FileExists { path } => {
                write!(f, "File already exists: {path}")
            }
            CliError::ExecutionError { command, message } => {
                write!(f, "Command '{command}' failed: {message}")
            }
            CliError::InternalError { message } => {
                write!(f, "Internal error: {message}")
            }
            CliError::IoError(e) => {
                write!(f, "IO error: {e}")
            }
            CliError::JsonError(e) => {
                write!(f, "JSON error: {e}")
            }
            CliError::TomlError(e) => {
                write!(f, "TOML error: {e}")
            }
        }
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::IoError(err)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        CliError::JsonError(err)
    }
}

impl From<toml::de::Error> for CliError {
    fn from(err: toml::de::Error) -> Self {
        CliError::TomlError(err)
    }
}
