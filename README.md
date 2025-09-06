# CPM - Crab Package Manager
  
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Node.js](https://img.shields.io/badge/node.js-18+-green.svg)](https://nodejs.org)

> A simple and intelligent wrapper for npm and cargo

CPM (Crab Package Manager) is a lightweight package manager that acts as an intelligent wrapper around npm and cargo, providing a unified interface for JavaScript and Rust projects.

## 🦀 **Why CPM?**

- **Simple**: No complex registries or cache management
- **Fast**: Delegates to proven tools (npm, cargo)
- **Unified**: Single interface for JS and Rust projects
- **Flexible**: Add Rust to JS projects when needed
- **Lightweight**: Minimal dependencies and overhead

## 🚀 **Quick Start**

### Installation

```bash
# Install CPM
cargo install cpm

# Or build from source
git clone https://github.com/JetCrabCollab/cpm.git
cd cpm
cargo build --release
```

### Prerequisites

CPM requires the following tools to be installed:

- **Rust & Cargo**: [Install Rust](https://rustup.rs/)
- **Node.js & npm**: [Install Node.js](https://nodejs.org/)
- **JetCrab Runtime**: `cargo install jetcrab` (optional, for development)

### Basic Usage

```bash
# Initialize a JavaScript project
cpm init my-project -y

# Add Rust to existing JS project
cpm add-rust

# Check Rust status
cpm rust-status

# Remove Rust from project
cpm remove-rust
```

## 📋 **Commands**

### `cpm init [name] [-y]`
Initialize a new JavaScript project with npm.

```bash
cpm init my-app -y
```

### `cpm install`
Install dependencies for the current project (npm + cargo).

```bash
cpm install
```

### `cpm build`
Build the project (cargo + wasm-pack if available).

```bash
cpm build
```

### `cpm dev`
Start development server (JetCrab first, then Node.js).

```bash
cpm dev
```

### `cpm test`
Run tests for the current project (npm + cargo).

```bash
cpm test
```

### `cpm npx <package> [args...]`
Execute packages using npx.

```bash
cpm npx create-react-app my-app
cpm npx eslint --init
```

### `cpm add-rust [-y]`
Add Rust to an existing JavaScript project.

```bash
cpm add-rust -y
```

### `cpm remove-rust [-y]`
Remove Rust from a project.

```bash
cpm remove-rust -y
```

### `cpm rust-status`
Check Rust status in the current project.

```bash
cpm rust-status
```

## 🏗️ **Project Structure**

### JavaScript Project
```
my-project/
├── package.json
├── index.js
└── README.md
```

### Hybrid Project (after `cpm add-rust`)
```
my-project/
├── package.json
├── Cargo.toml
├── src/
│   └── lib.rs
├── pkg/           # Generated WASM
├── index.js
└── README.md
```

## 🔧 **How It Works**

CPM is designed to be simple and lightweight:

1. **JavaScript Projects**: Uses `npm` for package management
2. **Rust Integration**: Uses `cargo` for Rust dependencies
3. **WASM Compilation**: Uses `wasm-pack` for WebAssembly
4. **Development**: Uses `jetcrab` or `node` for running projects
5. **Package Execution**: Uses `npx` for running packages
6. **Intelligent Detection**: Automatically detects project type and uses appropriate tools
7. **Cross-Platform**: Works on Windows, macOS, and Linux

## 📦 **Examples**

### Create a JavaScript Project
```bash
cpm init my-js-app -y
cd my-js-app
cpm install express  # Install dependencies
cpm dev             # Runs with jetcrab or node
cpm test            # Run tests
cpm build           # Build project
```

### Add Rust to JavaScript Project
```bash
cpm add-rust -y
cpm install         # Install both JS and Rust dependencies
cpm build           # Compiles Rust to WASM
cpm dev             # Runs hybrid project
cpm test            # Run both JS and Rust tests
```

### Using npx with CPM
```bash
cpm npx create-react-app my-react-app
cpm npx eslint --init
cpm npx jest --init
```

### Check Project Status
```bash
cpm rust-status
# Output:
# 📁 Project Structure:
#    package.json: ✅
#    Cargo.toml: ✅
#    src/ directory: ✅
#    src/lib.rs: ✅
#    pkg/ directory: ✅
# 
# 🦀 Rust is fully integrated!
```

## 🎯 **When to Use CPM**

### Use CPM when:
- ✅ You want a simple package manager
- ✅ You work with JavaScript projects
- ✅ You need Rust performance occasionally
- ✅ You want to avoid complex tooling

### Don't use CPM when:
- ❌ You need advanced package management features
- ❌ You work exclusively with Rust
- ❌ You need custom registries or caching

## 🔄 **Migration from Other Tools**

### From npm
```bash
# Your existing npm project works with CPM
cpm init . -y  # Adds CPM scripts to package.json
```

### From cargo
```bash
# Cargo projects can be integrated with CPM
cpm add-rust -y  # Adds JavaScript support
```

## 🤝 **Contributing**

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
```bash
git clone https://github.com/JetCrabCollab/cpm.git
cd cpm
cargo build
cargo test
```

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 **Acknowledgments**

- [npm](https://www.npmjs.com/) - JavaScript package manager
- [Cargo](https://crates.io/) - Rust package manager
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) - WebAssembly packaging
- [JetCrab](https://jetcrab.dev/) - JavaScript runtime

## 📞 **Support**

- **Documentation**: [docs.jetcrab.dev/cpm](https://docs.jetcrab.dev/cpm)
- **Issues**: [GitHub Issues](https://github.com/JetCrabCollab/cpm/issues)
- **Discussions**: [GitHub Discussions](https://github.com/JetCrabCollab/cpm/discussions)

---

**Part of the JetCrab ecosystem** - [jetcrab.dev](https://jetcrab.dev)