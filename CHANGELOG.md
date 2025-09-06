# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/0.4.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2025-09-02

### Added
- **CPM (Crab Package Manager)**: Lightweight package manager for JavaScript and Rust
- **JavaScript-First Approach**: Optimized for JavaScript projects with optional Rust
- **Intelligent Wrapper**: Delegates to npm and cargo for proven reliability
- **Core Commands**: init, install, build, dev, test, npx, add-rust, remove-rust, rust-status
- **Cross-Platform Support**: Windows, macOS, and Linux compatibility
- **WebAssembly Integration**: Automatic Rust to WASM compilation with wasm-pack
- **Runtime Detection**: Automatically uses JetCrab or Node.js for development
- **NPX Support**: Execute packages using npx through CPM

### Changed
- **Project Focus**: Transformed from JavaScript runtime to package manager
- **Architecture**: Simplified to lightweight wrapper around existing tools
- **Documentation**: Updated to reflect CPM's role as package manager
- **Project Structure**: Streamlined for package management focus

### Removed
- **Complex Runtime**: Removed Boa integration and custom JavaScript engine
- **Heavy Dependencies**: Removed unnecessary runtime and tooling dependencies
- **Legacy Features**: Removed complex package management features

### Fixed
- **Windows Compatibility**: Fixed npm execution on Windows with npm.cmd
- **Command Structure**: Simplified CLI interface for better usability
- **Documentation**: Updated all documentation to reflect CPM functionality

## [0.3.0] - 2025-09-01

### Added
- Initial Boa integration
- Basic runtime structure
- CLI interface foundation

### Changed
- Started migration to Boa engine
- Updated project structure

## [0.2.0] - 2025-08-31

### Added
- Comprehensive E2E test coverage system
- Advanced memory management with SpaceCoordinator
- Enhanced garbage collection with write barriers
- Complete documentation architecture
- ECMAScript 2024 compliance tests
- Performance benchmarks and stress tests
- Advanced examples (analytics, fibonacci, object handling)
- CI/CD pipeline with comprehensive checks
- Development scripts and automation tools

### Changed
- Improved error handling and recovery mechanisms
- Enhanced performance optimizations
- Updated documentation structure
- Refined API design

### Fixed
- Memory leak issues in garbage collection
- Performance bottlenecks in bytecode execution
- Error handling edge cases
- Documentation inconsistencies

## [0.1.0] - 2025-08-30

### Added
- Initial project structure
- Basic JavaScript engine implementation
- Core modules (lexer, parser, AST, VM)
- Basic test suite
- Initial documentation

---

**Note**: Version 0.4.0 represents a major architectural shift from a custom JavaScript engine to a Boa-based runtime, focusing on runtime services and developer experience rather than engine implementation.