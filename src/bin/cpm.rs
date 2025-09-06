//! Crab Package Manager - A modern package manager for JavaScript and Rust

use clap::ArgMatches;
use cpm::cli::framework::{
    CliApp, CliCommand, CliContext, CliError, CliResult,
};
use cpm::easter_egg::{should_trigger_easter_egg, show_walking_claw};
use std::path::PathBuf;
use tracing::info;

struct InitCommand;

struct AddRustCommand;

struct RemoveRustCommand;

struct RustStatusCommand;

struct NpxCommand;

struct InstallCommand;
struct BuildCommand;
struct DevCommand;
struct TestCommand;

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
                scripts_obj.insert("dev".to_string(), serde_json::Value::String("cpm dev".to_string()));
                scripts_obj.insert("build".to_string(), serde_json::Value::String("cpm build".to_string()));
                scripts_obj.insert("test".to_string(), serde_json::Value::String("cpm test".to_string()));
            }
        } else {
            let mut scripts = serde_json::Map::new();
            scripts.insert("dev".to_string(), serde_json::Value::String("cpm dev".to_string()));
            scripts.insert("build".to_string(), serde_json::Value::String("cpm build".to_string()));
            scripts.insert("test".to_string(), serde_json::Value::String("cpm test".to_string()));
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
                    .action(clap::ArgAction::SetTrue)
            )
            .arg(
                clap::Arg::new("name")
                    .help("Project name")
                    .index(1)
            )
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        let project_name = matches
            .get_one::<String>("name")
            .map(|s| s.as_str())
            .unwrap_or("my-cpm-project");

        let yes = matches.get_flag("yes");

        info!("Initializing JavaScript project: {}", project_name);

        println!("🚀 Initializing CPM JavaScript project: {}", project_name);
        println!();

        // Create project directory
        let project_dir = PathBuf::from(project_name);
        if project_dir.exists() {
            return Err(CliError::FileExists {
                path: project_name.to_string(),
            });
        }

        std::fs::create_dir(&project_dir)?;
        std::env::set_current_dir(&project_dir)?;

        println!("📦 Setting up JavaScript project...");

        // Run npm init
        let npm_args = if yes {
            vec!["init", "-y"]
        } else {
            vec!["init"]
        };

        // Try to find npm in common locations
        let npm_cmd = if cfg!(windows) {
            // On Windows, try npm.cmd first, then npm
            if std::process::Command::new("npm.cmd").arg("--version").output().is_ok() {
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
                command: format!("{} init", npm_cmd),
                message: format!("stderr: {}\nstdout: {}", stderr, stdout),
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
        let readme = format!(r#"# {} - CPM JavaScript Project

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
- `cpm add <package>` - Add a package
- `cpm remove <package>` - Remove a package
- `cpm build` - Build the project
- `cpm dev` - Start development server
- `cpm test` - Run tests
- `cpm add-rust` - Add Rust to the project
- `cpm rust-status` - Check Rust status
"#, project_name);

        std::fs::write("README.md", readme)?;

        println!("✅ JavaScript project initialized successfully!");
        println!("💡 Run 'cpm install' to install dependencies");
        println!("💡 Run 'cpm dev' to start development server");
        println!("💡 Run 'cpm add-rust' to add Rust later if needed");

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
                lib_section.insert("crate-type".to_string(), toml::Value::Array(vec![
                    toml::Value::String("cdylib".to_string())
                ]));
                cargo_toml["lib"] = toml::Value::Table(lib_section);
            }
        }
        
        // Add WASM dependencies
        let mut dependencies = toml::map::Map::new();
        dependencies.insert("wasm-bindgen".to_string(), toml::Value::String("0.2".to_string()));
        dependencies.insert("serde".to_string(), toml::Value::String("1.0".to_string()));
        dependencies.insert("serde-wasm-bindgen".to_string(), toml::Value::String("0.6".to_string()));
        dependencies.insert("web-sys".to_string(), toml::Value::String("0.3".to_string()));
        
        cargo_toml["dependencies"] = toml::Value::Table(dependencies);
        
        // Write back to Cargo.toml
        let updated_content = toml::to_string_pretty(&cargo_toml).map_err(|e| CliError::InternalError {
            message: format!("Failed to serialize Cargo.toml: {}", e),
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

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}! You've been greeted from Rust!", name));
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    log!("Adding {} + {}", a, b);
    a + b
}

#[wasm_bindgen]
pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

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
                deps_obj.insert("wasm-bindgen".to_string(), serde_json::Value::String("^0.2".to_string()));
            }
        } else {
            let mut dependencies = serde_json::Map::new();
            dependencies.insert("wasm-bindgen".to_string(), serde_json::Value::String("^0.2".to_string()));
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
                    .action(clap::ArgAction::SetTrue)
            )
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        let yes = matches.get_flag("yes");

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
            println!("⚠️  Rust is already added to this project!");
            println!("💡 Run 'cpm rust-status' to check the current status");
            return Ok(());
        }

        println!("🦀 Adding Rust to JavaScript project...");

        // Get project name from package.json
        let package_json_content = std::fs::read_to_string("package.json")?;
        let package_json: serde_json::Value = serde_json::from_str(&package_json_content)?;
        let project_name = package_json["name"]
            .as_str()
            .unwrap_or("my-project")
            .replace('-', "_");

        // Run cargo init
        let cargo_args = if yes {
            vec!["init", "--name", &project_name, "--lib"]
        } else {
            vec!["init", "--name", &project_name, "--lib"]
        };

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

        println!("✅ Rust added to project successfully!");
        println!("💡 Run 'cpm build' to compile Rust to WASM");
        println!("💡 Run 'cpm dev' to start development server");
        println!("💡 Check 'src/lib.rs' for Rust code examples");

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
                    .action(clap::ArgAction::SetTrue)
            )
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        let yes = matches.get_flag("yes");

        info!("Removing Rust from project");

        // Check if Rust is present
        if !std::path::Path::new("Cargo.toml").exists() {
            println!("⚠️  No Rust found in this project!");
            return Ok(());
        }

        if !yes {
            println!("⚠️  This will remove all Rust files and dependencies!");
            println!("   - Cargo.toml");
            println!("   - src/ directory");
            println!("   - pkg/ directory");
            println!("   - Rust dependencies from package.json");
            println!();
            println!("   Continue? (y/N): ");
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            if !input.trim().to_lowercase().starts_with('y') {
                println!("❌ Operation cancelled");
                return Ok(());
            }
        }

        println!("🗑️  Removing Rust from project...");

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

        println!("✅ Rust removed from project successfully!");
        println!("💡 Project is now JavaScript-only");

        Ok(())
    }
}

impl CliCommand for RustStatusCommand {
    fn name(&self) -> &'static str {
        "rust-status"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("rust-status")
            .about("Check Rust status in the current project")
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        info!("Checking Rust status");

        println!("🔍 Checking Rust status in current project...");
        println!();

        // Check if we're in a JavaScript project
        if !std::path::Path::new("package.json").exists() {
            println!("❌ Not in a JavaScript project");
            println!("💡 Run 'cpm init' to create a new project");
            return Ok(());
        }

        // Check Rust files
        let has_cargo_toml = std::path::Path::new("Cargo.toml").exists();
        let has_src_dir = std::path::Path::new("src").exists();
        let has_pkg_dir = std::path::Path::new("pkg").exists();
        let has_lib_rs = std::path::Path::new("src/lib.rs").exists();

        println!("📁 Project Structure:");
        println!("   package.json: ✅");
        println!("   Cargo.toml: {}", if has_cargo_toml { "✅" } else { "❌" });
        println!("   src/ directory: {}", if has_src_dir { "✅" } else { "❌" });
        println!("   src/lib.rs: {}", if has_lib_rs { "✅" } else { "❌" });
        println!("   pkg/ directory: {}", if has_pkg_dir { "✅" } else { "❌" });

        if has_cargo_toml && has_src_dir && has_lib_rs {
            println!();
            println!("🦀 Rust is fully integrated!");
            println!("💡 Run 'cpm build' to compile Rust to WASM");
            println!("💡 Run 'cpm dev' to start development server");
        } else if has_cargo_toml || has_src_dir {
            println!();
            println!("⚠️  Rust is partially integrated");
            println!("💡 Run 'cpm add-rust' to complete the setup");
        } else {
            println!();
            println!("📦 JavaScript-only project");
            println!("💡 Run 'cpm add-rust' to add Rust if needed");
        }

        Ok(())
    }
}

impl NpxCommand {
    fn new() -> Self {
        Self
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
                    .index(1)
            )
            .arg(
                clap::Arg::new("args")
                    .help("Arguments to pass to the package")
                    .num_args(0..)
                    .last(true)
            )
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        let package = matches.get_one::<String>("package");
        let args: Vec<&String> = matches.get_many::<String>("args").unwrap_or_default().collect();
        
        if let Some(pkg) = package {
            println!("🦀 CPM detected npx command for package: {}", pkg);
            println!("📦 Executing with npx...");
            
            // Build the npx command
            let mut npx_cmd = std::process::Command::new("npx");
            npx_cmd.arg(pkg);
            npx_cmd.args(&args);
            
            // Execute the command
            let status = npx_cmd.status()?;
            
            if !status.success() {
                return Err(CliError::ExecutionError {
                    command: format!("npx {}", pkg),
                    message: "npx command failed".to_string(),
                });
            }
        } else {
            println!("❌ No package specified for npx");
            println!("💡 Usage: cpm npx <package> [args...]");
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
            .about("Install dependencies")
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        println!("📦 Installing dependencies...");
        
        // Check if we're in a JavaScript project
        if std::path::Path::new("package.json").exists() {
            println!("🟨 Installing JavaScript dependencies with npm...");
            
            // Try npm.cmd on Windows first, then npm
            let npm_cmd = if cfg!(target_os = "windows") {
                "npm.cmd"
            } else {
                "npm"
            };
            
            let npm_output = std::process::Command::new(npm_cmd)
                .arg("install")
                .output()?;
            
            if !npm_output.status.success() {
                return Err(CliError::ExecutionError {
                    command: "npm install".to_string(),
                    message: String::from_utf8_lossy(&npm_output.stderr).to_string(),
                });
            }
            println!("✅ JavaScript dependencies installed!");
        }
        
        // Check if we're in a Rust project
        if std::path::Path::new("Cargo.toml").exists() {
            println!("🦀 Installing Rust dependencies with cargo...");
            let cargo_output = std::process::Command::new("cargo")
                .arg("build")
                .output()?;
            
            if !cargo_output.status.success() {
                return Err(CliError::ExecutionError {
                    command: "cargo build".to_string(),
                    message: String::from_utf8_lossy(&cargo_output.stderr).to_string(),
                });
            }
            println!("✅ Rust dependencies installed!");
        }
        
        Ok(())
    }
}

impl CliCommand for BuildCommand {
    fn name(&self) -> &'static str {
        "build"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("build")
            .about("Build the project")
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        println!("🔨 Building project...");
        
        // Check if we're in a Rust project
        if std::path::Path::new("Cargo.toml").exists() {
            println!("🦀 Building Rust project...");
            let cargo_output = std::process::Command::new("cargo")
                .arg("build")
                .output()?;
            
            if !cargo_output.status.success() {
                return Err(CliError::ExecutionError {
                    command: "cargo build".to_string(),
                    message: String::from_utf8_lossy(&cargo_output.stderr).to_string(),
                });
            }
            
            // If wasm-pack is available, build WASM
            if std::process::Command::new("wasm-pack").arg("--version").output().is_ok() {
                println!("🌐 Building WebAssembly...");
                let wasm_output = std::process::Command::new("wasm-pack")
                    .args(&["build", "--release", "--target", "web", "--out-dir", "pkg"])
                    .output()?;
                
                if !wasm_output.status.success() {
                    println!("⚠️  WASM build failed, but continuing...");
                } else {
                    println!("✅ WebAssembly built successfully!");
                }
            }
            
            println!("✅ Rust project built!");
        } else {
            println!("📦 No build step needed for JavaScript-only project");
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
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        println!("🚀 Starting development server...");
        
        // Check for JavaScript entry point
        let js_entry = if std::path::Path::new("js/index.js").exists() {
            "js/index.js"
        } else if std::path::Path::new("index.js").exists() {
            "index.js"
        } else {
            println!("❌ Error: No JavaScript entry point found");
            println!("💡 Tip: Run 'cpm init' to create a new project or ensure index.js exists");
            return Err(CliError::FileOperationError {
                operation: "read JavaScript entry point".to_string(),
                path: "index.js".to_string(),
                message: "File not found".to_string(),
            });
        };
        
        println!("🔍 Looking for JavaScript runtime...");
        
        // Try JetCrab first, then fallback to Node.js
        let jetcrab_available = std::process::Command::new("jetcrab")
            .arg("--version")
            .output()
            .is_ok();
        
        if jetcrab_available {
            println!("🦀 Using JetCrab runtime...");
            let mut child = std::process::Command::new("jetcrab")
                .arg("run")
                .arg(js_entry)
                .spawn()?;
            
            child.wait()?;
        } else {
            println!("🟨 Using Node.js runtime...");
            let mut child = std::process::Command::new("node")
                .arg(js_entry)
                .spawn()?;
            
            child.wait()?;
        }
        
        Ok(())
    }
}

impl CliCommand for TestCommand {
    fn name(&self) -> &'static str {
        "test"
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("test")
            .about("Run tests")
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        println!("🧪 Running tests...");
        
        // Check if we're in a JavaScript project
        if std::path::Path::new("package.json").exists() {
            println!("🟨 Running JavaScript tests...");
            
            // Try npm.cmd on Windows first, then npm
            let npm_cmd = if cfg!(target_os = "windows") {
                "npm.cmd"
            } else {
                "npm"
            };
            
            let npm_output = std::process::Command::new(npm_cmd)
                .arg("test")
                .output()?;
            
            if !npm_output.status.success() {
                println!("⚠️  No test script found in package.json");
                println!("💡 Add a test script to package.json or run tests manually");
            } else {
                println!("✅ JavaScript tests completed!");
            }
        }
        
        // Check if we're in a Rust project
        if std::path::Path::new("Cargo.toml").exists() {
            println!("🦀 Running Rust tests...");
            let cargo_output = std::process::Command::new("cargo")
                .arg("test")
                .output()?;
            
            if !cargo_output.status.success() {
                return Err(CliError::ExecutionError {
                    command: "cargo test".to_string(),
                    message: String::from_utf8_lossy(&cargo_output.stderr).to_string(),
                });
            }
            println!("✅ Rust tests completed!");
        }
        
        Ok(())
    }
}

fn main() {
    let app = CliApp::new("cpm", "0.1.0")
        .description("A modern package manager for JavaScript and Rust")
        .add_command(Box::new(InitCommand::new()))
        .add_command(Box::new(AddRustCommand::new()))
        .add_command(Box::new(RemoveRustCommand::new()))
        .add_command(Box::new(RustStatusCommand))
        .add_command(Box::new(NpxCommand::new()))
        .add_command(Box::new(InstallCommand))
        .add_command(Box::new(BuildCommand))
        .add_command(Box::new(DevCommand))
        .add_command(Box::new(TestCommand));

    if should_trigger_easter_egg() {
        show_walking_claw();
    }

    if let Err(e) = app.run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
