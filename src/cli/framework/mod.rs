//! CPM CLI Framework - Simplified framework for package management

pub mod base;
pub mod error;

// Re-export commonly used types
pub use base::{CliApp, CliCommand, CliContext, CliResult};
pub use error::CliError;
