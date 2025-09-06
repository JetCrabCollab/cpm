# CPM (Crab Package Manager) Docker Image
FROM rust:1.75-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js and npm
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install wasm-pack for WebAssembly support
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml ./
COPY Cargo.lock* ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Copy the binary to a standard location
RUN cp target/release/cpm /usr/local/bin/cpm

# Make it executable
RUN chmod +x /usr/local/bin/cpm

# Set the entrypoint
ENTRYPOINT ["cpm"]

# Default command
CMD ["--help"]
