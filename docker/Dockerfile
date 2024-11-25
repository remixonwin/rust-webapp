# Build stage
FROM rust:1.75-slim-bookworm AS builder

WORKDIR /app

# Install PostgreSQL client library for building
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the entire project
COPY . .

# Build the application with debug symbols
RUN RUSTFLAGS="-g" cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install essential dependencies and debugging tools
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    procps \
    strace \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary and config
COPY --from=builder /app/target/release/rust-webapp /app/
COPY --from=builder /app/config /app/config

# Ensure proper permissions
RUN chmod +x /app/rust-webapp

# Set environment variables
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=full
ENV PORT=3000
ENV HOST=0.0.0.0

# Run as root to ensure we have permission to bind to ports
USER root

# Run the binary with strace to see what's happening
CMD ["strace", "-f", "-e", "trace=network,process", "./rust-webapp"]
