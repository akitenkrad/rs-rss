FROM rust:slim-bookworm AS builder
WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

RUN apt update -y && apt upgrade -y && \
    apt install -y build-essential pkg-config poppler-utils libopencv-dev clang libclang-dev libssl-dev curl && \
    rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release

FROM ubuntu:24.04 AS runtime
WORKDIR /app
RUN apt update -y && apt upgrade -y && \
    apt install -y build-essential pkg-config poppler-utils libopencv-dev clang libclang-dev libssl-dev curl && \
    rm -rf /var/lib/apt/lists/*
RUN adduser rss && chown -R rss /app
USER rss
COPY --from=builder /app/target/release/rsrss ./target/release/rsrss

ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT ["/app/target/release/rsrss"]