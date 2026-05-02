# ---------- Build stage ----------
FROM rust:1.86-bookworm AS builder

WORKDIR /app

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./
COPY compiler/Cargo.toml compiler/Cargo.toml
COPY domain/Cargo.toml  domain/Cargo.toml
COPY miner/Cargo.toml   miner/Cargo.toml
COPY mp3_player/Cargo.toml mp3_player/Cargo.toml

# Create stub sources so `cargo build` can cache dependencies
RUN mkdir -p compiler/src domain/src miner/src mp3_player/src && \
    echo "fn main(){}" > compiler/src/main.rs && \
    echo "fn main(){}" > miner/src/main.rs   && \
    echo "fn main(){}" > mp3_player/src/main.rs && \
    touch domain/src/lib.rs

RUN cargo build --release 2>/dev/null || true

# Remove the stub artifacts so the real source triggers a fresh compile
RUN rm -rf compiler/src domain/src miner/src mp3_player/src \
    target/release/.fingerprint/compiler-* \
    target/release/.fingerprint/domain-* \
    target/release/.fingerprint/miner-* \
    target/release/.fingerprint/mp3_player-*

# Copy real source code and assets
COPY compiler/ compiler/
COPY domain/  domain/
COPY miner/   miner/
COPY mp3_player/ mp3_player/
COPY assets/  assets/

RUN cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy all workspace binaries
COPY --from=builder /app/target/release/compiler   ./
COPY --from=builder /app/target/release/miner      ./
COPY --from=builder /app/target/release/mp3_player  ./
COPY assets/ assets/

# Default entrypoint — change as needed
ENTRYPOINT ["./mp3_player"]
