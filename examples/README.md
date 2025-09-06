# CPM Examples

This directory contains practical examples demonstrating when to use pure JavaScript vs Rust + JavaScript in different scenarios.

## 📁 **Example Projects**

### **1. Pure JavaScript Examples**
- `simple-crud/` - Basic CRUD operations (JavaScript only)
- `ui-dashboard/` - Interactive dashboard (JavaScript only)
- `prototype-app/` - Quick prototyping (JavaScript only)

### **2. Hybrid Rust + JavaScript Examples**
- `image-processor/` - Image processing with Rust performance
- `crypto-wallet/` - Secure operations with Rust
- `data-analyzer/` - Heavy data processing with Rust
- `game-engine/` - 2D game with Rust physics

### **3. Comparison Examples**
- `performance-comparison/` - Side-by-side performance tests
- `security-comparison/` - Security features comparison

## 🎯 **When to Use What?**

### **Use Pure JavaScript when:**
- ✅ Simple CRUD operations
- ✅ UI/UX focused applications
- ✅ Quick prototyping
- ✅ Small team, tight deadlines
- ✅ Existing JavaScript ecosystem is sufficient

### **Use Rust + JavaScript when:**
- ✅ Performance-critical algorithms
- ✅ Heavy data processing
- ✅ Security-sensitive operations
- ✅ Access to native APIs
- ✅ Complex mathematical computations

## Getting Started

1. Navigate to any example directory:
   ```bash
   cd examples/basic-js-project
   ```

2. Install dependencies:
   ```bash
   cpm install
   ```

3. Start development server:
   ```bash
   cpm dev
   ```

4. Build the project:
   ```bash
   cpm build
   ```

## Project Structure

Each example includes:
- `package.json` - JavaScript dependencies and scripts
- `Cargo.toml` - Rust dependencies (for hybrid projects)
- `src/` - Source code directory
- `index.js` - Main JavaScript entry point
- `pkg/` - Generated WebAssembly output (after build)

## Commands

- `cpm init` - Initialize a new project
- `cpm install` - Install all dependencies
- `cpm add <package>` - Add a new package
- `cpm remove <package>` - Remove a package
- `cpm build` - Build the project
- `cpm dev` - Start development server
- `cpm test` - Run tests
