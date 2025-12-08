# Multi-stage build for Avila Analytics GA4
# Stage 1: Build frontend (WebAssembly dashboard)
FROM rust:1.75 as frontend-builder

# Install wasm-pack and wasm-bindgen
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /app/frontend

# Copy frontend files
COPY frontend/wasm-dashboard frontend/wasm-dashboard
COPY frontend/tracker frontend/tracker
COPY frontend/static frontend/static

# Build WASM dashboard
WORKDIR /app/frontend/wasm-dashboard
RUN wasm-pack build --target web --out-dir ../static/pkg

# Stage 2: Build Rust backend
FROM rust:1.75 as backend-builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.toml

# Copy source code
COPY src src

# Copy workspace dependencies (if they exist)
COPY ../analytics-engine ../analytics-engine 2>/dev/null || true
COPY ../avila-crypto ../avila-crypto 2>/dev/null || true
COPY ../avila-db ../avila-db 2>/dev/null || true

# Build release binary
RUN cargo build --release --bin avila-analytics

# Stage 3: Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=backend-builder /app/target/release/avila-analytics /app/avila-analytics

# Copy CLI tool
COPY --from=backend-builder /app/target/release/avila-analytics-cli /app/avila-analytics-cli

# Copy frontend assets
COPY --from=frontend-builder /app/frontend/static /app/frontend/static

# Create data directory
RUN mkdir -p /app/data

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run server
CMD ["/app/avila-analytics"]
