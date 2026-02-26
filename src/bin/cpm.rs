//! Crab Package Manager - A modern package manager for JavaScript and Rust

use clap::ArgMatches;
use cpm::cli::framework::{CliApp, CliCommand, CliContext, CliError, CliResult};
use cpm::easter_egg::{should_trigger_easter_egg, show_walking_claw};
use std::path::PathBuf;
use tracing::info;

struct InitCommand;

struct AddRustCommand;

struct RemoveRustCommand;

struct RustStatusCommand;

struct NpxCommand;

struct AddCommand;
struct RemoveCommand;

struct LockCommand;

struct WorkspaceCommand;

struct PublishCommand;

struct InstallCommand;
struct BuildCommand;
struct DevCommand;
struct TestCommand;
struct RunCommand;


impl InitCommand {
    fn new() -> Self {
        Self
    }

    fn add_cpm_scripts_to_package_json(&self) -> CliResult<()> {
        // Read existing package.json
        let package_json_content = std::fs::read_to_string("package.json")?;
        let mut package_json: serde_json::Value = serde_json::from_str(&package_json_content)?;

        // Add CPM scripts
        if let Some(scripts) = package_json.get_mut("scripts") {
            if let Some(scripts_obj) = scripts.as_object_mut() {
                scripts_obj.insert(
                    "dev".to_string(),
                    serde_json::Value::String("cpm dev".to_string()),
                );
                scripts_obj.insert(
                    "build".to_string(),
                    serde_json::Value::String("cpm build".to_string()),
                );
                scripts_obj.insert(
                    "test".to_string(),
                    serde_json::Value::String("cpm test".to_string()),
                );
                scripts_obj.insert(
                    "preinstall".to_string(),
                    serde_json::Value::String("echo 'preinstall' || true".to_string()),
                );
                scripts_obj.insert(
                    "postinstall".to_string(),
                    serde_json::Value::String("echo 'postinstall' || true".to_string()),
                );
            }
        } else {
            let mut scripts = serde_json::Map::new();
            scripts.insert("dev".to_string(), serde_json::Value::String("cpm dev".to_string()));
            scripts.insert("build".to_string(), serde_json::Value::String("cpm build".to_string()));
            scripts.insert("test".to_string(), serde_json::Value::String("cpm test".to_string()));
            scripts.insert("preinstall".to_string(), serde_json::Value::String("echo preinstall || true".to_string()));
            scripts.insert("postinstall".to_string(), serde_json::Value::String("echo postinstall || true".to_string()));
            package_json["scripts"] = serde_json::Value::Object(scripts);
        }

        // Write back to package.json
        let updated_content = serde_json::to_string_pretty(&package_json)?;
        std::fs::write("package.json", updated_content)?;

        Ok(())
    }
}

impl CliCommand for InitCommand {
    fn name(&self) -> &'static str {
        "init"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("init")
            .about("Initialize a new JavaScript project")
            .arg(
                clap::Arg::new("yes")
                    .short('y')
                    .long("yes")
                    .help("Use default values without prompting")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(clap::Arg::new("name").help("Project name").index(1))
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        let project_name = matches
            .get_one::<String>("name")
            .map(|s| s.as_str())
            .unwrap_or("my-cpm-project");

        let yes = matches.get_flag("yes");

        info!("Initializing JavaScript project: {}", project_name);

        eprintln!("🚀 Initializing CPM JavaScript project: {project_name}");
        eprintln!();

        // Create project directory
        let project_dir = PathBuf::from(project_name);
        if project_dir.exists() {
            return Err(CliError::FileExists {
                path: project_name.to_string(),
            });
        }

        std::fs::create_dir(&project_dir)?;
        std::env::set_current_dir(&project_dir)?;

        eprintln!("📦 Setting up JavaScript project...");

        // Run npm init
        let npm_args = if yes {
            vec!["init", "-y"]
        } else {
            vec!["init"]
        };

        // Try to find npm in common locations
        let npm_cmd = if cfg!(windows) {
            // On Windows, try npm.cmd first, then npm
            if std::process::Command::new("npm.cmd")
                .arg("--version")
                .output()
                .is_ok()
            {
                "npm.cmd"
            } else {
                "npm"
            }
        } else {
            "npm"
        };

        let npm_output = std::process::Command::new(npm_cmd)
            .args(&npm_args)
            .output()?;

        if !npm_output.status.success() {
            let stderr = String::from_utf8_lossy(&npm_output.stderr);
            let stdout = String::from_utf8_lossy(&npm_output.stdout);
            return Err(CliError::ExecutionError {
                command: format!("{npm_cmd} init"),
                message: format!("stderr: {stderr}\nstdout: {stdout}"),
            });
        }

        // Add CPM scripts to package.json
        self.add_cpm_scripts_to_package_json()?;

        // Create basic index.js
        let index_js = r#"// CPM JavaScript Project
console.log('Hello from CPM! 🦀');

// Example function
function greet(name) {
    console.log(`Hello, ${name}! Welcome to CPM.`);
}

// Call the function
greet('World');
"#;

        std::fs::write("index.js", index_js)?;

        // Create README
        let readme = format!(
            r#"# {project_name} - CPM JavaScript Project

This is a JavaScript project managed by CPM (Crab Package Manager).

## Getting Started

1. **Install dependencies:**
   ```bash
   cpm install
   ```

2. **Start development server:**
   ```bash
   cpm dev
   ```

3. **Build the project:**
   ```bash
   cpm build
   ```

## Adding Rust (Optional)

To add Rust to this project later:
```bash
cpm add-rust
```

## Available Commands

- `cpm install` - Install dependencies
- `cpm build` - Build the project
- `cpm dev` - Start development server
- `cpm test` - Run tests
- `cpm add-rust` - Add Rust to the project
- `cpm rust-status` - Check Rust status
"#
        );

        std::fs::write("README.md", readme)?;

        eprintln!("✅ JavaScript project initialized successfully!");
        eprintln!("💡 Run 'cpm install' to install dependencies");
        eprintln!("💡 Run 'cpm dev' to start development server");
        eprintln!("💡 Run 'cpm add-rust' to add Rust later if needed");

        Ok(())
    }
}

impl AddRustCommand {
    fn new() -> Self {
        Self
    }

    fn add_cpm_scripts_to_cargo_toml(&self) -> CliResult<()> {
        // Read existing Cargo.toml
        let cargo_toml_content = std::fs::read_to_string("Cargo.toml")?;
        let mut cargo_toml: toml::Value = toml::from_str(&cargo_toml_content)?;

        // Ensure [lib] section exists with cdylib
        if let Some(package) = cargo_toml.get_mut("package") {
            if let Some(_package_obj) = package.as_table_mut() {
                // Add [lib] section
                let mut lib_section = toml::map::Map::new();
                lib_section.insert(
                    "crate-type".to_string(),
                    toml::Value::Array(vec![toml::Value::String("cdylib".to_string())]),
                );
                cargo_toml["lib"] = toml::Value::Table(lib_section);
            }
        }

        // Add WASM dependencies
        let mut dependencies = toml::map::Map::new();
        dependencies.insert(
            "wasm-bindgen".to_string(),
            toml::Value::String("0.2".to_string()),
        );
        dependencies.insert("serde".to_string(), toml::Value::String("1.0".to_string()));
        dependencies.insert(
            "serde-wasm-bindgen".to_string(),
            toml::Value::String("0.6".to_string()),
        );
        dependencies.insert(
            "web-sys".to_string(),
            toml::Value::String("0.3".to_string()),
        );

        cargo_toml["dependencies"] = toml::Value::Table(dependencies);

        // Write back to Cargo.toml
        let updated_content =
            toml::to_string_pretty(&cargo_toml).map_err(|e| CliError::InternalError {
                message: format!("Failed to serialize Cargo.toml: {e}"),
            })?;
        std::fs::write("Cargo.toml", updated_content)?;

        Ok(())
    }

    fn create_rust_files(&self, _project_name: &str) -> CliResult<()> {
        // Create src directory if it doesn't exist
        std::fs::create_dir_all("src")?;

        // Create lib.rs
        let lib_rs = r#"use wasm_bindgen::prelude::*;

// Import console.log from web-sys
use web_sys::console;

// A macro to provide `eprintln!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/// Greet a user with a message from Rust
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}! You've been greeted from Rust!", name));
}

/// Add two numbers and return the result
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    log!("Adding {} + {}", a, b);
    a + b
}

/// Calculate the nth Fibonacci number
#[wasm_bindgen]
pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

/// Get the current Rust version
#[wasm_bindgen]
pub fn get_rust_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
"#;

        std::fs::write("src/lib.rs", lib_rs)?;

        // Create pkg directory
        std::fs::create_dir_all("pkg")?;

        // Update package.json to include WASM
        let package_json_content = std::fs::read_to_string("package.json")?;
        let mut package_json: serde_json::Value = serde_json::from_str(&package_json_content)?;

        // Add WASM dependency
        if let Some(dependencies) = package_json.get_mut("dependencies") {
            if let Some(deps_obj) = dependencies.as_object_mut() {
                deps_obj.insert(
                    "wasm-bindgen".to_string(),
                    serde_json::Value::String("^0.2".to_string()),
                );
            }
        } else {
            let mut dependencies = serde_json::Map::new();
            dependencies.insert(
                "wasm-bindgen".to_string(),
                serde_json::Value::String("^0.2".to_string()),
            );
            package_json["dependencies"] = serde_json::Value::Object(dependencies);
        }

        // Write back to package.json
        let updated_content = serde_json::to_string_pretty(&package_json)?;
        std::fs::write("package.json", updated_content)?;

        Ok(())
    }
}

impl CliCommand for AddRustCommand {
    fn name(&self) -> &'static str {
        "add-rust"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("add-rust")
            .about("Add Rust to an existing JavaScript project")
            .arg(
                clap::Arg::new("yes")
                    .short('y')
                    .long("yes")
                    .help("Use default values without prompting")
                    .action(clap::ArgAction::SetTrue),
            )
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        let _yes = matches.get_flag("yes");

        info!("Adding Rust to existing JavaScript project");

        // Check if we're in a JavaScript project
        if !std::path::Path::new("package.json").exists() {
            return Err(CliError::FileOperationError {
                operation: "read package.json".to_string(),
                path: "package.json".to_string(),
                message: "Not in a JavaScript project. Run 'cpm init' first.".to_string(),
            });
        }

        // Check if Rust is already added
        if std::path::Path::new("Cargo.toml").exists() {
            eprintln!("⚠️  Rust is already added to this project!");
            eprintln!("💡 Run 'cpm rust-status' to check the current status");
            return Ok(());
        }

        eprintln!("🦀 Adding Rust to JavaScript project...");

        // Get project name from package.json
        let package_json_content = std::fs::read_to_string("package.json")?;
        let package_json: serde_json::Value = serde_json::from_str(&package_json_content)?;
        let project_name = package_json["name"]
            .as_str()
            .unwrap_or("my-project")
            .replace('-', "_");

        // Run cargo init
        let cargo_args = vec!["init", "--name", &project_name, "--lib"];

        let cargo_output = std::process::Command::new("cargo")
            .args(&cargo_args)
            .output()?;

        if !cargo_output.status.success() {
            return Err(CliError::ExecutionError {
                command: "cargo init".to_string(),
                message: String::from_utf8_lossy(&cargo_output.stderr).to_string(),
            });
        }

        // Add CPM scripts to Cargo.toml
        self.add_cpm_scripts_to_cargo_toml()?;

        // Create Rust files
        self.create_rust_files(&project_name)?;

        eprintln!("✅ Rust added to project successfully!");
        eprintln!("💡 Run 'cpm build' to compile Rust to WASM");
        eprintln!("💡 Run 'cpm dev' to start development server");
        eprintln!("💡 Check 'src/lib.rs' for Rust code examples");

        Ok(())
    }
}

impl RemoveRustCommand {
    fn new() -> Self {
        Self
    }
}

impl CliCommand for RemoveRustCommand {
    fn name(&self) -> &'static str {
        "remove-rust"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("remove-rust")
            .about("Remove Rust from a project")
            .arg(
                clap::Arg::new("yes")
                    .short('y')
                    .long("yes")
                    .help("Remove without prompting")
                    .action(clap::ArgAction::SetTrue),
            )
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        let yes = matches.get_flag("yes");

        info!("Removing Rust from project");

        // Check if Rust is present
        if !std::path::Path::new("Cargo.toml").exists() {
            eprintln!("⚠️  No Rust found in this project!");
            return Ok(());
        }

        if !yes {
            eprintln!("⚠️  This will remove all Rust files and dependencies!");
            eprintln!("   - Cargo.toml");
            eprintln!("   - src/ directory");
            eprintln!("   - pkg/ directory");
            eprintln!("   - Rust dependencies from package.json");
            eprintln!();
            eprintln!("   Continue? (y/N): ");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            if !input.trim().to_lowercase().starts_with('y') {
                eprintln!("❌ Operation cancelled");
                return Ok(());
            }
        }

        eprintln!("🗑️  Removing Rust from project...");

        // Remove Rust files
        if std::path::Path::new("Cargo.toml").exists() {
            std::fs::remove_file("Cargo.toml")?;
        }

        if std::path::Path::new("src").exists() {
            std::fs::remove_dir_all("src")?;
        }

        if std::path::Path::new("pkg").exists() {
            std::fs::remove_dir_all("pkg")?;
        }

        // Remove WASM dependencies from package.json
        let package_json_content = std::fs::read_to_string("package.json")?;
        let mut package_json: serde_json::Value = serde_json::from_str(&package_json_content)?;

        if let Some(dependencies) = package_json.get_mut("dependencies") {
            if let Some(deps_obj) = dependencies.as_object_mut() {
                deps_obj.remove("wasm-bindgen");
            }
        }

        // Write back to package.json
        let updated_content = serde_json::to_string_pretty(&package_json)?;
        std::fs::write("package.json", updated_content)?;

        eprintln!("✅ Rust removed from project successfully!");
        eprintln!("💡 Project is now JavaScript-only");

        Ok(())
    }
}

impl CliCommand for RustStatusCommand {
    fn name(&self) -> &'static str {
        "rust-status"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("rust-status").about("Check Rust status in the current project")
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        info!("Checking Rust status");

        eprintln!("🔍 Checking Rust status in current project...");
        eprintln!();

        // Check if we're in a JavaScript project
        if !std::path::Path::new("package.json").exists() {
            eprintln!("❌ Not in a JavaScript project");
            eprintln!("💡 Run 'cpm init' to create a new project");
            return Ok(());
        }

        // Check Rust files
        let has_cargo_toml = std::path::Path::new("Cargo.toml").exists();
        let has_src_dir = std::path::Path::new("src").exists();
        let has_pkg_dir = std::path::Path::new("pkg").exists();
        let has_lib_rs = std::path::Path::new("src/lib.rs").exists();

        eprintln!("📁 Project Structure:");
        eprintln!("   package.json: ✅");
        eprintln!(
            "   Cargo.toml: {}",
            if has_cargo_toml { "✅" } else { "❌" }
        );
        eprintln!(
            "   src/ directory: {}",
            if has_src_dir { "✅" } else { "❌" }
        );
        eprintln!("   src/lib.rs: {}", if has_lib_rs { "✅" } else { "❌" });
        eprintln!(
            "   pkg/ directory: {}",
            if has_pkg_dir { "✅" } else { "❌" }
        );

        if has_cargo_toml && has_src_dir && has_lib_rs {
            eprintln!();
            eprintln!("🦀 Rust is fully integrated!");
            eprintln!("💡 Run 'cpm build' to compile Rust to WASM");
            eprintln!("💡 Run 'cpm dev' to start development server");
        } else if has_cargo_toml || has_src_dir {
            eprintln!();
            eprintln!("⚠️  Rust is partially integrated");
            eprintln!("💡 Run 'cpm add-rust' to complete the setup");
        } else {
            eprintln!();
            eprintln!("📦 JavaScript-only project");
            eprintln!("💡 Run 'cpm add-rust' to add Rust if needed");
        }

        Ok(())
    }
}

impl NpxCommand {
    fn new() -> Self {
        Self
    }
}

impl CliCommand for AddCommand {
    fn name(&self) -> &'static str {
        "add"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("add")
            .about("Add a package (delegates to npm install)")
            .arg(clap::Arg::new("packages").required(true).num_args(1..))
            .arg(clap::Arg::new("save_dev").short('D').long("save-dev").action(clap::ArgAction::SetTrue))
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        if !std::path::Path::new("package.json").exists() {
            return Err(CliError::FileOperationError {
                operation: "add package".to_string(),
                path: "package.json".to_string(),
                message: "Not in a JavaScript project. Run 'cpm init' first.".to_string(),
            });
        }
        let packages: Vec<&String> = matches.get_many::<String>("packages").unwrap_or_default().collect();
        if packages.is_empty() {
            eprintln!("Usage: cpm add <package> [packages...]");
            return Ok(());
        }
        let save_dev = matches.get_flag("save_dev");
        let npm_cmd = if cfg!(target_os = "windows") { "npm.cmd" } else { "npm" };
        let mut args = vec!["install".to_string()];
        if save_dev {
            args.push("--save-dev".to_string());
        }
        args.extend(packages.iter().map(|s| (*s).clone()));
        let status = std::process::Command::new(npm_cmd).args(&args).status()?;
        if !status.success() {
            return Err(CliError::ExecutionError {
                command: format!("npm install {}", packages.join(" ")),
                message: "npm install failed".to_string(),
            });
        }
        eprintln!("Packages added successfully.");
        Ok(())
    }
}

impl CliCommand for LockCommand {
    fn name(&self) -> &'static str {
        "lock"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("lock")
            .about("Update lockfiles (package-lock.json, Cargo.lock)")
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        eprintln!("Updating lockfiles...");
        if std::path::Path::new("package.json").exists() {
            let npm_cmd = if cfg!(target_os = "windows") { "npm.cmd" } else { "npm" };
            let status = std::process::Command::new(npm_cmd)
                .args(["install", "--package-lock-only"])
                .status()?;
            if status.success() {
                eprintln!("package-lock.json updated.");
            } else {
                return Err(CliError::ExecutionError {
                    command: "npm install --package-lock-only".to_string(),
                    message: "Failed to update package-lock.json".to_string(),
                });
            }
        }
        if std::path::Path::new("Cargo.toml").exists() {
            let status = std::process::Command::new("cargo").arg("update").status()?;
            if status.success() {
                eprintln!("Cargo.lock updated.");
            } else {
                return Err(CliError::ExecutionError {
                    command: "cargo update".to_string(),
                    message: "Failed to update Cargo.lock".to_string(),
                });
            }
        }
        if !std::path::Path::new("package.json").exists() && !std::path::Path::new("Cargo.toml").exists() {
            eprintln!("No project found. Run 'cpm init' first.");
        }
        Ok(())
    }
}

impl CliCommand for RemoveCommand {
    fn name(&self) -> &'static str {
        "remove"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("remove")
            .about("Remove a package (delegates to npm uninstall)")
            .arg(clap::Arg::new("packages").required(true).num_args(1..))
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        if !std::path::Path::new("package.json").exists() {
            return Err(CliError::FileOperationError {
                operation: "remove package".to_string(),
                path: "package.json".to_string(),
                message: "Not in a JavaScript project. Run 'cpm init' first.".to_string(),
            });
        }
        let packages: Vec<&String> = matches.get_many::<String>("packages").unwrap_or_default().collect();
        if packages.is_empty() {
            eprintln!("Usage: cpm remove <package> [packages...]");
            return Ok(());
        }
        let npm_cmd = if cfg!(target_os = "windows") { "npm.cmd" } else { "npm" };
        let mut args = vec!["uninstall".to_string()];
        args.extend(packages.iter().map(|s| (*s).clone()));
        let status = std::process::Command::new(npm_cmd).args(&args).status()?;
        if !status.success() {
            return Err(CliError::ExecutionError {
                command: format!("npm uninstall {}", packages.join(" ")),
                message: "npm uninstall failed".to_string(),
            });
        }
        eprintln!("Packages removed successfully.");
        Ok(())
    }
}

impl CliCommand for NpxCommand {
    fn name(&self) -> &'static str {
        "npx"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("npx")
            .about("Execute packages using npx")
            .arg(
                clap::Arg::new("package")
                    .help("Package to execute")
                    .index(1),
            )
            .arg(
                clap::Arg::new("args")
                    .help("Arguments to pass to the package")
                    .num_args(0..)
                    .last(true),
            )
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        let package = matches.get_one::<String>("package");
        let args: Vec<&String> = matches
            .get_many::<String>("args")
            .unwrap_or_default()
            .collect();

        if let Some(pkg) = package {
            eprintln!("🦀 CPM detected npx command for package: {pkg}");
            eprintln!("📦 Executing with npx...");

            // Build the npx command
            let mut npx_cmd = std::process::Command::new("npx");
            npx_cmd.arg(pkg);
            npx_cmd.args(&args);

            // Execute the command
            let status = npx_cmd.status()?;

            if !status.success() {
                return Err(CliError::ExecutionError {
                    command: format!("npx {pkg}"),
                    message: "npx command failed".to_string(),
                });
            }
        } else {
            eprintln!("❌ No package specified for npx");
            eprintln!("💡 Usage: cpm npx <package> [args...]");
        }

        Ok(())
    }
}

impl CliCommand for PublishCommand {
    fn name(&self) -> &'static str {
        "publish"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("publish")
            .about("Publish package to registry (delegates to npm publish)")
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        if !std::path::Path::new("package.json").exists() {
            return Err(CliError::FileOperationError {
                operation: "publish".to_string(),
                path: "package.json".to_string(),
                message: "Not in a project. Run 'cpm init' first.".to_string(),
            });
        }
        let npm_cmd = if cfg!(target_os = "windows") { "npm.cmd" } else { "npm" };
        let status = std::process::Command::new(npm_cmd)
            .arg("publish")
            .status()?;
        if !status.success() {
            return Err(CliError::ExecutionError {
                command: "npm publish".to_string(),
                message: "Publish failed".to_string(),
            });
        }
        eprintln!("Package published successfully.");
        Ok(())
    }
}

impl CliCommand for WorkspaceCommand {
    fn name(&self) -> &'static str {
        "workspace"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("workspace")
            .about("List workspace packages (npm workspaces)")
            .arg(clap::Arg::new("ls").short('l').long("ls").action(clap::ArgAction::SetTrue))
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        if !std::path::Path::new("package.json").exists() {
            return Err(CliError::FileOperationError {
                operation: "read package.json".to_string(),
                path: "package.json".to_string(),
                message: "Not in a project. Run 'cpm init' first.".to_string(),
            });
        }
        let content = std::fs::read_to_string("package.json")?;
        let pkg: serde_json::Value = serde_json::from_str(&content)?;
        let workspaces = pkg.get("workspaces");
        match workspaces {
            Some(serde_json::Value::Array(arr)) => {
                eprintln!("Workspace packages:");
                for w in arr {
                    if let Some(s) = w.as_str() {
                        eprintln!("  - {}", s);
                    }
                }
            }
            Some(serde_json::Value::Object(obj)) => {
                if let Some(arr) = obj.get("packages").and_then(|v| v.as_array()) {
                    eprintln!("Workspace packages:");
                    for w in arr {
                        if let Some(s) = w.as_str() {
                            eprintln!("  - {}", s);
                        }
                    }
                }
            }
            _ => eprintln!("No workspaces configured. Add \"workspaces\": [\"packages/*\"] to package.json."),
        }
        Ok(())
    }
}

impl CliCommand for InstallCommand {
    fn name(&self) -> &'static str {
        "install"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("install")
            .about("Install dependencies (supports npm workspaces)")
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        eprintln!("📦 Installing dependencies...");

        // Check if we're in a JavaScript project
        if std::path::Path::new("package.json").exists() {
            eprintln!("🟨 Installing JavaScript dependencies with npm...");

            // Try npm.cmd on Windows first, then npm
            let npm_cmd = if cfg!(target_os = "windows") {
                "npm.cmd"
            } else {
                "npm"
            };

            let npm_output = std::process::Command::new(npm_cmd)
                .args(["install"])
                .output()?;

            if !npm_output.status.success() {
                return Err(CliError::ExecutionError {
                    command: "npm install".to_string(),
                    message: String::from_utf8_lossy(&npm_output.stderr).to_string(),
                });
            }
            eprintln!("✅ JavaScript dependencies installed!");
        }

        // Check if we're in a Rust project
        if std::path::Path::new("Cargo.toml").exists() {
            eprintln!("🦀 Installing Rust dependencies with cargo...");
            let cargo_output = std::process::Command::new("cargo").arg("build").output()?;

            if !cargo_output.status.success() {
                return Err(CliError::ExecutionError {
                    command: "cargo build".to_string(),
                    message: String::from_utf8_lossy(&cargo_output.stderr).to_string(),
                });
            }
            eprintln!("✅ Rust dependencies installed!");
        }

        Ok(())
    }
}

impl CliCommand for BuildCommand {
    fn name(&self) -> &'static str {
        "build"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("build").about("Build the project")
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        eprintln!("🔨 Building project...");

        // Check if we're in a Rust project
        if std::path::Path::new("Cargo.toml").exists() {
            eprintln!("🦀 Building Rust project...");
            let cargo_output = std::process::Command::new("cargo").arg("build").output()?;

            if !cargo_output.status.success() {
                return Err(CliError::ExecutionError {
                    command: "cargo build".to_string(),
                    message: String::from_utf8_lossy(&cargo_output.stderr).to_string(),
                });
            }

            // If wasm-pack is available, build WASM
            if std::process::Command::new("wasm-pack")
                .arg("--version")
                .output()
                .is_ok()
            {
                eprintln!("🌐 Building WebAssembly...");
                let wasm_output = std::process::Command::new("wasm-pack")
                    .args(["build", "--release", "--target", "web", "--out-dir", "pkg"])
                    .output()?;

                if !wasm_output.status.success() {
                    eprintln!("⚠️  WASM build failed, but continuing...");
                } else {
                    eprintln!("✅ WebAssembly built successfully!");
                }
            }

            eprintln!("✅ Rust project built!");
        } else {
            // It's a JavaScript project
            eprintln!("📦 JavaScript project detected.");
            eprintln!("🦀 Bundling with JetCrab Runtime...");

            let mut entry_file = "index.js";
            if std::path::Path::new("js/index.js").exists() {
                entry_file = "js/index.js";
            }
            
            // Get project name from package.json
            let mut output_name = "app.exe".to_string();
            if let Ok(content) = std::fs::read_to_string("package.json") {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(name) = json["name"].as_str() {
                        output_name = if cfg!(windows) {
                            format!("{}.exe", name)
                        } else {
                            name.to_string()
                        };
                    }
                }
            }

            eprintln!("🔨 Bundling '{}' into '{}'...", entry_file, output_name);

            // Bundling Logic (Native)
            let build_dir = std::path::Path::new("jetcrab_build");
            if build_dir.exists() {
                std::fs::remove_dir_all(build_dir)?;
            }
            std::fs::create_dir_all(build_dir.join("src"))?;

            // Copy entry file
            std::fs::copy(entry_file, build_dir.join("app.js"))?;

            // Write templates
            // We need to determine the path to JetCrab library
            // For development, we assume it's a sibling. For release, we might want to use git or registry.
            // Here we prioritize JETCRAB_PATH env var, then sibling directory.
            let current_dir = std::env::current_dir()?;
            let jetcrab_path = std::env::var("JETCRAB_PATH")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|_| {
                    // Try to find JetCrab relative to cpm executable or current dir?
                    // Assuming we are in a workspace:
                    // ../JetCrab
                    current_dir.join("../JetCrab").canonicalize().unwrap_or(current_dir.join("../JetCrab"))
                });
            
            let jetcrab_path_str = jetcrab_path.display().to_string().replace("\\", "/");

            std::fs::write(
                build_dir.join("src/main.rs"), 
                cpm::templates::STANDALONE_TEMPLATE
            )?;

            let cargo_toml = cpm::templates::CARGO_TOML_TEMPLATE
                .replace("standalone-app", output_name.trim_end_matches(".exe"))
                .replace("JETCRAB_PATH", &jetcrab_path_str);

            std::fs::write(build_dir.join("Cargo.toml"), cargo_toml)?;

            // Run cargo build
            let status = std::process::Command::new("cargo")
                .arg("build")
                .arg("--release")
                .current_dir(build_dir)
                .status()?;

            if !status.success() {
                 return Err(CliError::ExecutionError {
                     command: "cargo build (bundling)".to_string(),
                     message: "Bundling failed".to_string(),
                 });
            }

            // Move binary
            let binary_name = if cfg!(windows) {
                format!("{}.exe", output_name.trim_end_matches(".exe"))
            } else {
                output_name.to_string()
            };

            let target_path = build_dir.join("target/release").join(&binary_name);
            if target_path.exists() {
                std::fs::copy(&target_path, &binary_name)?;
                eprintln!("✨ Success! Standalone binary created: {}", binary_name);
            } else {
                return Err(CliError::InternalError {
                    message: "Binary not found after build".into(),
                });
            }
            
            // Clean up
            // std::fs::remove_dir_all(build_dir)?; // Keep for debugging for now or if user wants to see it
            

        }

        Ok(())
    }
}


impl CliCommand for DevCommand {
    fn name(&self) -> &'static str {
        "dev"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("dev")
            .about("Start development server")
            .arg(clap::Arg::new("watch").short('w').long("watch").help("Watch for changes and reload").action(clap::ArgAction::SetTrue))
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        let watch = matches.get_flag("watch");
        eprintln!("🚀 Starting development server...");

        let js_entry = if std::path::Path::new("js/index.js").exists() {
            "js/index.js"
        } else if std::path::Path::new("index.js").exists() {
            "index.js"
        } else {
            eprintln!("❌ Error: No JavaScript entry point found");
            eprintln!("💡 Tip: Run 'cpm init' to create a new project or ensure index.js exists");
            return Err(CliError::FileOperationError {
                operation: "read JavaScript entry point".to_string(),
                path: "index.js".to_string(),
                message: "File not found".to_string(),
            });
        };

        eprintln!("🔍 Looking for JavaScript runtime...");

        if watch {
            let npm_cmd = if cfg!(target_os = "windows") { "npm.cmd" } else { "npm" };
            let nodemon_ok = std::process::Command::new(npm_cmd)
                .args(["exec", "nodemon", "--version"])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
            if nodemon_ok {
                eprintln!("Watching with nodemon...");
                let runner = if std::process::Command::new("jetcrab").arg("--version").output().is_ok() {
                    "jetcrab run"
                } else {
                    "node"
                };
                let status = std::process::Command::new(npm_cmd)
                    .args(["exec", "nodemon", "--exec", runner, js_entry])
                    .status()?;
                if !status.success() {
                    return Err(CliError::ExecutionError {
                        command: "nodemon".to_string(),
                        message: "nodemon failed".to_string(),
                    });
                }
                return Ok(());
            }
        }

        let jetcrab_available = std::process::Command::new("jetcrab")
            .arg("--version")
            .output()
            .is_ok();

        if jetcrab_available {
            eprintln!("🦀 Using JetCrab runtime...");
            let mut child = std::process::Command::new("jetcrab")
                .arg("run")
                .arg(js_entry)
                .spawn()?;

            child.wait()?;
        } else {
            eprintln!("🟨 Using Node.js runtime...");
            let mut child = std::process::Command::new("node").arg(js_entry).spawn()?;

            child.wait()?;
        }

        Ok(())
    }
}

impl CliCommand for RunCommand {
    fn name(&self) -> &'static str {
        "run"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("run")
            .about("Run a script or file")
            .arg(
                clap::Arg::new("script")
                    .help("Script name or file path")
                    .required(true)
                    .index(1),
            )
            .arg(
                clap::Arg::new("args")
                    .help("Arguments to pass to the script")
                    .num_args(0..)
                    .last(true),
            )
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        let script = matches.get_one::<String>("script").unwrap();
        let args: Vec<&String> = matches
            .get_many::<String>("args")
            .unwrap_or_default()
            .collect();

        eprintln!("🚀 Running: {}", script);

        // 1. Try to run as a package.json script via npm
        if std::path::Path::new("package.json").exists() {
             if let Ok(content) = std::fs::read_to_string("package.json") {
                 if let Ok(package_json) = serde_json::from_str::<serde_json::Value>(&content) {
                     if let Some(scripts) = package_json.get("scripts").and_then(|s| s.as_object()) {
                         if scripts.contains_key(script) {
                             eprintln!("📜 Found npm script '{}'", script);
                             
                             let npm_cmd = if cfg!(target_os = "windows") { "npm.cmd" } else { "npm" };
                             let mut cmd = std::process::Command::new(npm_cmd);
                             cmd.arg("run").arg(script);
                             if !args.is_empty() {
                                 cmd.arg("--").args(args);
                             }
                             
                             let status = cmd.status()?;
                             if !status.success() {
                                 return Err(CliError::ExecutionError {
                                     command: format!("npm run {}", script),
                                     message: "Script failed".to_string(),
                                 });
                             }
                             return Ok(());
                         }
                     }
                 }
             }
        }

        // 2. Try to run as a JS file with JetCrab
        let script_path = std::path::Path::new(script);
        if script_path.exists() && (script.ends_with(".js") || script.ends_with(".rs")) {
             eprintln!("🦀 Executing file with JetCrab...");
             let mut cmd = std::process::Command::new("jetcrab");
             cmd.arg("run").arg(script);
             cmd.args(args);
             
             let status = cmd.status()?;
             if !status.success() {
                 return Err(CliError::ExecutionError {
                     command: format!("jetcrab run {}", script),
                     message: "Execution failed".to_string(),
                 });
             }
             return Ok(());
        }
        
        eprintln!("❌ Could not find script or file: {}", script);
        Ok(())
    }
}


impl CliCommand for TestCommand {
    fn name(&self) -> &'static str {
        "test"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("test").about("Run tests")
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        eprintln!("🧪 Running tests...");

        // Check if we're in a JavaScript project
        if std::path::Path::new("package.json").exists() {
            eprintln!("🟨 Running JavaScript tests...");

            // Try npm.cmd on Windows first, then npm
            let npm_cmd = if cfg!(target_os = "windows") {
                "npm.cmd"
            } else {
                "npm"
            };

            let npm_output = std::process::Command::new(npm_cmd).arg("test").output()?;

            if !npm_output.status.success() {
                eprintln!("⚠️  No test script found in package.json");
                eprintln!("💡 Add a test script to package.json or run tests manually");
            } else {
                eprintln!("✅ JavaScript tests completed!");
            }
        }

        // Check if we're in a Rust project
        if std::path::Path::new("Cargo.toml").exists() {
            eprintln!("🦀 Running Rust tests...");
            let cargo_output = std::process::Command::new("cargo").arg("test").output()?;

            if !cargo_output.status.success() {
                return Err(CliError::ExecutionError {
                    command: "cargo test".to_string(),
                    message: String::from_utf8_lossy(&cargo_output.stderr).to_string(),
                });
            }
            eprintln!("✅ Rust tests completed!");
        }

        Ok(())
    }
}

fn main() {
    let app = CliApp::new("cpm", env!("CARGO_PKG_VERSION"))
        .description("A modern package manager for JavaScript and Rust")
        .add_command(Box::new(InitCommand::new()))
        .add_command(Box::new(AddRustCommand::new()))
        .add_command(Box::new(RemoveRustCommand::new()))
        .add_command(Box::new(RustStatusCommand))
        .add_command(Box::new(NpxCommand::new()))
        .add_command(Box::new(AddCommand))
        .add_command(Box::new(RemoveCommand))
        .add_command(Box::new(LockCommand))
        .add_command(Box::new(WorkspaceCommand))
        .add_command(Box::new(PublishCommand))
        .add_command(Box::new(InstallCommand))
        .add_command(Box::new(BuildCommand))
        .add_command(Box::new(DevCommand))
        .add_command(Box::new(TestCommand))
        .add_command(Box::new(RunCommand));

    if should_trigger_easter_egg() {
        show_walking_claw();
    }

    if let Err(e) = app.run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
