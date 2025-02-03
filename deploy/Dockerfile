# Stage 1: Build
FROM rust:latest as builder

WORKDIR /usr/src/relayer

RUN cd server
COPY Cargo.toml Cargo.lock ./
COPY bin/ bin/
COPY crates/ crates/


# Create a dummy main.rs to build the dependencies
RUN mkdir src && echo "fn main() { println!(\"Dummy\"); }" > src/main.rs

# Build the project to cache dependencies
RUN cargo build --release
RUN rm src/main.rs

# Build the final binary
RUN cargo build --release --workspace

# Stage 2: Run
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/server

COPY --from=builder /usr/src/relayer/target/release/relayer ./relayer

CMD ["./relayer"]

EXPOSE 8080
