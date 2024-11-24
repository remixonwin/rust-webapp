# Build stage
FROM rust:1.73-slim-bookworm as builder

WORKDIR /app

# Copy only files needed for dependency resolution first
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Now copy the real source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary and config from builder
COPY --from=builder /app/target/release/rust-webapp /app/
COPY --from=builder /app/config /app/config

# Set environment variables
ENV RUST_LOG=info
ENV APP_ENV=production

# Expose port
EXPOSE 8080

# Run the binary
CMD ["./rust-webapp"]
