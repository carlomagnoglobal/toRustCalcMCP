# Multi-stage Dockerfile for toRustCalcMCP

# Stage 1: Builder
# Using rust:1.85 which supports Cargo.lock v4 and edition2024
FROM rust:1.85 AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the release binary
RUN cargo build --release --bin toRustCalcMCP

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (minimal)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/toRustCalcMCP /app/toRustCalcMCP

# Set the entrypoint
ENTRYPOINT ["/app/toRustCalcMCP", "--mcp"]

# Labels for metadata
LABEL org.opencontainers.image.title="toRustCalcMCP"
LABEL org.opencontainers.image.description="Rust port of calc - exact-rational calculator as MCP server"
LABEL org.opencontainers.image.authors="Elis Javier Mendez Perez"
LABEL org.opencontainers.image.source="https://github.com/carlomagnoglobal/toRustCalcMCP"
LABEL org.opencontainers.image.version="0.1.0"
