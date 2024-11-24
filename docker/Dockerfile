FROM ubuntu:22.04

# Install minimal required packages
RUN apt-get update && apt-get install -y \
    ca-certificates \
    netcat \
    && rm -rf /var/lib/apt/lists/*

# Create deployment directory
RUN mkdir -p /opt/rust-webapp

# Copy application files
COPY target/release/rust-webapp /opt/rust-webapp/
COPY static /opt/rust-webapp/static/

# Create non-root user
RUN useradd -r -s /bin/false webapp

# Set permissions
RUN chown -R webapp:webapp /opt/rust-webapp && \
    chmod +x /opt/rust-webapp/rust-webapp

# Switch to non-root user
USER webapp
WORKDIR /opt/rust-webapp

# Configure environment
ENV RUST_LOG=debug
ENV PORT=3000
ENV HOST=0.0.0.0

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s \
  CMD nc -z localhost 3000 || exit 1

# Start the application
CMD ["./rust-webapp"]
