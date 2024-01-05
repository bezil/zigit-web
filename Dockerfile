# Build stage
FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --target wasm32-unknown-unknown --release

# Final run stage
FROM debian:bookworm-slim AS runner

WORKDIR /app
COPY --from=builder /app/target/release/zigit-web /app/zigit-web
CMD ["/app/zigit-web"]
