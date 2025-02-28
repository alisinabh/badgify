FROM node:18-alpine AS ui-builder

WORKDIR /app

COPY /ui/package*.json ./
RUN npm install
COPY /ui .
RUN npm run build

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin badgify

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=ui-builder /app/dist /app/ui/dist
COPY --from=builder /app/target/release/badgify /usr/local/bin

ENTRYPOINT ["/usr/local/bin/badgify"]
