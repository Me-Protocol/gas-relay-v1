# Stage 1: Builder
FROM rust:latest as builder

WORKDIR /usr/src/server

COPY server/ ./

RUN mkdir -p src && echo "fn main() { println!(\"Dummy\"); }" > src/main.rs

RUN cargo build --release

RUN rm src/main.rs

RUN cargo build --release --workspace

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/server

# Copy the compiled binary
COPY --from=builder /usr/src/server/target/release/relayer ./relayer

COPY server/.config.toml ./.config.toml

EXPOSE 8080

CMD ["./relayer", "--config-path", "./.config.toml"]
