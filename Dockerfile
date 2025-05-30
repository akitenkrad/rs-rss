FROM rust:slim-bookworm AS builder
WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN adduser rss && chown -R rss /app
USER rss
COPY --from=builder ./app/target/release/app ./target/release/app

ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT ["./target/release/app"]