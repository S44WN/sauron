# ── Stage 1: Build ──────────────────────────────────────────────
FROM rust:1.85-bookworm AS builder

# System deps for opencv-rust (uses system libopencv + clang for bindgen)
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    pkg-config \
    libopencv-dev \
    libclang-dev \
    clang \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependency build: copy manifests first, do a dummy build
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Copy real source and rebuild (only our code recompiles)
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ── Stage 2: Runtime ───────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libopencv-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/edge-node /usr/local/bin/edge-node

ENV RUST_LOG=info

ENTRYPOINT ["edge-node"]
