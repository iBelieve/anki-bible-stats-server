# Stage 1: Planner - Generate dependency recipe
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Builder - Build dependencies and application
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies (cached layer)
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source code and build application
COPY . .
RUN cargo build --release --bin anki-bible-stats

# Stage 3: Runtime - Minimal image with just the binary
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Install runtime dependencies (SQLite is bundled, but we need base libs)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

# Copy binary from builder
COPY --from=builder /app/target/release/anki-bible-stats /usr/local/bin/anki-bible-stats

USER appuser

# Expose API port
EXPOSE 3000

# Run the web server
CMD ["anki-bible-stats"]
