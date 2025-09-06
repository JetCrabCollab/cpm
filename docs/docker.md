# CPM Docker Guide

This guide explains how to use CPM with Docker for consistent builds and development.

## Quick Start

### Build the Docker Image

```bash
# Build the CPM Docker image
docker build -t cpm:latest .

# Or use the provided script
./scripts/docker-build.sh
# On Windows:
.\scripts\docker-build.ps1
```

### Run CPM Commands

```bash
# Show help
docker run --rm cpm:latest --help

# Initialize a new project
docker run --rm -v $(pwd):/workspace -w /workspace cpm:latest init my-project -y

# Install dependencies
docker run --rm -v $(pwd):/workspace -w /workspace cpm:latest install

# Run development server
docker run --rm -v $(pwd):/workspace -w /workspace cpm:latest dev
```

## Docker Compose

Use docker-compose for easier development:

```bash
# Run tests
docker-compose up cpm-test

# Run clippy
docker-compose up cpm-clippy

# Start development environment
docker-compose up cpm
```

## Development Workflow

### 1. Build and Test

```bash
# Build the image
make docker-build

# Run tests
make docker-test

# Run clippy
make docker-clippy
```

### 2. Interactive Development

```bash
# Start an interactive container
docker run -it --rm -v $(pwd):/workspace -w /workspace cpm:latest bash

# Inside the container, you can run any CPM commands
cpm init my-project -y
cd my-project
cpm install
cpm dev
```

### 3. CI/CD Integration

The Docker image is perfect for CI/CD pipelines:

```yaml
# Example GitHub Actions step
- name: Test with CPM
  run: |
    docker run --rm -v ${{ github.workspace }}:/workspace -w /workspace cpm:latest test
```

## Image Details

- **Base Image**: `rust:1.75-slim`
- **Node.js**: Version 20.x
- **wasm-pack**: Latest version for WebAssembly support
- **Size**: ~800MB (includes Rust toolchain, Node.js, and wasm-pack)

## Benefits of Using Docker

1. **Consistent Environment**: Same Rust, Node.js, and wasm-pack versions everywhere
2. **No Local Dependencies**: No need to install Rust, Node.js, or wasm-pack locally
3. **CI/CD Ready**: Perfect for automated builds and testing
4. **Isolation**: Clean environment for each build
5. **Cross-Platform**: Works on any platform that supports Docker

## Troubleshooting

### Permission Issues

If you encounter permission issues with mounted volumes:

```bash
# Fix ownership
docker run --rm -v $(pwd):/workspace -w /workspace --user $(id -u):$(id -g) cpm:latest init my-project -y
```

### Build Cache

To clear Docker build cache:

```bash
docker builder prune -a
```

### Debug Container

To debug issues inside the container:

```bash
docker run -it --rm -v $(pwd):/workspace -w /workspace cpm:latest bash
```

## Examples

### Create a New Project

```bash
# Create a new JavaScript project
docker run --rm -v $(pwd):/workspace -w /workspace cpm:latest init my-app -y

# Add Rust to the project
docker run --rm -v $(pwd)/my-app:/workspace -w /workspace cpm:latest add-rust -y
```

### Run Tests

```bash
# Run tests in a project
docker run --rm -v $(pwd):/workspace -w /workspace cpm:latest test
```

### Use npx

```bash
# Create a React app using npx
docker run --rm -v $(pwd):/workspace -w /workspace cpm:latest npx create-react-app my-react-app
```

---

**Docker makes CPM development consistent and reliable across all environments!** 🐳🦀
