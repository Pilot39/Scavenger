FROM rust:1.78-slim AS dev
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-watch
COPY Cargo.toml Cargo.lock ./
# Pre-fetch deps
RUN mkdir src && echo 'fn main(){}' > src/main.rs && cargo build && rm -rf src
COPY . .
CMD ["cargo", "watch", "-x", "run"]

FROM rust:1.78-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main(){}' > src/main.rs && cargo build --release && rm -rf src
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/scavenger-backend /usr/local/bin/
EXPOSE 8080
CMD ["scavenger-backend"]
