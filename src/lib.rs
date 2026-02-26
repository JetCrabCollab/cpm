//! Crab Package Manager - A simple wrapper for npm and cargo
//!
//! CPM is a package manager that acts as an intelligent wrapper around npm and cargo,
//! providing a unified interface for JavaScript and Rust projects.

pub mod cli;
pub mod templates;
pub mod easter_egg;


// Re-export commonly used types
pub use cli::framework::{CliApp, CliCommand, CliContext, CliError, CliResult};
