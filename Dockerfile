# Multi-stage build for optimal image size
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this is cached as long as Cargo.toml doesn't change)
RUN cargo build --release && rm src/main.rs

# Copy source code
COPY src ./src

# Build the application
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false cmdvault

# Copy the binary from builder stage
COPY --from=builder /usr/src/app/target/release/cmd-vault /usr/local/bin/cmd-vault

# Create directory for user data
RUN mkdir -p /home/cmdvault/.config && chown cmdvault:cmdvault /home/cmdvault/.config

# Switch to non-root user
USER cmdvault
WORKDIR /home/cmdvault

# Set environment variables
ENV RUST_LOG=info
ENV CMD_VAULT_CONFIG=/home/cmdvault/.config/cmd-vault.json

# Expose any ports if needed (not applicable for this CLI app)
# EXPOSE 8080

# Default command
ENTRYPOINT ["cmd-vault"]
CMD ["--help"]