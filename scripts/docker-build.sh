#!/bin/bash

# CPM Docker Build Script
set -e

echo "🦀 Building CPM with Docker..."

# Build the Docker image
docker build -t cpm:latest .

echo "✅ CPM Docker image built successfully!"
echo ""
echo "Usage examples:"
echo "  docker run --rm cpm --help"
echo "  docker run --rm -v \$(pwd):/workspace cpm init my-project -y"
echo "  docker-compose up cpm-test"
echo "  docker-compose up cpm-clippy"
