//! CPM CLI Framework - Base types and traits

use clap::{ArgMatches, Command};
use super::error::CliError;

/// Handle panic hook
pub fn handle_panic() {
    // Simple panic handler
}

/// Initialize logging
pub fn init_logging() {
    // Simple logging initialization
}

/// Setup signal handlers
pub fn setup_signal_handlers() -> Result<(), Box<dyn std::error::Error>> {
    // Simple signal handler setup
    Ok(())
}

/// Result type for CLI operations
pub type CliResult<T> = Result<T, CliError>;

/// CLI context for command execution
#[derive(Debug, Clone, Default)]
pub struct CliContext {
    pub verbose: bool,
    pub quiet: bool,
}


/// Trait for CLI commands
pub trait CliCommand: Send + Sync {
    /// Command name
    fn name(&self) -> &'static str;
    
    /// Build the clap command
    fn build_clap_command(&self) -> clap::Command;
    
    /// Execute the command
    fn execute(&self, context: &mut CliContext, matches: &ArgMatches) -> CliResult<()>;
}

/// Main CLI application
pub struct CliApp {
    name: &'static str,
    version: &'static str,
    description: &'static str,
    commands: Vec<Box<dyn CliCommand>>,
}

impl CliApp {
    pub fn new(name: &'static str, version: &'static str) -> Self {
        Self {
            name,
            version,
            description: "",
            commands: Vec::new(),
        }
    }
    
    pub fn description(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }
    
    pub fn add_command(mut self, command: Box<dyn CliCommand>) -> Self {
        self.commands.push(command);
        self
    }
    
    pub fn run(self) -> CliResult<()> {
        let mut clap_app = Command::new(self.name)
            .version(self.version)
            .about(self.description);
        
        // Add subcommands
        for command in &self.commands {
            clap_app = clap_app.subcommand(command.build_clap_command());
        }
        
        let matches = clap_app.get_matches();

        if let Some((command_name, sub_matches)) = matches.subcommand() {
        for command in &self.commands {
                if command.name() == command_name {
                    let mut context = CliContext::default();
                    return command.execute(&mut context, sub_matches);
                }
            }
        }
        
        // If no subcommand, show help
        println!("Use --help to see available commands");
        Ok(())
    }
}