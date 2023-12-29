# syntax=docker/dockerfile:1.2
FROM rust:1.73-buster AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libpq-dev libpcre3-dev && \
    rm -rf /var/lib/apt/lists/*

RUN rustup default 1.73 && \
    rustup target add wasm32-unknown-unknown

WORKDIR app

COPY Cargo.toml Cargo.lock .

# Create a temporary src/lib.rs so that cargo will run (it'll fail in an empty project).
RUN mkdir src && \
    touch src/lib.rs && \
    cargo update && \
    cargo install cargo-leptos@0.2.5 && \
    rm -rf src

COPY . ./

RUN cargo leptos build --release

FROM debian:buster-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        curl libpq-dev libnuma1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/instarst /app/instarst
COPY --from=builder /app/target/site /app/site

EXPOSE 8080
CMD ["/app/instarst"]
