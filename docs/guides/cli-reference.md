# CPM CLI Reference

This document provides a comprehensive reference for the CPM (Crab Package Manager) command-line interface.

## Overview

CPM provides the following CLI tools:
- **`cpm`**: Package manager for JavaScript and Rust packages
- **`jetcrab`**: Main JavaScript runtime and execution tool (external dependency)

## jetcrab Command

The main JetCrab command for running JavaScript code.

### Usage
```bash
jetcrab [OPTIONS] <COMMAND>
```

### Global Options
- `-h, --help`: Show help information
- `-V, --version`: Show version information
- `-v, --verbose`: Enable verbose output
- `--log-level <LEVEL>`: Set logging level (error, warn, info, debug, trace)

### Commands

#### `run <FILE>`
Run a JavaScript file.

```bash
jetcrab run script.js
jetcrab run --args "arg1" "arg2" script.js
```

**Options:**
- `--args <ARGS>...`: Command line arguments to pass to the script
- `--timeout <SECONDS>`: Set execution timeout
- `--max-memory <MB>`: Set maximum memory usage

**Examples:**
```bash
# Run a simple script
jetcrab run hello.js

# Run with arguments
jetcrab run --args "world" "universe" greet.js

# Run with timeout
jetcrab run --timeout 30 long-running.js
```

#### `eval <CODE>`
Evaluate JavaScript code directly.

```bash
jetcrab eval "console.log('Hello, World!')"
jetcrab eval "2 + 2"
```

**Options:**
- `--timeout <SECONDS>`: Set execution timeout
- `--max-memory <MB>`: Set maximum memory usage

**Examples:**
```bash
# Simple expression
jetcrab eval "42 + 8"

# Complex code
jetcrab eval "const fs = require('fs'); console.log(fs.readFileSync('package.json', 'utf8'))"
```

#### `repl`
Start an interactive REPL (Read-Eval-Print Loop).

```bash
jetcrab repl
```

**Options:**
- `--history <FILE>`: Specify history file location
- `--no-history`: Disable command history
- `--prompt <TEXT>`: Customize the prompt

**Examples:**
```bash
# Start REPL
jetcrab repl

# Start REPL with custom history
jetcrab repl --history ~/.jetcrab_history

# Start REPL with custom prompt
jetcrab repl --prompt "jetcrab> "
```

#### `dev`
Start development mode with file watching.

```bash
jetcrab dev [FILE]
```

**Options:**
- `--watch`: Watch for file changes
- `--port <PORT>`: Set development server port
- `--host <HOST>`: Set development server host
- `--open`: Open browser automatically

**Examples:**
```bash
# Start dev server
jetcrab dev

# Start dev server with file watching
jetcrab dev --watch

# Start dev server on specific port
jetcrab dev --port 3000
```

#### `build`
Build the project for production.

```bash
jetcrab build
```

**Options:**
- `--target <TARGET>`: Build for specific target
- `--release`: Build in release mode
- `--optimize`: Enable optimizations

**Examples:**
```bash
# Build project
jetcrab build

# Build for release
jetcrab build --release

# Build for specific target
jetcrab build --target x86_64-unknown-linux-gnu
```

## cpm Command

The CPM (Crab Package Manager) for managing JavaScript and Rust dependencies.

### Usage
```bash
cpm [OPTIONS] <COMMAND>
```

### Global Options
- `-h, --help`: Show help information
- `-V, --version`: Show version information

### Commands

#### `init [NAME] [-y]`
Initialize a new JavaScript project.

```bash
cpm init my-project -y
```

**Options:**
- `-y, --yes`: Use default values without prompting

**Examples:**
```bash
# Initialize with default values
cpm init my-project -y

# Initialize with prompts
cpm init my-project
```

#### `install`
Install dependencies for the current project (npm + cargo).

```bash
cpm install
```

**Examples:**
```bash
# Install all dependencies
cpm install
```

#### `build`
Build the project (cargo + wasm-pack if available).

```bash
cpm build
```

**Examples:**
```bash
# Build project
cpm build
```

#### `dev`
Start development server (JetCrab first, then Node.js).

```bash
cpm dev
```

**Examples:**
```bash
# Start development server
cpm dev
```

#### `test`
Run tests for the current project (npm + cargo).

```bash
cpm test
```

**Examples:**
```bash
# Run tests
cpm test
```

#### `npx <package> [args...]`
Execute packages using npx.

```bash
cpm npx create-react-app my-app
cpm npx eslint --init
```

**Examples:**
```bash
# Create React app
cpm npx create-react-app my-app

# Initialize ESLint
cpm npx eslint --init

# Run any npx command
cpm npx jest --init
```

#### `add-rust [-y]`
Add Rust to an existing JavaScript project.

```bash
cpm add-rust -y
```

**Options:**
- `-y, --yes`: Use default values without prompting

**Examples:**
```bash
# Add Rust with prompts
cpm add-rust

# Add Rust with defaults
cpm add-rust -y
```

#### `remove-rust [-y]`
Remove Rust from a project.

```bash
cpm remove-rust -y
```

**Options:**
- `-y, --yes`: Skip confirmation prompts

**Examples:**
```bash
# Remove Rust with confirmation
cpm remove-rust

# Remove Rust without confirmation
cpm remove-rust -y
```

#### `rust-status`
Check Rust status in the current project.

```bash
cpm rust-status
```

**Examples:**
```bash
# Check Rust integration status
cpm rust-status
```

## Configuration

### jetcrab Configuration
JetCrab can be configured using environment variables or a configuration file.

**Environment Variables:**
- `JETCRAB_LOG_LEVEL`: Set logging level
- `JETCRAB_MAX_MEMORY`: Set maximum memory usage
- `JETCRAB_TIMEOUT`: Set default timeout

**Configuration File:**
Create a `jetcrab.toml` file in your project root:

```toml
[api]
enabled_apis = ["console", "process", "fs", "http"]
disabled_apis = []
experimental_apis = ["worker_threads"]

[performance]
enable_lazy_loading = true
api_timeout_ms = 5000

[logging]
level = "info"
format = "json"

[development]
enable_source_maps = true
enable_hot_reload = true
```

### cpm Configuration
CPM uses `package.json` for JavaScript projects and `Cargo.toml` for Rust projects:

**package.json:**
```json
{
  "name": "my-project",
  "version": "1.0.0",
  "description": "My CPM project",
  "main": "index.js",
  "scripts": {
    "build": "cpm build",
    "dev": "cpm dev",
    "test": "cpm test"
  },
  "dependencies": {
    "lodash": "^4.17.21"
  },
  "devDependencies": {
    "jest": "^29.0.0"
  }
}
```

**Cargo.toml (for hybrid projects):**
```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
web-sys = "0.3"
```

## Examples

### Basic Usage
```bash
# Run a JavaScript file
jetcrab run app.js

# Start interactive REPL
jetcrab repl

# Evaluate code directly
jetcrab eval "console.log('Hello, World!')"
```

### Package Management
```bash
# Initialize new project
cpm init my-project -y
cd my-project

# Install dependencies
cpm install

# Add Rust to project
cpm add-rust -y

# Build project
cpm build

# Run development server
cpm dev
```

### Development Workflow
```bash
# Start development server
cpm dev

# Run tests
cpm test

# Build project
cpm build

# Use npx commands
cpm npx create-react-app my-app
cpm npx eslint --init
```

## Troubleshooting

### Common Issues

#### Command Not Found
```bash
# Check if CPM is installed
which cpm

# Check installation
cargo install --list | grep cpm

# Check if JetCrab is installed
which jetcrab
```

#### Permission Denied
```bash
# Check file permissions
ls -la script.js

# Make file executable
chmod +x script.js
```

#### Memory Issues
```bash
# Run with memory limit
jetcrab run --max-memory 512 script.js

# Check system memory
free -h
```

#### Timeout Issues
```bash
# Run with longer timeout
jetcrab run --timeout 60 long-script.js

# Check script for infinite loops
```

## Resources

- **[JetCrab Guide](jetcrab-guide.md)** - Comprehensive usage guide
- **[API Reference](api-reference.md)** - API documentation
- **[Examples](examples.md)** - Code examples
- **[Contributing Guide](../contributing.md)** - How to contribute

---

**CPM CLI Reference** - Command-line interface documentation 🦀
